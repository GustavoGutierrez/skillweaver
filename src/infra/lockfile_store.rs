use std::{fs, path::Path};

use crate::{AppResult, domain::lockfile::SkillsLockfile};

pub fn load_lockfile(project_root: &Path) -> AppResult<SkillsLockfile> {
    let path = project_root.join("skills-lock.json");
    if !path.exists() {
        return Ok(SkillsLockfile {
            version: 1,
            ..Default::default()
        });
    }
    let raw = fs::read_to_string(path)?;
    let lock: SkillsLockfile = serde_json::from_str(&raw)?;
    Ok(lock.ensure_defaults())
}

pub fn save_lockfile(project_root: &Path, lockfile: &SkillsLockfile) -> AppResult<()> {
    let path = project_root.join("skills-lock.json");
    fs::write(path, serde_json::to_string_pretty(lockfile)?)?;
    Ok(())
}
