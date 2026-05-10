use std::{fs, path::PathBuf};

use crate::{AppResult, domain::profile::ProfileStoreData, error::AppError};

pub struct ProfileStore {
    path: PathBuf,
}

impl ProfileStore {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn user_default() -> AppResult<Self> {
        let base = dirs::config_dir().ok_or_else(|| AppError::Validation("missing config dir".into()))?;
        Ok(Self::new(base.join("skillweaver").join("profiles.json")))
    }

    pub fn load(&self) -> AppResult<ProfileStoreData> {
        if !self.path.exists() {
            return Ok(ProfileStoreData {
                schema_version: ProfileStoreData::SCHEMA_VERSION,
                ..Default::default()
            });
        }
        let raw = fs::read_to_string(&self.path)?;
        let data: ProfileStoreData = serde_json::from_str(&raw)?;
        data.validate()?;
        Ok(data)
    }

    pub fn save(&self, data: &ProfileStoreData) -> AppResult<()> {
        data.validate()?;
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(&self.path, serde_json::to_string_pretty(data)?)?;
        Ok(())
    }
}
