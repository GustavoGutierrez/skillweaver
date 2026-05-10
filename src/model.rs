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
    Preview,
    InstallConfirm,
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
    pub status: String,
    pub quit: bool,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            active: Screen::Dashboard,
            modal: None,
            status: "Ready".into(),
            quit: false,
        }
    }
}
