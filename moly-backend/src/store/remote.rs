use std::fs::File;
use std::io::{self, Seek, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

use moly_protocol::data::Model;
use moly_protocol::protocol::FileDownloadResponse;
use std::time::Duration;
use tokio::time::timeout;

use crate::backend_impls::DownloadControlCommand;

async fn get_file_content_length(client: &reqwest::Client, url: &str) -> reqwest::Result<u64> {
    let response = client.head(url).send().await?;

    let content_length = response
        .headers()
        .get(reqwest::header::CONTENT_LENGTH)
        .and_then(|val| val.to_str().ok())
        .and_then(|val| val.parse::<u64>().ok())
        .unwrap_or(0);

    Ok(content_length)
}

pub enum DownloadResult {
    Completed,
    Stopped
}

async fn download_file<P: AsRef<Path>>(
    client: &reqwest::Client,
    content_length: u64,
    url: &str,
    local_path: P,
    step: f64,
    report_fn: &mut (dyn FnMut(f64) -> anyhow::Result<()> + Send),
) -> anyhow::Result<DownloadResult> {
    use futures_util::stream::StreamExt;

    let path: &Path = local_path.as_ref();
    std::fs::create_dir_all(path.parent().unwrap())?;

    let mut file = File::options().write(true).create(true).open(&local_path)?;

    let file_length = file.metadata()?.len();

    if file_length >= content_length {
        Ok(DownloadResult::Completed)
    } else {
        file.seek(io::SeekFrom::End(0))?;

        let range = format!("bytes={}-", file_length);
        let resp = client
            .get(url)
            .header("Range", range)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        let mut downloaded: u64 = file_length;
        let mut last_progress = 0.0;

        let mut stream = resp.bytes_stream();

        loop {
            match timeout(Duration::from_secs(10), stream.next()).await? {
                Some(chunk) => {
                    let chunk = chunk.map_err(|e| anyhow::anyhow!(e))?;
                    let len = chunk.len();
                    file.write_all(&chunk)?;
                    downloaded += len as u64;

                    let progress = (downloaded as f64 / content_length as f64) * 100.0;
                    if progress > last_progress + step {
                        last_progress = progress;
                        // Only report progress up to 99% until fully complete
                        let progress_to_report = progress.min(99.0);
                        match report_fn(progress_to_report) {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                }
                None => {
                    // Download is complete
                    break;
                }
            }
        }

        Ok(DownloadResult::Completed)
    }
}

#[derive(Debug, Clone)]
pub struct ModelFileDownloader {
    client: reqwest::Client,
    sql_conn: Arc<Mutex<rusqlite::Connection>>,
    control_tx: tokio::sync::broadcast::Sender<DownloadControlCommand>,
    country_code: String,
    step: f64,
}

impl ModelFileDownloader {
    const DEFAULT_COUNTRY_CODE: &'static str = "default";
    pub fn new(
        client: reqwest::Client,
        sql_conn: Arc<Mutex<rusqlite::Connection>>,
        control_tx: tokio::sync::broadcast::Sender<DownloadControlCommand>,
        country_code: String,
        step: f64,
    ) -> Self {
        Self {
            client,
            sql_conn,
            control_tx,
            country_code,
            step,
        }
    }

    fn get_download_url(
        &self,
        file: &super::download_files::DownloadedFile,
        remote_file: &super::model_cards::RemoteFile,
    ) -> String {
        remote_file
            .download
            .get(&self.country_code)
            .cloned()
            .unwrap_or_else(|| {
                remote_file
                    .download
                    .get(Self::DEFAULT_COUNTRY_CODE)
                    .cloned()
                    .unwrap_or(format!(
                        "https://huggingface.co/{}/resolve/main/{}",
                        file.model_id, file.name
                    ))
            })
    }

    async fn download(
        self,
        file: super::download_files::DownloadedFile,
        remote_file: super::model_cards::RemoteFile,
        tx: tokio::sync::mpsc::Sender<anyhow::Result<FileDownloadResponse>>,
    ) {
        let file_id = file.id.to_string();

        let mut send_progress = |progress| {
            let r = tx.try_send(Ok(FileDownloadResponse::Progress(
                file_id.clone(),
                progress as f32,
            )));
            log::debug!("send progress {file_id} {progress} {r:?}");
            Ok(())
        };

        let r = self
            .download_file_from_remote(file, remote_file, &mut send_progress)
            .await;

        match r {
            Ok(Some(response)) => {
                let _ = tx.try_send(Ok(response));
            }
            Ok(None) => {
                // TODO Implement file removal when download is stopped, nothing to do when it is paused
            }
            Err(e) => {
                let _ = tx.try_send(Err(e));
            }
        }
    }

    pub async fn run_loop(
        downloader: Self,
        max_downloader: usize,
        mut download_rx: tokio::sync::mpsc::UnboundedReceiver<(
            super::models::Model,
            super::download_files::DownloadedFile,
            super::model_cards::RemoteFile,
            tokio::sync::mpsc::Sender<anyhow::Result<FileDownloadResponse>>,
        )>,
    ) {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_downloader));

        while let Some((model, mut file, remote_file, tx)) = download_rx.recv().await {
            let url = downloader.get_download_url(&file, &remote_file);
            log::info!("Downloading file: {}", url);

            let f = async {
                let content_length = get_file_content_length(&downloader.client, &url)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;

                {
                    file.file_size = content_length;
                    let conn = downloader.sql_conn.lock().unwrap();
                    // insert a pending download
                    file.insert_into_db(&conn).map_err(|e| anyhow::anyhow!(e))?;
                    model.save_to_db(&conn).map_err(|e| anyhow::anyhow!(e))?;
                }

                Ok(())
            };

            let r: anyhow::Result<()> = f.await;

            if let Err(e) = r {
                let _ = tx.send(Err(e));
                continue;
            }

            let downloader_ = downloader.clone();
            let semaphore_ = semaphore.clone();
            tokio::spawn(async move {
                let permit = semaphore_.acquire_owned().await.unwrap();
                downloader_.download(file, remote_file, tx).await;
                drop(permit);
            });
        }
    }

    async fn download_file_from_remote(
        &self,
        mut file: super::download_files::DownloadedFile,
        remote_file: super::model_cards::RemoteFile,
        report_fn: &mut (dyn FnMut(f64) -> anyhow::Result<()> + Send),
    ) -> anyhow::Result<Option<FileDownloadResponse>> {
        let url = self.get_download_url(&file, &remote_file);

        let local_path = Path::new(&file.download_dir)
            .join(&file.model_id)
            .join(&file.name);

        let file_id_ = file.id.as_ref().clone();
        let mut control_rx = self.control_tx.subscribe();

        let listen_control_cmd = async {
            loop {
                let cmd = control_rx.recv().await;
                if let Ok(DownloadControlCommand::Stop(file_id)) = cmd {
                    if file_id == file_id_ {
                        return DownloadResult::Stopped;
                    }
                }
            }
        };

        let r = tokio::select! {
            r = download_file(
                &self.client,
                file.file_size,
                &url,
                &local_path,
                self.step,
                report_fn,
            ) => r?,
            r = listen_control_cmd => {
                r
            }
        };

        match r {
            DownloadResult::Completed => {
                {
                    let conn = self.sql_conn.lock().unwrap();
                    file.mark_downloads();
                    let _ = file.update_downloaded(&conn);
                }

                Ok(Some(FileDownloadResponse::Completed(
                    moly_protocol::data::DownloadedFile {
                        file: moly_protocol::data::File {
                            id: file.id.as_ref().clone(),
                            name: file.name.clone(),
                            size: file.size.clone(),
                            quantization: file.quantization.clone(),
                            downloaded: true,
                            downloaded_path: Some(
                                local_path
                                    .to_str()
                                    .map(|s| s.to_string())
                                    .unwrap_or_default(),
                            ),
                            tags: file.tags,
                            featured: false,
                        },
                        model: Model::default(),
                        downloaded_at: file.downloaded_at,
                        compatibility_guess:
                            moly_protocol::data::CompatibilityGuess::PossiblySupported,
                        information: String::new(),
                    },
                )))
            }
            DownloadResult::Stopped => Ok(None),
        }
    }
}
