use makepad_widgets::*;

live_design! {
    link translator_nl;

    pub SIDEBAR_TAB_DISCOVER = "Ontdekken"
    pub SIDEBAR_TAB_CHAT = "Chatten"
    pub SIDEBAR_TAB_MY_MODELS = "Mijn modellen"
    pub SIDEBAR_TAB_SETTINGS = "Instellingen"
    
    pub MY_MODELS_TITLE = "Mijn modellen"
    pub MY_MODELS_NO_MODELS = "U hebt nog geen modellen gedownload."
    pub MY_MODELS_BUTTON_CHANGE_FOLDER = "Wijzig bestanden"
    pub MY_MODELS_BUTTON_SHOW_FILES = "In bestanden tonen"
    
    pub CHAT_AVATAR_ASSISTANT = "Assistent"
    pub CHAT_AVATAR_USER = "Gebruiker"
    pub CHAT_BUTTON_SAVE = "Opslaan"
    pub CHAT_BUTTON_CANCEL = "Annuleren"
    pub CHAT_BUTTON_SEND = "Verzenden"
    
    pub MODEL_LIST_QUANTIZATION = "Quantisatie"
    pub MODEL_INFO_SIZE = "Modelgrootte"
    pub MODEL_INFO_ARCHITECTURE = "Architectuur"
    pub MODEL_INFO_REQUIRES = "Vereisten"
    
    pub LANDING_SEARCH_TITLE = "Ontdekken, downloaden en lokaal uitvoeren LLM"
    pub LANDING_SEARCH_PLACEHOLDER = "Zoek modellen..."
    
    pub SETTINGS_DOWNLOADS_TITLE = "Downloadlocatie"
    pub SETTINGS_DOWNLOADS_BUTTON = "Wijzig bestanden"
    pub SETTINGS_DOWNLOADS_SHOW = "In bestanden tonen"

    // Settings Screen
    pub SETTINGS_TITLE = "Instellingen"
    pub SETTINGS_SERVER_INFO = "Lokale infoserver"
    pub SETTINGS_NO_MODEL_INFO = "Lokale infoserver wordt weergegeven na het laden van een model."
    pub SETTINGS_PORT_NUMBER = "Poortnummer:"
    pub SETTINGS_PORT_ERROR = "Er is een fout opgetreden bij het laden van het model met deze poortnummer. Probeer een ander poortnummer."
    pub SETTINGS_CODE_EXAMPLE = "Clientcodevoorbeeld"

    // My Models Screen
    pub CHANGE_DOWNLOAD_LOCATION_TEXT = "Wijzig downloadlocatie"
    pub SHOW_IN_FILES_TEXT = "In bestanden tonen"
    pub MY_MODELS_SEARCH_BAR_EMPTY_MESSAGE = "Filter bestanden"
    pub MY_MODELS_TITLE = "Mijn modellen"
    pub READ_FROM_TEXT = "Van"
    pub COPY_TO_CLIPBOARD_TEXT = "Kopiëren naar klembord"
    pub MODEL_CARD_ON_HUGGING_FACE_TEXT = "Modelkaart op Hugging Face"

    // Downloaded Files Table
    pub MODEL_FILE_TEXT = "Modelbestand"
    pub FILE_SIZE_TEXT = "Bestandsgrootte"
    pub ADDED_DATE_TEXT = "Toegevoegd op"
    pub CHAT_WITH_MODEL_TEXT = "Chat met model"

    // Delete Model Modal
    pub DELETE_MODEL_TEXT = "Model verwijderen"
    pub CANCEL_TEXT = "Annuleren"
    pub DELETE_TEXT = "Verwijderen"
    pub DELETE_MODEL_PROMPT_TEXT = "Weet je zeker dat je {} wilt verwijderen?\nDeze actie kan niet worden ongedaan gemaakt."

    // Settings Screen
    pub ADD_A_NEW_SERVER_TEXT = "Voeg een nieuwe server toe"
    pub MOFA_SETTINGS_TITLE = "MoFa instellingen"
    pub MOFA_SERVERS_TITLE = "MoFa servers"

    // Delete Server Modal
    pub CANCEL_SERVER_DELETION_TEXT = "Annuleren"
    pub CONFIRM_DELETE_SERVER_TEXT = "Verwijderen"
    pub DELETE_SERVER_MODAL_TITLE_TEXT = "Server verwijderen"

    // Landing Shared
    pub MODEL_SIZE_TEXT = "Modelgrootte"
    pub REQUIRES_TEXT = "Vereisten"
    pub ARCHITECTURE_TEXT = "Architectuur"

    // Landing Search Bar
    pub SEARCH_BAR_PLACEHOLDER_TEXT = "Zoek modellen..."

    // Landing Search Loading
    pub SEARCHING_TEXT = "Zoeken..."

    // Landing Model Files
    pub SHOW_FILES_TEXT = "Tonen"
    pub FILE_NAME_TEXT = "Bestandsnaam"
    pub FULL_SIZE_TEXT = "Volledige grootte"
    pub QUANTIZATION_TEXT = "Quantisatie"

    // Landing Model Files Item
    pub DOWNLOAD_TEXT = "Downloaden"
    pub CHAT_WITH_MODEL_TEXT = "Chat met model"

    // Landing Downloads
    pub MODEL_DOWNLOADS_TEXT = "Model downloads"
}
