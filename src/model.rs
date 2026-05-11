#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Dashboard,
    Profiles,
    Repositories,
    SystemSettings,
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Modal {
    CreateProfile,
    EditProfile,
    DeleteProfileConfirm,
    AddSource,
    ImportPath,
    ExportPath,
    AddSkill,
    AddRule,
    InstallPath,
}

impl Screen {
    pub fn next(self) -> Self {
        match self {
            Screen::Dashboard => Screen::Profiles,
            Screen::Profiles => Screen::Repositories,
            Screen::Repositories => Screen::SystemSettings,
            Screen::SystemSettings => Screen::Help,
            Screen::Help => Screen::Dashboard,
        }
    }
}

pub struct AppModel {
    pub active: Screen,
    pub modal: Option<Modal>,
    pub input: String,
    pub input_secondary: String,
    pub input_focus_secondary: bool,
    pub store: crate::domain::profile::ProfileStoreData,
    pub selected_profile: usize,
    pub selected_source: usize,
    pub selected_discovery: usize,
    pub discoveries: Vec<RuntimeDiscovery>,
    pub status: String,
    pub modal_error: String,
    pub quit: bool,
}

#[derive(Debug, Clone)]
pub struct RuntimeDiscovery {
    pub source_name: String,
    pub source_root: String,
    pub skill_name: String,
    pub skill_path: String,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            active: Screen::Dashboard,
            modal: None,
            input: String::new(),
            input_secondary: String::new(),
            input_focus_secondary: false,
            store: crate::domain::profile::ProfileStoreData {
                schema_version: crate::domain::profile::ProfileStoreData::SCHEMA_VERSION,
                ..Default::default()
            },
            selected_profile: 0,
            selected_source: 0,
            selected_discovery: 0,
            discoveries: Vec::new(),
            status: "Ready".into(),
            modal_error: String::new(),
            quit: false,
        }
    }
}
