use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::metadata::{SkillFrontmatter, parse_skill_frontmatter};
use crate::domain::source::APPROVED_SKILL_PATHS;
use crate::{AppResult, error::AppError};

#[derive(Debug, Clone)]
pub struct DiscoveredSkill {
    pub path: PathBuf,
    pub metadata: SkillFrontmatter,
}

pub fn discover_skills(root: &Path) -> AppResult<Vec<DiscoveredSkill>> {
    let mut out = Vec::new();
    for rel in APPROVED_SKILL_PATHS {
        let dir = root.join(rel);
        if !dir.is_dir() {
            continue;
        }
        let canon_root = match root.canonicalize() {
            Ok(c) => c,
            Err(_) => root.to_path_buf(),
        };
        for ent in fs::read_dir(&dir)? {
            let ent = ent?;
            if !ent.file_type()?.is_dir() {
                continue;
            }
            let skill_dir = ent.path();
            let canon = match skill_dir.canonicalize() {
                Ok(c) => c,
                Err(_) => skill_dir.clone(),
            };
            if !canon.starts_with(&canon_root) {
                return Err(AppError::Blocked("source path escaped registered root".into()));
            }
            let skill_md = skill_dir.join("SKILL.md");
            if skill_md.exists() {
                let raw = fs::read_to_string(&skill_md)?;
                let metadata = parse_skill_frontmatter(&raw)?;
                out.push(DiscoveredSkill {
                    path: skill_dir,
                    metadata,
                });
            }
        }
    }
    Ok(out)
}
