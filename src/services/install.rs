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

    InstallPlan {
        can_install: blocked_by.is_empty(),
        blocked_by,
        skills: vec![],
        rules_markdown: render_rules(&profile.rules),
    }
}

fn download_skill(url: &str, skill_name: &str, dest_dir: &Path) -> AppResult<()> {
    eprintln!("  ⬇ downloading {skill_name}...");
    let response = ureq::get(url).call().map_err(|e| {
        AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("download failed for {skill_name}: {e}")))
    })?;

    let mut body = Vec::new();
    response.into_body().as_reader().read_to_end(&mut body)?;

    eprintln!("  📦 extracting {skill_name}...");
    let tmp_dir = tempfile::tempdir().map_err(|e| {
        AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("tempdir failed for {skill_name}: {e}")))
    })?;
    let tmp_zip = tmp_dir.path().join("skill.zip");
    std::fs::write(&tmp_zip, &body)?;
    crate::infra::archive::extract_zip_safely(&tmp_zip, tmp_dir.path())
        .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("extract failed for {skill_name}: {e}"))))?;

    // ZIPs have skills/<name>/... — copy just <name>/ contents to dest
    let skill_inner = tmp_dir.path().join("skills").join(skill_name);
    if dest_dir.exists() {
        std::fs::remove_dir_all(dest_dir)
            .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("remove_dir failed for {skill_name}: {e}"))))?;
    }
    std::fs::create_dir_all(dest_dir)
        .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("create_dir failed for {skill_name}: {e}"))))?;

    if skill_inner.is_dir() {
        for entry in std::fs::read_dir(&skill_inner).map_err(|e| {
            AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("read_dir failed for {skill_name}: {e}")))
        })? {
            let entry = entry?;
            let target = dest_dir.join(entry.file_name());
            if entry.file_type()?.is_dir() {
                copy_dir(&entry.path(), &target)?;
            } else {
                std::fs::copy(entry.path(), &target)
                    .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("copy failed for {skill_name}: {e}"))))?;
            }
        }
    } else {
        for entry in std::fs::read_dir(tmp_dir.path())? {
            let entry = entry?;
            let name = entry.file_name();
            if name == "skill.zip" { continue; }
            let target = dest_dir.join(&name);
            if entry.file_type()?.is_dir() {
                copy_dir(&entry.path(), &target)?;
            } else {
                std::fs::copy(entry.path(), &target)?;
            }
        }
    }
    eprintln!("  ✅ {skill_name} installed");
    Ok(())
}

fn copy_dir(src: &Path, dst: &Path) -> AppResult<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let target = dst.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

pub fn execute_install(profile: &Profile, project_root: &Path, sources: &[SourceRegistration]) -> AppResult<String> {
    let plan = build_preview(profile, project_root);
    if !plan.can_install {
        return Err(AppError::Blocked("preview reported blocking findings".into()));
    }

    let skills_dir = project_root.join(".agents/skills");
    if skills_dir.is_file() {
        std::fs::remove_file(&skills_dir)?;
    }
    std::fs::create_dir_all(&skills_dir)?;

    let total = profile.skills.len();
    let mut downloaded = 0;
    let mut skipped = 0;
    for (i, skill) in profile.skills.iter().enumerate() {
        let dest = skills_dir.join(skill);
        if dest.exists() {
            skipped += 1;
            eprintln!("[{}/{}] ⏭ {skill} (already installed)", i + 1, total);
            continue;
        }
        eprintln!("[{}/{}] {skill}", i + 1, total);
        let mut found = false;
        for src in sources {
            let url = format!("{}/{skill}.zip", src.root.trim_end_matches('/'));
            match download_skill(&url, skill, &dest) {
                Ok(()) => {
                    downloaded += 1;
                    found = true;
                    break;
                }
                Err(e) => {
                    eprintln!("  ⚠ source '{}' failed: {e}", src.name);
                }
            }
        }
        if !found {
            eprintln!("  ❌ could not download '{skill}' from any source");
        }
    }

    eprintln!("\n📝 writing AGENTS.md...");
    let managed = resolve_managed_file(project_root);
    apply_managed_rules(&managed, &plan.rules_markdown)
        .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("AGENTS.md write failed: {e}"))))?;

    eprintln!("🔒 updating skills-lock.json...");
    let mut lock = load_lockfile(project_root)?;
    for s in &profile.skills {
        let skill_path = format!(".agents/skills/{s}/SKILL.md");
        lock.skills.entry(s.clone()).or_insert(SkillLockEntry {
            source: "skillweaver".into(),
            source_type: "zip".into(),
            skill_path,
            computed_hash: "pending".into(),
        });
    }
    save_lockfile(project_root, &lock)
        .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("lockfile write failed: {e}"))))?;

    let dst = project_root.join(".claude/skills");
    if skills_dir.exists() && !dst.exists() {
        eprintln!("🔗 linking .claude/skills...");
        if let Some(parent) = dst.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!(".claude dir create failed: {e}"))))?;
        }
        symlink_or_copy(&skills_dir, &dst)
            .map_err(|e| AppError::Io(std::io::Error::new(std::io::ErrorKind::Other, format!("symlink failed: {e}"))))?;
    }

    let msg = if skipped > 0 {
        format!("{downloaded} skill(s) installed, {skipped} already present")
    } else {
        format!("{downloaded} skill(s) installed")
    };
    Ok(msg)
}
