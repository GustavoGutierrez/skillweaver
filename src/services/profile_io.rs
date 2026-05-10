use std::fs;
use std::path::Path;

use crate::domain::profile::ProfileStoreData;
use crate::{AppResult, error::AppError};

pub fn export_profiles(path: &Path, data: &ProfileStoreData) -> AppResult<()> {
    fs::write(path, serde_json::to_string_pretty(data)?)?;
    Ok(())
}

pub fn import_profiles(path: &Path) -> AppResult<ProfileStoreData> {
    let raw = fs::read_to_string(path)?;
    let data: ProfileStoreData = serde_json::from_str(&raw)?;
    if data.schema_version != ProfileStoreData::SCHEMA_VERSION {
        return Err(AppError::Validation("incompatible import schema version".into()));
    }
    data.validate()?;
    Ok(data)
}
