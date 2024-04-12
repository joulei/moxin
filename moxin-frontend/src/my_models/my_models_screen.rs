use makepad_widgets::*;
use moxin_protocol::data::DownloadedFile;

use crate::{data::store::Store, shared::utils::BYTES_PER_MB};

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;
    import makepad_draw::shader::std::*;

    import crate::shared::styles::*;

    import crate::my_models::downloaded_files_table::DownloadedFilesTable;

    ICON_EDIT_FOLDER = dep("crate://self/resources/icons/edit_folder.svg")
    ICON_SEARCH = dep("crate://self/resources/icons/search.svg")

    DownloadLocation = <RoundedView> {
        width: Fit,
        height: Fit,
        padding: {top: 6, bottom: 6, left: 4, right: 14}

        spacing: 8,

        draw_bg: {
            instance radius: 2.0,
            color: #FEFEFE,
        }

        <Icon> {
            draw_icon: {
                svg_file: (ICON_EDIT_FOLDER),
                fn get_color(self) -> vec4 {
                    return #000;
                }
            }
            icon_walk: {width: 14, height: 14}
        }

        <Label> {
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 11},
                color: #000
            }
            text: "Change Download Location"
        }
    }

    ReviewInFinder = <RoundedView> {
        width: Fit,
        height: Fit,
        margin: {left: 10}
        padding: {top: 6, bottom: 6, left: 4, right: 10}
        spacing: 8,

        draw_bg: {
            instance radius: 2.0,
            color: #FEFEFE,
        }

        <Icon> {
            draw_icon: {
                svg_file: (ICON_EDIT_FOLDER),
                fn get_color(self) -> vec4 {
                    return #000;
                }
            }
            icon_walk: {width: 14, height: 14}
        }

        <Label> {
            draw_text:{
                text_style: <REGULAR_FONT>{font_size: 11},
                color: #000
            }
            text: "Review in Finder"
        }
    }

    SearchBar = <RoundedView> {
        width: Fit,
        height: Fit,

        show_bg: true,
        draw_bg: {
            color: #fff
        }

        padding: {top: 3, bottom: 3, left: 20, right: 20}

        spacing: 4,
        align: {x: 0.0, y: 0.5},

        draw_bg: {
            radius: 10.0,
            border_color: #D0D5DD,
            border_width: 1.0,
        }

        <Icon> {
            draw_icon: {
                svg_file: (ICON_SEARCH),
                fn get_color(self) -> vec4 {
                    return #666;
                }
            }
            icon_walk: {width: 14, height: 14}
        }

        input = <TextInput> {
            width: 250,
            height: Fit,

            empty_message: "Search Model by Keyword"
            draw_bg: {
                color: #fff
            }
            draw_text: {
                text_style:<REGULAR_FONT>{font_size: 10},
                fn get_color(self) -> vec4 {
                    return #555
                }
            }

            // TODO find a way to override colors
            draw_cursor: {
                instance focus: 0.0
                uniform border_radius: 0.5
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        0.,
                        0.,
                        self.rect_size.x,
                        self.rect_size.y,
                        self.border_radius
                    )
                    sdf.fill(mix(#fff, #bbb, self.focus));
                    return sdf.result
                }
            }

            // TODO find a way to override colors
            draw_select: {
                instance hover: 0.0
                instance focus: 0.0
                uniform border_radius: 2.0
                fn pixel(self) -> vec4 {
                    let sdf = Sdf2d::viewport(self.pos * self.rect_size);
                    sdf.box(
                        0.,
                        0.,
                        self.rect_size.x,
                        self.rect_size.y,
                        self.border_radius
                    )
                    sdf.fill(mix(#eee, #ddd, self.focus)); // Pad color
                    return sdf.result
                }
            }
        }
    }

    MyModelsScreen = {{MyModelsScreen}} {
        width: Fill
        height: Fill
        padding: 60
        spacing: 20
        flow: Down

        show_bg: true
        draw_bg: {
            color: #cccccc33,
            instance color2: #AF56DA55
            fn get_color(self) -> vec4 {
                let coef = self.rect_size.y / self.rect_size.x;

                let distance_vec = self.pos - vec2(0.8, 0.8);
                let norm_distance = length(vec2(distance_vec.x, distance_vec.y * coef) * 1.8);

                if pow(norm_distance, 1.4) > 1.0 {
                    return self.color;
                } else {
                    return mix(self.color2, self.color, pow(norm_distance, 1.4));
                }
            }

            fn pixel(self) -> vec4 {
                return Pal::premul(self.get_color());
            }
        }

        header = <View> {
            width: Fill, height: Fit
            spacing: 15
            flow: Right
            align: {x: 0.0, y: 1.0}

            title = <Label> {
                draw_text:{
                    text_style: <BOLD_FONT>{font_size: 30}
                    color: #000
                }
                text: "My Models"
            }

            models_summary = <Label> {
                draw_text:{
                    text_style: <REGULAR_FONT>{font_size: 20}
                    color: #555
                }
            }
        }

        sub_header = <View> {
            width: Fill, height: Fit
            flow: Right
            spacing: 10
            align: {x: 0.0, y: 0.5}

            <Label> {
                width: Fit, height: Fit
                draw_text:{
                    text_style: <REGULAR_FONT>{font_size: 11}
                    color: #000
                }
                text: "Local Models Folder"
            }
            local_models_folder = <Label> {
                width: 300, height: Fit
                draw_text:{
                    text_style: <REGULAR_FONT>{font_size: 11}
                    color: #222
                    wrap: Ellipsis
                }
            }

            <DownloadLocation> {}
            <ReviewInFinder> {}
            <View> { width: Fill }
            <SearchBar> {}
        }

        table = <DownloadedFilesTable> {
            margin: {top: 20}
        }
    }
}

#[derive(Widget, LiveHook, Live)]
pub struct MyModelsScreen {
    #[deref]
    view: View,
}

impl Widget for MyModelsScreen {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let store = &scope.data.get::<Store>().unwrap();

        let summary = generate_models_summary(&&store.downloaded_files);
        let models_summary_label = self.view.label(id!(header.models_summary));
        models_summary_label.set_text(&summary);

        let models_folder_label = self.view.label(id!(sub_header.local_models_folder));
        models_folder_label.set_text(&store.downloaded_files_folder);

        self.view.draw_walk(cx, scope, walk)
    }
}

fn generate_models_summary(downloaded_files: &Vec<DownloadedFile>) -> String {
    let total_diskspace_mb = total_files_disk_space(downloaded_files);
    let disk_space_label = if total_diskspace_mb >= 1024.0 {
        format!("{:.2} GB Diskspace", total_diskspace_mb / 1024.0)
    } else {
        format!("{} MB Diskspace", total_diskspace_mb as i32)
    };

    let model_label = if downloaded_files.len() == 1 {
        "Model"
    } else {
        "Models"
    };

    format!(
        "{} {}, {}",
        downloaded_files.len(),
        model_label,
        disk_space_label
    )
}

fn total_files_disk_space(files: &Vec<DownloadedFile>) -> f64 {
    files.iter().fold(0., |acc, file| {
        let file_size_bytes = file.file.size.parse::<f64>();
        match file_size_bytes {
            Ok(size_bytes) => acc + (size_bytes / BYTES_PER_MB),
            Err(_) => acc,
        }
    })
}
