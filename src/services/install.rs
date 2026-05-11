use std::io::Read;
use std::path::Path;

use crate::domain::install::{BlockReason, InstallPlan};
use crate::domain::lockfile::SkillLockEntry;
use crate::domain::profile::Profile;
use crate::domain::source::SourceRegistration;
use crate::domain::target::resolve_managed_file;
use crate::infra::archive::symlink_or_copy;
use crate::infra::lockfile_store::{load_lockfile, save_lockfile};
use crate::infra::markdown::{apply_managed_rules, render_rules};
use crate::{AppResult, error::AppError};

pub fn build_preview(profile: &Profile, _project_root: &Path) -> InstallPlan {
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
    blocked_by.sort_by_key(|b| match b {
        BlockReason::Traversal => 0,
        BlockReason::InvalidMetadata => 1,
        BlockReason::UnsupportedStructure => 2,
        BlockReason::Conflict => 3,
    });
    blocked_by.dedup();
    InstallPlan { can_install: blocked_by.is_empty(), blocked_by, skills: vec![], rules_markdown: render_rules(&profile.rules) }
}

fn download_skill(url: &str, skill_name: &str, dest_dir: &Path) -> AppResult<()> {
    let response = ureq::get(url).call().map_err(|e|
        AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("dl:{skill_name}:{e}")))
    )?;
    let mut body = Vec::new();
    response.into_body().as_reader().read_to_end(&mut body)?;
    let tmp_dir = tempfile::tempdir().map_err(|e|
        AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("tmp:{skill_name}:{e}")))
    )?;
    let tmp_zip = tmp_dir.path().join("skill.zip");
    std::fs::write(&tmp_zip, &body)?;
    crate::infra::archive::extract_zip_safely(&tmp_zip, tmp_dir.path())
        .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("extract:{skill_name}:{e}"))))?;
    let skill_inner = tmp_dir.path().join("skills").join(skill_name);
    if dest_dir.exists() {
        std::fs::remove_dir_all(dest_dir).map_err(|e|
            AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("rm:{skill_name}:{e}")))
        )?;
    }
    std::fs::create_dir_all(dest_dir).map_err(|e|
        AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("mkdir:{skill_name}:{e}")))
    )?;
    if skill_inner.is_dir() {
        for entry in std::fs::read_dir(&skill_inner).map_err(|e|
            AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("rd:{skill_name}:{e}")))
        )? {
            let entry = entry?;
            let target = dest_dir.join(entry.file_name());
            if entry.file_type()?.is_dir() { copy_dir(&entry.path(), &target)?; }
            else { std::fs::copy(entry.path(), &target).map_err(|e|
                AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("cp:{skill_name}:{e}")))
            )?; }
        }
    } else {
        for entry in std::fs::read_dir(tmp_dir.path())? {
            let entry = entry?;
            let name = entry.file_name();
            if name == "skill.zip" { continue; }
            let target = dest_dir.join(&name);
            if entry.file_type()?.is_dir() { copy_dir(&entry.path(), &target)?; }
            else { std::fs::copy(entry.path(), &target)?; }
        }
    }
    Ok(())
}

fn err(ctx: &str, e: impl std::fmt::Display) -> AppError {
    AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("{ctx}: {e}")))
}

fn copy_dir(src: &Path, dst: &Path) -> AppResult<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let target = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() { copy_dir(&entry.path(), &target)?; }
        else { std::fs::copy(entry.path(), &target)?; }
    }
    Ok(())
}

pub fn execute_install(profile: &Profile, project_root: &Path, sources: &[SourceRegistration]) -> AppResult<String> {
    let plan = build_preview(profile, project_root);
    if !plan.can_install { return Err(AppError::Blocked("preview blocked".into())); }

    let skills_dir = project_root.join(".agents/skills");
    if skills_dir.is_file() {
        std::fs::remove_file(&skills_dir)
            .map_err(|e| err("rm_skills_file", e))?;
    }
    std::fs::create_dir_all(&skills_dir)
        .map_err(|e| err("mkdir_skills", e))?;

    let mut downloaded = 0;
    let mut skipped = 0;
    for skill in &profile.skills {
        let dest = skills_dir.join(skill);
        if dest.exists() { skipped += 1; continue; }
        let mut found = false;
        let effective_sources: Vec<SourceRegistration> = if let Some(ref url) = profile.source_url {
            vec![SourceRegistration { name: profile.name.clone(), root: url.clone(), source_type: crate::domain::source::SourceType::Zip }]
        } else {
            sources.to_vec()
        };
        for src in &effective_sources {
            let url = format!("{}/{skill}.zip", src.root.trim_end_matches('/'));
            if download_skill(&url, skill, &dest).is_ok() { downloaded += 1; found = true; break; }
        }
        if !found {
            return Err(AppError::Io(std::io::Error::new(std::io::ErrorKind::Other,
                format!("no_source for '{skill}' ({} sources available)", sources.len()))));
        }
    }

    let managed = resolve_managed_file(project_root);
    apply_managed_rules(&managed, &plan.rules_markdown)
        .map_err(|e| err("agents_md", e))?;

    let mut lock = load_lockfile(project_root)?;
    for s in &profile.skills {
        lock.skills.entry(s.clone()).or_insert(SkillLockEntry {
            source: "skillweaver".into(), source_type: "zip".into(),
            skill_path: format!(".agents/skills/{s}/SKILL.md"), computed_hash: "pending".into(),
        });
    }
    save_lockfile(project_root, &lock)
        .map_err(|e| err("lockfile", e))?;

    let has_claude = project_root.join("CLAUDE.md").exists() || project_root.join(".claude").is_dir();
    if has_claude {
        let claude_skills = project_root.join(".claude/skills");
        std::fs::create_dir_all(&claude_skills)
            .map_err(|e| err("mkdir_claude", e))?;
        for skill in &profile.skills {
            let src = skills_dir.join(skill);
            let dst = claude_skills.join(skill);
            if src.exists() && !dst.exists() {
                let abs_src = src.canonicalize().unwrap_or_else(|_| src.clone());
                symlink_or_copy(&abs_src, &dst)
                    .map_err(|e| err("symlink_{skill}", e))?;
            }
        }
    }

    Ok(if skipped > 0 { format!("{downloaded} new, {skipped} ok") } else { format!("{downloaded} installed") })
}
