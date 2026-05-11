use std::path::Path;

use color_eyre::Result;
use color_eyre::eyre::bail;

use skillweaver::domain::profile::ProfileStoreData;
use skillweaver::infra::profile_store::ProfileStore;
use skillweaver::services::install::execute_install;
use skillweaver::services::profile_io::import_profiles;

pub fn run(args: &[String]) -> Result<()> {
    match args.get(1).map(|s| s.as_str()) {
        Some("install") => cmd_install(args),
        Some("list") => cmd_list(),
        Some("import") => cmd_import(args),
        Some(cmd) => {
            eprintln!("Unknown command: {cmd}");
            eprintln!("Usage: skillweaver [install <name> --target <path> | list | import <file>]");
            Ok(())
        }
        None => Ok(()),
    }
}

fn resolve_store() -> (ProfileStore, ProfileStoreData) {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| Path::new(".").to_path_buf())
        .join("skillweaver");
    let store = ProfileStore::new(config_dir.join("profiles.json"));
    let data = store.load().unwrap_or_else(|_| ProfileStoreData {
        schema_version: ProfileStoreData::SCHEMA_VERSION,
        ..Default::default()
    });
    (store, data)
}

fn cmd_install(args: &[String]) -> Result<()> {
    let mut name: Option<String> = None;
    let mut target: Option<String> = None;
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--target" | "-t" => {
                i += 1;
                target = args.get(i).map(|s| s.clone());
            }
            other => {
                if !other.starts_with('-') && name.is_none() {
                    name = Some(other.to_string());
                }
            }
        }
        i += 1;
    }

    let name = name.ok_or_else(|| {
        color_eyre::eyre::eyre!("Profile name required. Usage: skillweaver install <name> --target <path>")
    })?;
    let target = target.unwrap_or_else(|| ".".to_string());

    let (_store, data) = resolve_store();
    let profile = data.profiles.iter().find(|p| p.name == name || p.id == name)
        .ok_or_else(|| color_eyre::eyre::eyre!("Profile not found: {name}. Use 'skillweaver list' to see available profiles."))?;

    let project_root = Path::new(&target);
    if !project_root.exists() {
        bail!("Target path does not exist: {target}");
    }

    eprintln!("Installing profile '{}' ({} skills)...", profile.name, profile.skills.len());
    let mode = execute_install(profile, project_root, &data.sources)
        .map_err(|e| color_eyre::eyre::eyre!("Install failed: {e}"))?;
    eprintln!("\nDone: {mode}");
    println!("Installed profile '{}' into '{}' via {mode}", profile.name, target);
    println!("Skills: {}", profile.skills.join(", "));
    if !profile.rules.is_empty() {
        println!("Rules applied to managed block in AGENTS.md/CLAUDE.md");
    }
    Ok(())
}

fn cmd_list() -> Result<()> {
    let (_store, data) = resolve_store();
    if data.profiles.is_empty() {
        println!("No profiles found. Create one in the TUI or import with 'skillweaver import <file>'.");
        return Ok(());
    }
    println!("Profiles:");
    for p in &data.profiles {
        let default = if data.default_profile_id.as_deref() == Some(p.id.as_str()) { " (default)" } else { "" };
        println!("  - {}{} — {} skill(s), {} rule(s)", p.name, default, p.skills.len(), p.rules.len());
    }
    Ok(())
}

fn cmd_import(args: &[String]) -> Result<()> {
    let file = args.get(2).cloned().unwrap_or_else(|| "skillweaver-profiles.json".to_string());
    let path = Path::new(&file);
    if !path.exists() {
        bail!("File not found: {file}");
    }
    let imported = import_profiles(path)?;
    let (_store, mut data) = resolve_store();
    let before = data.profiles.len();
    for p in imported.profiles {
        if !data.profiles.iter().any(|e| e.id == p.id) {
            data.profiles.push(p);
        }
    }
    for s in imported.sources {
        if !data.sources.iter().any(|e| e.name == s.name) {
            data.sources.push(s);
        }
    }
    let store = ProfileStore::new(dirs::config_dir().unwrap_or_else(|| Path::new(".").to_path_buf()).join("skillweaver").join("profiles.json"));
    store.save(&data)?;
    println!("Imported {} profile(s) from {file}", data.profiles.len() - before);
    Ok(())
}
