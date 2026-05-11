use serde::{Deserialize, Serialize};

use crate::{AppResult, error::AppError};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub skills: Vec<String>,
    pub rules: Vec<String>,
    pub install_mode: InstallMode,
    #[serde(default)]
    pub source_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InstallMode {
    Auto,
    Symlink,
    Copy,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProfileStoreData {
    pub schema_version: u32,
    pub default_profile_id: Option<String>,
    pub profiles: Vec<Profile>,
    pub sources: Vec<crate::domain::source::SourceRegistration>,
}

impl Profile {
    pub fn validate(&self) -> AppResult<()> {
        if self.id.trim().is_empty() || self.name.trim().is_empty() {
            return Err(AppError::Validation(
                "profile id/name must not be empty".into(),
            ));
        }
        Ok(())
    }
}

impl ProfileStoreData {
    pub const SCHEMA_VERSION: u32 = 1;

    pub fn validate(&self) -> AppResult<()> {
        if self.schema_version != Self::SCHEMA_VERSION {
            return Err(AppError::Validation(
                "unsupported profile schema version".into(),
            ));
        }
        for p in &self.profiles {
            p.validate()?;
        }
        Ok(())
    }
}
