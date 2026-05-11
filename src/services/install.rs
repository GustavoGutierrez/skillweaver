use std::path::Path;

use crate::domain::install::{BlockReason, InstallPlan};
use crate::domain::lockfile::SkillLockEntry;
use crate::domain::profile::Profile;
use crate::domain::target::resolve_managed_file;
use crate::infra::archive::symlink_or_copy;
use crate::infra::lockfile_store::{load_lockfile, save_lockfile};
use crate::infra::markdown::{apply_managed_rules, render_rules};
use crate::{AppResult, error::AppError};

pub fn build_preview(profile: &Profile, project_root: &Path) -> InstallPlan {
    let mut blocked_by = Vec::new();

    if profile.name.trim().is_empty() {
        blocked_by.push(BlockReason::Conflict);
    }

    for skill in &profile.skills {
        if skill.contains("..") || skill.contains('/') || skill.contains('\\') {
            blocked_by.push(BlockReason::Traversal);
            break;
        }
    }

    let target_root = project_root.join(".agents/skills");
    for skill in &profile.skills {
        let target = target_root.join(skill);
        if target.exists() {
            blocked_by.push(BlockReason::Conflict);
            break;
        }
    }

    blocked_by.sort_by_key(|b| match b {
        BlockReason::Traversal => 0,
        BlockReason::InvalidMetadata => 1,
        BlockReason::UnsupportedStructure => 2,
        BlockReason::Conflict => 3,
    });
    blocked_by.dedup();

    InstallPlan {
        can_install: blocked_by.is_empty(),
        blocked_by,
        skills: vec![],
        rules_markdown: render_rules(&profile.rules),
    }
}

pub fn execute_install(profile: &Profile, project_root: &Path) -> AppResult<String> {
    let plan = build_preview(profile, project_root);
    if !plan.can_install {
        return Err(AppError::Blocked("preview reported blocking findings".into()));
    }
    let managed = resolve_managed_file(project_root);
    apply_managed_rules(&managed, &plan.rules_markdown)?;

    let mut lock = load_lockfile(project_root)?;
    for s in &profile.skills {
        lock.skills.insert(
            s.clone(),
            SkillLockEntry {
                source: "manual".into(),
                source_type: "local".into(),
                skill_path: format!(".agents/skills/{s}/SKILL.md"),
                computed_hash: "pending".into(),
            },
        );
    }
    save_lockfile(project_root, &lock)?;

    let src = project_root.join(".agents/skills");
    let dst = project_root.join(".claude/skills");
    if src.exists() && !dst.exists() {
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mode = symlink_or_copy(&src, &dst)?;
        return Ok(mode.into());
    }
    Ok("copy".into())
}
