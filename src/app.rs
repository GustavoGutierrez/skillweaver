use std::path::Path;

use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

use crate::AppResult;
use crate::domain::profile::{InstallMode, Profile, ProfileStoreData};
use crate::domain::source::{SourceRegistration, SourceType};
use crate::infra::profile_store::ProfileStore;
use crate::model::{AppModel, Modal, RuntimeDiscovery, Screen};
use crate::services::discovery::discover_skills;
use crate::services::profile_io::{export_profiles, import_profiles};

fn default_store() -> ProfileStoreData {
    ProfileStoreData {
        schema_version: ProfileStoreData::SCHEMA_VERSION,
        ..Default::default()
    }
}

fn normalize_selection(model: &mut AppModel) {
    if model.store.profiles.is_empty() {
        model.selected_profile = 0;
    } else {
        model.selected_profile = model.selected_profile.min(model.store.profiles.len() - 1);
    }
    if model.store.sources.is_empty() {
        model.selected_source = 0;
    } else {
        model.selected_source = model.selected_source.min(model.store.sources.len() - 1);
    }
    if model.discoveries.is_empty() {
        model.selected_discovery = 0;
    } else {
        model.selected_discovery = model.selected_discovery.min(model.discoveries.len() - 1);
    }
}

fn refresh_discoveries(model: &mut AppModel) {
    model.discoveries.clear();
    for src in &model.store.sources {
        let root = Path::new(&src.root);
        let found = match discover_skills(root) {
            Ok(f) => f,
            Err(e) => {
                model.status = format!("Discovery error for '{}': {e}", src.name);
                continue;
            }
        };
        for item in found {
            let skill_name = item
                .path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| item.metadata.name.clone());
            model.discoveries.push(RuntimeDiscovery {
                source_name: src.name.clone(),
                source_root: src.root.clone(),
                skill_name,
                skill_path: item.path.display().to_string(),
            });
        }
    }
    normalize_selection(model);
}

fn save_model(model: &mut AppModel, store: &ProfileStore) {
    if let Err(e) = store.save(&model.store) {
        model.status = format!("Save failed: {e}");
    }
}

fn merge_imported_data(existing: &mut ProfileStoreData, imported: ProfileStoreData) {
    for profile in imported.profiles {
        if !existing.profiles.iter().any(|p| p.id == profile.id) {
            existing.profiles.push(profile);
        }
    }
    for source in imported.sources {
        if !existing.sources.iter().any(|s| s.name == source.name) {
            existing.sources.push(source);
        }
    }
    if existing.default_profile_id.is_none() {
        existing.default_profile_id = imported.default_profile_id;
    }
}

fn selected_profile_mut(model: &mut AppModel) -> Option<&mut Profile> {
    model.store.profiles.get_mut(model.selected_profile)
}

fn selected_profile(model: &AppModel) -> Option<&Profile> {
    model.store.profiles.get(model.selected_profile)
}

fn add_selected_discovery_to_active_profile(model: &mut AppModel) {
    let Some(discovery) = model.discoveries.get(model.selected_discovery).cloned() else {
        model.status = "No discovery selected".into();
        return;
    };
    let Some(profile) = selected_profile_mut(model) else {
        model.status = "No active profile".into();
        return;
    };
    if profile.skills.iter().any(|s| s == &discovery.skill_name) {
        model.status = "Skill already in profile".into();
        return;
    }
    profile.skills.push(discovery.skill_name.clone());
    model.status = format!("Added '{}' to profile", discovery.skill_name);
}

pub fn update(model: &mut AppModel, key: KeyCode, store: &ProfileStore) {
    if model.modal.is_some() {
        match key {
            KeyCode::Char('q') => model.quit = true,
            KeyCode::Esc => {
                model.modal = None;
                model.modal_error.clear();
                model.input.clear();
                model.input_secondary.clear();
                model.input_focus_secondary = false;
                model.status = "Modal closed".into();
            }
            KeyCode::Tab if model.modal == Some(Modal::AddSource) => {
                model.input_focus_secondary = !model.input_focus_secondary;
            }
            KeyCode::Backspace => {
                if model.modal == Some(Modal::AddSource) && model.input_focus_secondary {
                    model.input_secondary.pop();
                } else {
                    model.input.pop();
                }
            }
            KeyCode::Enter => {
                match model.modal {
                    Some(Modal::CreateProfile) => {
                        let name = model.input.trim();
                        model.modal_error.clear();
                        if name.is_empty() {
                            model.modal_error = "Profile name is required".into();
                            return;
                        }
                        let id = format!("profile-{}", model.store.profiles.len() + 1);
                        model.store.profiles.push(Profile {
                            id,
                            name: name.to_string(),
                            description: None,
                            skills: Vec::new(),
                            rules: Vec::new(),
                            install_mode: InstallMode::Auto,
                        });
                        model.selected_profile = model.store.profiles.len() - 1;
                        model.store.default_profile_id = model.store.profiles.get(model.selected_profile).map(|p| p.id.clone());
                        model.modal = None;
                        model.input.clear();
                        save_model(model, store);
                        model.status = "Profile created".into();
                    }
                    Some(Modal::EditProfile) => {
                        let name = model.input.trim().to_string();
                        model.modal_error.clear();
                        if name.is_empty() {
                            model.modal_error = "Profile name is required".into();
                            return;
                        }
                        if let Some(profile) = selected_profile_mut(model) {
                            profile.name = name;
                            model.modal = None;
                            model.input.clear();
                            save_model(model, store);
                            model.status = "Profile renamed".into();
                        }
                    }
                    Some(Modal::DeleteProfileConfirm) => {
                        if model.store.profiles.is_empty() {
                            model.modal = None;
                            return;
                        }
                        model.store.profiles.remove(model.selected_profile);
                        normalize_selection(model);
                        model.store.default_profile_id = selected_profile(model).map(|p| p.id.clone());
                        model.modal = None;
                        save_model(model, store);
                        model.status = "Profile deleted".into();
                    }
                    Some(Modal::AddSource) => {
                        let name = model.input.trim();
                        let root = model.input_secondary.trim();
                        model.modal_error.clear();
                        if name.is_empty() || root.is_empty() {
                            model.modal_error = "Source name and path are required".into();
                            return;
                        }
                        if !Path::new(root).exists() {
                            model.modal_error = format!("Path does not exist: {root}\nOnly local directory paths are supported, not URLs.");
                            return;
                        }
                        model.store.sources.push(SourceRegistration {
                            name: name.to_string(),
                            root: root.to_string(),
                            source_type: SourceType::Local,
                        });
                        model.selected_source = model.store.sources.len() - 1;
                        model.modal = None;
                        model.input.clear();
                        model.input_secondary.clear();
                        model.input_focus_secondary = false;
                        refresh_discoveries(model);
                        save_model(model, store);
                        model.status = "Source registered".into();
                    }
                    Some(Modal::ImportPath) => {
                        let path_str = model.input.trim().to_string();
                        if path_str.is_empty() {
                            model.modal_error = "File path is required".into();
                            return;
                        }
                        let path = Path::new(&path_str);
                        match import_profiles(path) {
                            Ok(data) => {
                                let before = model.store.profiles.len();
                                merge_imported_data(&mut model.store, data);
                                let added = model.store.profiles.len() - before;
                                model.modal = None;
                                model.input.clear();
                                normalize_selection(model);
                                refresh_discoveries(model);
                                save_model(model, store);
                                model.status = format!("Imported {} profile(s) from {}", added, path.display());
                            }
                            Err(err) => {
                                model.modal_error = format!("Import failed: {err}");
                            }
                        }
                    }
                    Some(Modal::ExportPath) => {
                        let path_str = model.input.trim().to_string();
                        if path_str.is_empty() {
                            model.modal_error = "File path is required".into();
                            return;
                        }
                        let path = Path::new(&path_str);
                        match export_profiles(path, &model.store) {
                            Ok(()) => {
                                model.modal = None;
                                model.input.clear();
                                model.status = format!("Exported to {}", path.display());
                            }
                            Err(err) => {
                                model.modal_error = format!("Export failed: {err}");
                            }
                        }
                    }
                    Some(Modal::AddSkill) => {
                        let name = model.input.trim().to_string();
                        if name.is_empty() {
                            model.modal_error = "Skill name is required".into();
                            return;
                        }
                        if let Some(profile) = selected_profile_mut(model) {
                            if profile.skills.contains(&name) {
                                model.modal_error = "Skill already in profile".into();
                                return;
                            }
                            profile.skills.push(name);
                            model.modal = None;
                            model.input.clear();
                            save_model(model, store);
                            model.status = "Skill added to profile".into();
                        }
                    }
                    Some(Modal::AddRule) => {
                        let rule = model.input.trim().to_string();
                        if rule.is_empty() {
                            model.modal_error = "Rule is required".into();
                            return;
                        }
                        if let Some(profile) = selected_profile_mut(model) {
                            if profile.rules.contains(&rule) {
                                model.modal_error = "Rule already in profile".into();
                                return;
                            }
                            profile.rules.push(rule);
                            model.modal = None;
                            model.input.clear();
                            save_model(model, store);
                            model.status = "Rule added to profile".into();
                        }
                    }
                    Some(Modal::InstallPath) => {
                        let mut path_str = model.input.trim().to_string();
                        if path_str.is_empty() {
                            path_str = ".".to_string();
                        }
                        let project_root = Path::new(&path_str);
                        if !project_root.exists() {
                            model.modal_error = format!("Path does not exist: {path_str}");
                            return;
                        }
                        let profile = match selected_profile(model) {
                            Some(p) => p.clone(),
                            None => {
                                model.modal_error = "No profile selected".into();
                                return;
                            }
                        };
                        model.modal = None;
                        model.modal_error.clear();
                        model.input.clear();
                        model.status = "⏳ Installing...".into();
                        model.install_target = Some((profile, path_str));
                    }
                    None => {}
                }
            }
            KeyCode::Char(' ') => {
                if model.modal == Some(Modal::AddSource) && model.input_focus_secondary {
                    model.input_secondary.push(' ');
                } else {
                    model.input.push(' ');
                }
            }
            KeyCode::Char(ch) => {
                if model.modal == Some(Modal::AddSource) && model.input_focus_secondary {
                    model.input_secondary.push(ch);
                } else {
                    model.input.push(ch);
                }
            }
            _ => {}
        }
        return;
    }

    match key {
        KeyCode::Char('q') => model.quit = true,
        KeyCode::Tab => model.active = model.active.next(),
        KeyCode::Char('1') => model.active = Screen::Dashboard,
        KeyCode::Char('2') => model.active = Screen::Profiles,
        KeyCode::Char('3') => model.active = Screen::Repositories,
        KeyCode::Char('4') => model.active = Screen::SystemSettings,
        KeyCode::Char('5') | KeyCode::Char('h') => model.active = Screen::Help,
        _ => match model.active {
            Screen::Dashboard => match key {
                KeyCode::Down | KeyCode::Char('j') => {
                    if !model.store.profiles.is_empty() {
                        model.selected_profile = (model.selected_profile + 1).min(model.store.profiles.len() - 1);
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    model.selected_profile = model.selected_profile.saturating_sub(1);
                }
                KeyCode::Enter | KeyCode::Char(' ') => {
                    model.store.default_profile_id = selected_profile(model).map(|p| p.id.clone());
                    save_model(model, store);
                    model.status = "Default profile selected".into();
                }
                KeyCode::Char('i') => {
                    if selected_profile(model).is_some() {
                        model.modal = Some(Modal::InstallPath);
                        model.input = ".".to_string();
                        model.modal_error.clear();
                        model.status = "Press Enter to install here, or type a path".into();
                    }
                }
                _ => {}
            },
            Screen::Profiles => match key {
                KeyCode::Down | KeyCode::Char('j') => {
                    if !model.store.profiles.is_empty() {
                        model.selected_profile = (model.selected_profile + 1).min(model.store.profiles.len() - 1);
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    model.selected_profile = model.selected_profile.saturating_sub(1);
                }
                KeyCode::Char('c') => {
                    model.modal = Some(Modal::CreateProfile);
                    model.input.clear();
                    model.status = "Create profile".into();
                }
                KeyCode::Char('e') => {
                    if let Some(profile_name) = selected_profile(model).map(|p| p.name.clone()) {
                        model.modal = Some(Modal::EditProfile);
                        model.input = profile_name;
                        model.status = "Edit profile".into();
                    }
                }
                KeyCode::Char('x') => {
                    if selected_profile(model).is_some() {
                        model.modal = Some(Modal::DeleteProfileConfirm);
                    }
                }
                KeyCode::Char('d') => {
                    if let Some(profile) = selected_profile(model).cloned() {
                        let mut duplicated = profile;
                        duplicated.id = format!("profile-{}", model.store.profiles.len() + 1);
                        duplicated.name = format!("{} copy", duplicated.name);
                        model.store.profiles.push(duplicated);
                        model.selected_profile = model.store.profiles.len() - 1;
                        save_model(model, store);
                        model.status = "Profile duplicated".into();
                    }
                }
                KeyCode::Enter => {
                    model.store.default_profile_id = selected_profile(model).map(|p| p.id.clone());
                    save_model(model, store);
                    model.status = "Default profile selected".into();
                }
                KeyCode::Char(' ') => {
                    model.store.default_profile_id = selected_profile(model).map(|p| p.id.clone());
                    save_model(model, store);
                    model.status = "Default profile selected".into();
                }
                KeyCode::Char('i') => {
                    model.modal = Some(Modal::ImportPath);
                    model.input.clear();
                    model.modal_error.clear();
                    model.status = "Enter import file path".into();
                }
                KeyCode::Char('s') => {
                    if selected_profile(model).is_some() {
                        model.modal = Some(Modal::AddSkill);
                        model.input.clear();
                        model.modal_error.clear();
                        model.status = "Add skill to profile".into();
                    }
                }
                KeyCode::Char('r') => {
                    if selected_profile(model).is_some() {
                        model.modal = Some(Modal::AddRule);
                        model.input.clear();
                        model.modal_error.clear();
                        model.status = "Add rule to profile".into();
                    }
                }
                KeyCode::Char('o') => {
                    model.modal = Some(Modal::ExportPath);
                    model.input.clear();
                    model.modal_error.clear();
                    model.status = "Enter export file path".into();
                }
                _ => {}
            },
            Screen::Repositories => match key {
                KeyCode::Down | KeyCode::Char('j') => {
                    if !model.discoveries.is_empty() {
                        model.selected_discovery = (model.selected_discovery + 1).min(model.discoveries.len() - 1);
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    model.selected_discovery = model.selected_discovery.saturating_sub(1);
                }
                KeyCode::Char('a') => {
                    add_selected_discovery_to_active_profile(model);
                    save_model(model, store);
                }
                KeyCode::Char(' ') => {
                    add_selected_discovery_to_active_profile(model);
                    save_model(model, store);
                }
                KeyCode::Char('n') => {
                    model.modal = Some(Modal::AddSource);
                    model.input.clear();
                    model.input_secondary.clear();
                    model.input_focus_secondary = false;
                }
                KeyCode::Char('r') => {
                    refresh_discoveries(model);
                    model.status = format!("Scan complete: {} skills", model.discoveries.len());
                }
                _ => {}
            },
            _ => {}
        },
    }
    normalize_selection(model);
}

pub fn run(mut terminal: DefaultTerminal) -> AppResult<()> {
    let store = ProfileStore::user_default()?;
    let mut model = AppModel::default();
    model.store = store.load().unwrap_or_else(|_| default_store());
    refresh_discoveries(&mut model);
    normalize_selection(&mut model);

    while !model.quit {
        terminal.draw(|frame| crate::ui::render(frame, &model))?;
        if let Event::Key(k) = event::read()? {
            update(&mut model, k.code, &store);
        }
        if let Some((profile, target)) = model.install_target.take() {
            terminal.draw(|frame| crate::ui::render(frame, &model))?;
            let project_root = Path::new(&target);
            match crate::services::install::execute_install(&profile, project_root, &model.store.sources) {
                Ok(mode) => {
                    model.status = format!("Installed: {mode}");
                }
                Err(err) => {
                    model.status = format!("Install failed: {err}");
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;

    use crate::app::update;
    use crate::infra::profile_store::ProfileStore;
    use crate::model::{AppModel, Modal, Screen};

    #[test]
    fn create_profile_modal_creates_and_selects_profile() {
        let dir = tempfile::tempdir().unwrap();
        let store = ProfileStore::new(dir.path().join("profiles.json"));
        let mut model = AppModel::default();

        model.active = Screen::Profiles;
        update(&mut model, KeyCode::Char('c'), &store);
        assert_eq!(model.modal, Some(Modal::CreateProfile));
        update(&mut model, KeyCode::Char('A'), &store);
        update(&mut model, KeyCode::Char('p'), &store);
        update(&mut model, KeyCode::Enter, &store);

        assert!(model.modal.is_none());
        assert_eq!(model.store.profiles.len(), 1);
        assert_eq!(model.store.profiles[0].name, "Ap");
        assert!(model.store.default_profile_id.is_some());
    }

    #[test]
    fn modal_blocks_navigation_until_escape() {
        let dir = tempfile::tempdir().unwrap();
        let store = ProfileStore::new(dir.path().join("profiles.json"));
        let mut model = AppModel::default();

        update(&mut model, KeyCode::Char('2'), &store);
        update(&mut model, KeyCode::Char('c'), &store);
        let before = model.active;
        update(&mut model, KeyCode::Tab, &store);
        assert_eq!(model.active, before);

        update(&mut model, KeyCode::Esc, &store);
        update(&mut model, KeyCode::Tab, &store);
        assert_ne!(model.active, before);
    }

    #[test]
    fn add_source_flow_validates_and_registers_local_source() {
        let dir = tempfile::tempdir().unwrap();
        let sources_root = dir.path().join("sources");
        std::fs::create_dir_all(&sources_root).unwrap();
        let store = ProfileStore::new(dir.path().join("profiles.json"));
        let mut model = AppModel::default();
        model.active = Screen::Repositories;

        update(&mut model, KeyCode::Char('n'), &store);
        for ch in "local".chars() {
            update(&mut model, KeyCode::Char(ch), &store);
        }
        update(&mut model, KeyCode::Tab, &store);
        for ch in sources_root.display().to_string().chars() {
            update(&mut model, KeyCode::Char(ch), &store);
        }
        update(&mut model, KeyCode::Enter, &store);

        assert_eq!(model.store.sources.len(), 1);
        assert_eq!(model.store.sources[0].name, "local");
    }

    #[test]
    fn edit_profile_modal_updates_name() {
        let dir = tempfile::tempdir().unwrap();
        let store = ProfileStore::new(dir.path().join("profiles.json"));
        let mut model = AppModel::default();

        model.active = Screen::Profiles;
        update(&mut model, KeyCode::Char('c'), &store);
        for ch in "Old".chars() {
            update(&mut model, KeyCode::Char(ch), &store);
        }
        update(&mut model, KeyCode::Enter, &store);

        update(&mut model, KeyCode::Char('e'), &store);
        assert_eq!(model.modal, Some(Modal::EditProfile));
        update(&mut model, KeyCode::Backspace, &store);
        update(&mut model, KeyCode::Backspace, &store);
        update(&mut model, KeyCode::Backspace, &store);
        for ch in "New".chars() {
            update(&mut model, KeyCode::Char(ch), &store);
        }
        update(&mut model, KeyCode::Enter, &store);

        assert_eq!(model.store.profiles[0].name, "New");
    }

    #[test]
    fn export_then_import_profiles_from_runtime_flow() {
        let dir = tempfile::tempdir().unwrap();
        let cwd = std::env::current_dir().unwrap();
        std::env::set_current_dir(dir.path()).unwrap();

        let store = ProfileStore::new(dir.path().join("profiles.json"));
        let mut model = AppModel::default();
        model.active = Screen::Profiles;

        // Create profile
        update(&mut model, KeyCode::Char('c'), &store);
        for ch in "Alpha".chars() {
            update(&mut model, KeyCode::Char(ch), &store);
        }
        update(&mut model, KeyCode::Enter, &store);

        // Export via modal
        update(&mut model, KeyCode::Char('o'), &store);
        for ch in "exported.json".chars() {
            update(&mut model, KeyCode::Char(ch), &store);
        }
        update(&mut model, KeyCode::Enter, &store);

        // Clear state
        model.store.profiles.clear();
        model.store.default_profile_id = None;

        // Import via modal
        update(&mut model, KeyCode::Char('i'), &store);
        for ch in "exported.json".chars() {
            update(&mut model, KeyCode::Char(ch), &store);
        }
        update(&mut model, KeyCode::Enter, &store);

        assert_eq!(model.store.profiles.len(), 1);
        assert_eq!(model.store.profiles[0].name, "Alpha");

        std::env::set_current_dir(cwd).unwrap();
    }

    #[test]
    fn space_key_matches_contract_for_selection_actions() {
        use crate::domain::source::{SourceRegistration, SourceType};
        use crate::app::refresh_discoveries;
        let dir = tempfile::tempdir().unwrap();
        let sources_root = dir.path().join("sources");
        std::fs::create_dir_all(sources_root.join("skills/space-skill")).unwrap();
        std::fs::write(
            sources_root.join("skills/space-skill/SKILL.md"),
            "---\nname: space-skill\ndescription: demo\ntriggers:\n  - x\n---\n# body",
        )
        .unwrap();

        let store = ProfileStore::new(dir.path().join("profiles.json"));
        let mut model = AppModel::default();

        // Create a profile
        model.active = Screen::Profiles;
        update(&mut model, KeyCode::Char('c'), &store);
        for ch in "Main".chars() {
            update(&mut model, KeyCode::Char(ch), &store);
        }
        update(&mut model, KeyCode::Enter, &store);
        update(&mut model, KeyCode::Char(' '), &store);
        assert_eq!(model.store.default_profile_id.as_deref(), Some("profile-1"));

        // Register source directly in model, then scan
        model.store.sources.push(SourceRegistration {
            name: "local".into(),
            root: sources_root.display().to_string(),
            source_type: SourceType::Local,
        });
        refresh_discoveries(&mut model);
        assert!(!model.discoveries.is_empty(), "discoveries should not be empty after scan");

        // Add discovery to profile via Space key
        model.active = Screen::Repositories;
        update(&mut model, KeyCode::Char(' '), &store);

        assert_eq!(model.store.profiles[0].skills, vec!["space-skill"]);
    }
}
