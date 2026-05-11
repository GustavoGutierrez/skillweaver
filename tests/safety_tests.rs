use std::fs;

use skillweaver::domain::metadata::parse_skill_frontmatter;
use skillweaver::domain::profile::{InstallMode, Profile};
use skillweaver::domain::target::resolve_managed_file;
use skillweaver::infra::archive::{extract_zip_safely, symlink_or_copy};
use skillweaver::infra::lockfile_store::{load_lockfile, save_lockfile};
use skillweaver::infra::markdown::{BEGIN, END, apply_managed_rules};
use skillweaver::infra::profile_store::ProfileStore;
use skillweaver::services::discovery::discover_skills;
use skillweaver::services::install::{build_preview, execute_install};
use skillweaver::services::profile_io::{export_profiles, import_profiles};

fn sample_profile(skills: Vec<&str>) -> Profile {
    Profile {
        id: "1".into(),
        name: "default".into(),
        description: None,
        skills: skills.into_iter().map(|s| s.to_string()).collect(),
        rules: vec!["rule".into()],
        install_mode: InstallMode::Auto,
    }
}

#[test]
fn preserves_markdown_outside_managed_block() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("AGENTS.md");
    fs::write(
        &path,
        format!("before\n{BEGIN}\nold\n{END}\nafter\n"),
    )
    .unwrap();
    apply_managed_rules(&path, "new").unwrap();
    let out = fs::read_to_string(path).unwrap();
    assert!(out.contains("before"));
    assert!(out.contains("after"));
    assert!(out.contains("new"));
    assert!(!out.contains("old\n"));
}

#[test]
fn lockfile_remains_skills_only() {
    let dir = tempfile::tempdir().unwrap();
    let mut lock = load_lockfile(dir.path()).unwrap();
    lock.skills.insert(
        "a".into(),
        skillweaver::domain::lockfile::SkillLockEntry {
            source: "s".into(),
            source_type: "local".into(),
            skill_path: "skills/a/SKILL.md".into(),
            computed_hash: "h".into(),
        },
    );
    save_lockfile(dir.path(), &lock).unwrap();
    let raw = fs::read_to_string(dir.path().join("skills-lock.json")).unwrap();
    assert!(raw.contains("\"skills\""));
    assert!(!raw.contains("rules"));
}

#[test]
fn parses_frontmatter() {
    let md = "---\nname: test\ndescription: hello\ntriggers:\n  - a\n---\n# body";
    let m = parse_skill_frontmatter(md).unwrap();
    assert_eq!(m.name, "test");
}

#[test]
fn resolves_target_file_precedence() {
    let dir = tempfile::tempdir().unwrap();
    let path = resolve_managed_file(dir.path());
    assert!(path.ends_with("AGENTS.md"));
    fs::write(dir.path().join("CLAUDE.md"), "x").unwrap();
    let path = resolve_managed_file(dir.path());
    assert!(path.ends_with("CLAUDE.md"));
}

#[test]
fn profile_json_persistence_roundtrip() {
    let dir = tempfile::tempdir().unwrap();
    let store = ProfileStore::new(dir.path().join("profiles.json"));
    let data = skillweaver::domain::profile::ProfileStoreData {
        schema_version: 1,
        profiles: vec![skillweaver::domain::profile::Profile {
            id: "1".into(),
            name: "default".into(),
            description: None,
            skills: vec![],
            rules: vec![],
            install_mode: skillweaver::domain::profile::InstallMode::Auto,
        }],
        ..Default::default()
    };
    store.save(&data).unwrap();
    let loaded = store.load().unwrap();
    assert_eq!(loaded.profiles.len(), 1);
}

#[test]
fn blocks_zip_path_traversal() {
    let dir = tempfile::tempdir().unwrap();
    let zip_path = dir.path().join("bad.zip");
    {
        let file = fs::File::create(&zip_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        zip.start_file::<_, ()>("../evil.txt", zip::write::SimpleFileOptions::default())
            .unwrap();
        use std::io::Write;
        zip.write_all(b"x").unwrap();
        zip.finish().unwrap();
    }
    let out = extract_zip_safely(&zip_path, &dir.path().join("out"));
    assert!(out.is_err());
}

#[test]
fn symlink_or_copy_returns_mode() {
    let dir = tempfile::tempdir().unwrap();
    let src = dir.path().join("src");
    let dst = dir.path().join("dst");
    fs::create_dir_all(&src).unwrap();
    fs::write(src.join("a.txt"), "x").unwrap();
    let mode = symlink_or_copy(&src, &dst).unwrap();
    assert!(mode == "symlink" || mode == "copy");
}

#[test]
fn preview_blocks_traversal_skill_names() {
    let dir = tempfile::tempdir().unwrap();
    let profile = sample_profile(vec!["../escape"]);
    let plan = build_preview(&profile, dir.path());
    assert!(!plan.can_install);
    assert!(plan
        .blocked_by
        .contains(&skillweaver::domain::install::BlockReason::Traversal));
}

#[test]
fn preview_blocks_target_conflict_when_install_target_exists() {
    let dir = tempfile::tempdir().unwrap();
    let skill_dir = dir.path().join(".agents/skills/existing");
    fs::create_dir_all(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: existing\ndescription: ok\n---\n# body",
    )
    .unwrap();

    let profile = sample_profile(vec!["existing"]);
    let plan = build_preview(&profile, dir.path());
    assert!(!plan.can_install);
    assert!(plan
        .blocked_by
        .contains(&skillweaver::domain::install::BlockReason::Conflict));
}

#[test]
fn execute_install_refuses_when_preview_has_blockers() {
    let dir = tempfile::tempdir().unwrap();
    let profile = sample_profile(vec!["../escape"]);
    let out = execute_install(&profile, dir.path());
    assert!(out.is_err());
}

#[test]
fn profile_store_rejects_invalid_json() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("profiles.json");
    fs::write(&path, "{not-valid-json").unwrap();
    let store = ProfileStore::new(path);
    assert!(store.load().is_err());
}

#[test]
fn import_rejects_incompatible_schema_version() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bundle.json");
    fs::write(&path, r#"{"schema_version":99,"profiles":[],"sources":[]}"#).unwrap();
    assert!(import_profiles(&path).is_err());
}

#[test]
fn export_import_roundtrip_restores_profiles_without_install() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("bundle.json");
    let data = skillweaver::domain::profile::ProfileStoreData {
        schema_version: 1,
        profiles: vec![sample_profile(vec![])],
        ..Default::default()
    };
    export_profiles(&path, &data).unwrap();
    let imported = import_profiles(&path).unwrap();
    assert_eq!(imported.profiles.len(), 1);
    assert_eq!(imported.profiles[0].name, "default");
}

#[cfg(unix)]
#[test]
fn discovery_rejects_symlink_escape_outside_root() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().unwrap();
    let outside = tempfile::tempdir().unwrap();
    let outside_skill = outside.path().join("outside-skill");
    fs::create_dir_all(&outside_skill).unwrap();
    fs::write(
        outside_skill.join("SKILL.md"),
        "---\nname: out\ndescription: out\n---\n# body",
    )
    .unwrap();

    symlink(outside.path(), dir.path().join("skills")).unwrap();

    let out = discover_skills(dir.path());
    assert!(out.is_err());
}
