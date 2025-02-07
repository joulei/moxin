use makepad_widgets::*;

live_design! {
    link translator_en;
    
    pub SIDEBAR_TAB_DISCOVER = "Discover"
    pub SIDEBAR_TAB_CHAT = "Chat"
    pub SIDEBAR_TAB_MY_MODELS = "My Models"
    pub SIDEBAR_TAB_SETTINGS = "Settings"

    pub MY_MODELS_TITLE = "My Models"
    pub MY_MODELS_NO_MODELS = "You haven't downloaded any models yet."
    pub MY_MODELS_BUTTON_CHANGE_FOLDER = "Change Folder"
    pub MY_MODELS_BUTTON_SHOW_FILES = "Show in Files"

    pub CHAT_AVATAR_ASSISTANT = "P"
    pub CHAT_AVATAR_USER = "U"
    pub CHAT_BUTTON_SAVE = "Save"
    pub CHAT_BUTTON_CANCEL = "Cancel"
    pub CHAT_BUTTON_SEND = "Send"

    pub MODEL_LIST_QUANTIZATION = "Quantization"
    pub MODEL_INFO_SIZE = "Model Size"
    pub MODEL_INFO_ARCHITECTURE = "Architecture"
    pub MODEL_INFO_REQUIRES = "Requires"

    pub LANDING_SEARCH_TITLE = "Discover, download, and run local LLMs"
    pub LANDING_SEARCH_PLACEHOLDER = "Search models..."

    pub SETTINGS_DOWNLOADS_TITLE = "Download Location"
    pub SETTINGS_DOWNLOADS_BUTTON = "Change Folder"
    pub SETTINGS_DOWNLOADS_SHOW = "Show in Files"

    // Settings Screen
    pub SETTINGS_TITLE = "Settings"
    pub SETTINGS_SERVER_INFO = "Local inference server information"
    pub SETTINGS_NO_MODEL_INFO = "Local inference options will appear once you have a model loaded."
    pub SETTINGS_PORT_NUMBER = "Port number:"
    pub SETTINGS_PORT_ERROR = "Something went wrong while loading the model using this port number. Please try another one."
    pub SETTINGS_CODE_EXAMPLE = "Client code example"

    // My Models Screen
    pub CHANGE_DOWNLOAD_LOCATION_TEXT = "Change Download Location"
    pub SHOW_IN_FILES_TEXT = "Show in Folder"
    pub MY_MODELS_SEARCH_BAR_EMPTY_MESSAGE = "Filter files"
    pub MY_MODELS_TITLE = "My Models"
    pub READ_FROM_TEXT = "Read from"
    pub COPY_TO_CLIPBOARD_TEXT = "Copy to Clipboard"
    pub MODEL_CARD_ON_HUGGING_FACE_TEXT = "Model Card on Hugging Face"

    // Downloaded Files Table
    pub MODEL_FILE_TEXT = "Model File"
    pub FILE_SIZE_TEXT = "File Size"
    pub ADDED_DATE_TEXT = "Added Date"
    pub CHAT_WITH_MODEL_TEXT = "Chat with Model"

    // Delete Model Modal
    pub DELETE_MODEL_TEXT = "Delete Model"
    pub CANCEL_TEXT = "Cancel"
    pub DELETE_TEXT = "Delete"
    pub DELETE_MODEL_PROMPT_TEXT = "Are you sure you want to delete {}?\nThis action cannot be undone."

    // Settings Screen
    pub ADD_A_NEW_SERVER_TEXT = "Add a new server"
    pub MOFA_SETTINGS_TITLE = "MoFa Settings"
    pub MOFA_SERVERS_TITLE = "MoFa Servers"

    // Delete Server Modal
    pub CANCEL_SERVER_DELETION_TEXT = "Cancel"
    pub CONFIRM_DELETE_SERVER_TEXT = "Delete"
    pub DELETE_SERVER_MODAL_TITLE_TEXT = "Delete Server"
    
    // Landing Shared
    pub MODEL_SIZE_TEXT = "Model Size"
    pub REQUIRES_TEXT = "Requires"
    pub ARCHITECTURE_TEXT = "Architecture"

    // Landing Search Loading
    pub SEARCHING_TEXT = "Searching..."

    // Landing Model Files
    pub SHOW_FILES_TEXT = "SHOW"
    pub FILE_NAME_TEXT = "File name"
    pub FULL_SIZE_TEXT = "Full Size"
    pub QUANTIZATION_TEXT = "Quantization"

    // Landing Model Files Item
    pub DOWNLOAD_TEXT = "Download"
    pub CHAT_WITH_MODEL_TEXT = "Chat with Model"

    // Landing Downloads
    pub MODEL_DOWNLOADS_TEXT = "Model Downloads"
}
