use makepad_widgets::*;

live_design! {
    link translator_zh;

    pub SIDEBAR_TAB_DISCOVER = "发现"
    pub SIDEBAR_TAB_CHAT = "聊天"
    pub SIDEBAR_TAB_MY_MODELS = "我的模型"
    pub SIDEBAR_TAB_SETTINGS = "设置"
    
    pub MY_MODELS_TITLE = "我的模型"
    pub MY_MODELS_NO_MODELS = "您还没有下载任何模型。"
    pub MY_MODELS_BUTTON_CHANGE_FOLDER = "更改文件夹"
    pub MY_MODELS_BUTTON_SHOW_FILES = "在文件中显示"
    
    pub CHAT_AVATAR_ASSISTANT = "助"
    pub CHAT_AVATAR_USER = "用"
    pub CHAT_BUTTON_SAVE = "保存"
    pub CHAT_BUTTON_CANCEL = "取消"
    pub CHAT_BUTTON_SEND = "发送"
    
    pub MODEL_LIST_QUANTIZATION = "量化"
    pub MODEL_INFO_SIZE = "模型大小"
    pub MODEL_INFO_ARCHITECTURE = "架构"
    pub MODEL_INFO_REQUIRES = "要求"
    
    pub LANDING_SEARCH_TITLE = "发现、下载和运行本地LLM"
    pub LANDING_SEARCH_PLACEHOLDER = "搜索模型..."
    
    pub SETTINGS_DOWNLOADS_TITLE = "下载位置"
    pub SETTINGS_DOWNLOADS_BUTTON = "更改文件夹"
    pub SETTINGS_DOWNLOADS_SHOW = "在文件中显示"

    // Settings Screen
    pub SETTINGS_TITLE = "设置"
    pub SETTINGS_SERVER_INFO = "本地推理服务器信息"
    pub SETTINGS_NO_MODEL_INFO = "加载模型后将显示本地推理选项。"
    pub SETTINGS_PORT_NUMBER = "端口号："
    pub SETTINGS_PORT_ERROR = "使用此端口号加载模型时出现问题。请尝试另一个。"
    pub SETTINGS_CODE_EXAMPLE = "客户端代码示例"

    // My Models Screen
    pub CHANGE_DOWNLOAD_LOCATION_TEXT = "更改下载位置"
    pub SHOW_IN_FILES_TEXT = "在文件中显示"
    pub MY_MODELS_SEARCH_BAR_EMPTY_MESSAGE = "过滤文件"
    pub MY_MODELS_TITLE = "我的模型"
    pub READ_FROM_TEXT = "从"
    pub COPY_TO_CLIPBOARD_TEXT = "复制到剪贴板"
    pub MODEL_CARD_ON_HUGGING_FACE_TEXT = "Hugging Face上的模型卡片"

    // Downloaded Files Table
    pub MODEL_FILE_TEXT = "模型文件"
    pub FILE_SIZE_TEXT = "文件大小"
    pub ADDED_DATE_TEXT = "添加日期"
    pub CHAT_WITH_MODEL_TEXT = "与模型聊天"

    // Delete Model Modal
    pub DELETE_MODEL_TEXT = "删除模型"
    pub CANCEL_TEXT = "取消"
    pub DELETE_TEXT = "删除"
    pub DELETE_MODEL_PROMPT_TEXT = "您确定要删除 {0} 吗？此操作无法撤销。"

    // Settings Screen
    pub ADD_A_NEW_SERVER_TEXT = "添加一个新服务器"
    pub MOFA_SETTINGS_TITLE = "MoFa设置"
    pub MOFA_SERVERS_TITLE = "MoFa服务器"

    // Delete Server Modal
    pub CANCEL_SERVER_DELETION_TEXT = "取消"
    pub CONFIRM_DELETE_SERVER_TEXT = "删除"
    pub DELETE_SERVER_MODAL_TITLE_TEXT = "删除服务器"

    // Landing Shared
    pub MODEL_SIZE_TEXT = "模型大小"
    pub REQUIRES_TEXT = "要求"
    pub ARCHITECTURE_TEXT = "架构"

    // Landing Search Bar
    pub SEARCH_BAR_PLACEHOLDER_TEXT = "搜索模型..."

    // Landing Search Loading
    pub SEARCHING_TEXT = "搜索中..."

    // Landing Model Files
    pub SHOW_FILES_TEXT = "显示"
    pub FILE_NAME_TEXT = "文件名"
    pub FULL_SIZE_TEXT = "完整大小"
    pub QUANTIZATION_TEXT = "量化"

    // Landing Model Files Item
    pub DOWNLOAD_TEXT = "下载"
    pub CHAT_WITH_MODEL_TEXT = "与模型聊天"

    // Landing Downloads
    pub MODEL_DOWNLOADS_TEXT = "模型下载"
}
