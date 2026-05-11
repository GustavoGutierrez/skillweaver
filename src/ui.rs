use ratatui::layout::{Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::model::{AppModel, Modal, Screen};

fn render_dashboard(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let [left, right] = Layout::horizontal([Constraint::Percentage(35), Constraint::Fill(1)]).areas(area);

    let profile_items: Vec<ListItem> = if model.store.profiles.is_empty() {
        vec![ListItem::new("No profiles yet. Go to [2] Profiles to create one.".dim())]
    } else {
        model
            .store
            .profiles
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                let marker = if idx == model.selected_profile { ">" } else { " " };
                ListItem::new(format!("{marker} {}", p.name))
            })
            .collect()
    };
    frame.render_widget(
        List::new(profile_items)
            .block(Block::default().title("Profiles").borders(Borders::ALL)),
        left,
    );

    let detail = if let Some(profile) = model.store.profiles.get(model.selected_profile) {
        let default_mark = if model.store.default_profile_id.as_deref() == Some(profile.id.as_str()) {
            "yes"
        } else {
            "no"
        };
        format!(
            "View your active profile and its installed skills.\n\nActive: {}\nDefault: {}\n\nSkills:\n{}\n\nRules:\n{}",
            profile.name,
            default_mark,
            if profile.skills.is_empty() { "(none — go to [3] Repositories to add skills)".to_string() } else { profile.skills.join("\n") },
            if profile.rules.is_empty() { "(none)".to_string() } else { profile.rules.join("\n") },
        )
    } else {
        "Welcome! Create a profile in [2] Profiles, then add skills from [3] Repositories.".to_string()
    };
    frame.render_widget(
        Paragraph::new(detail)
            .block(Block::default().title("Dashboard — overview of your profile").borders(Borders::ALL)),
        right,
    );
}

fn render_profiles(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let [left, right] = Layout::horizontal([Constraint::Percentage(45), Constraint::Fill(1)]).areas(area);
    let items: Vec<ListItem> = if model.store.profiles.is_empty() {
        vec![ListItem::new("No profiles configured. Press [c] to create one.")]
    } else {
        model
            .store
            .profiles
            .iter()
            .enumerate()
            .map(|(idx, p)| {
                let marker = if idx == model.selected_profile { ">" } else { " " };
                ListItem::new(format!("{marker} {} ({})", p.name, p.id))
            })
            .collect()
    };
    frame.render_widget(
        List::new(items)
            .block(Block::default().title("Profile List").borders(Borders::ALL)),
        left,
    );

    let right_text = "Manage reusable skill bundles.\n\n[c] Create new profile\n[e] Edit name\n[d] Duplicate\n[x] Delete (confirm)\n[i] Import from file\n[o] Export to file\n[Enter] or [Space] Set as default\n[j/k] Move selection\n\nTo add skills: go to [3] Repositories.";
    frame.render_widget(
        Paragraph::new(right_text)
            .block(Block::default().title("Profiles — create and manage profiles").borders(Borders::ALL)),
        right,
    );
}

fn render_repositories(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let [left, right] = Layout::horizontal([Constraint::Percentage(45), Constraint::Fill(1)]).areas(area);

    let source_items: Vec<ListItem> = if model.store.sources.is_empty() {
        vec![ListItem::new("No sources. Press [n] to register a local skill directory.")]
    } else {
        model
            .store
            .sources
            .iter()
            .enumerate()
            .map(|(idx, s)| {
                let marker = if idx == model.selected_source { ">" } else { " " };
                ListItem::new(format!("{marker} {} -> {}", s.name, s.root))
            })
            .collect()
    };
    frame.render_widget(
        List::new(source_items)
            .block(Block::default().title("Registered Sources").borders(Borders::ALL)),
        left,
    );

    let discoveries = if model.discoveries.is_empty() {
        "No discoveries yet. Press [r] to scan registered sources.".to_string()
    } else {
        model
            .discoveries
            .iter()
            .enumerate()
            .map(|(idx, d)| {
                let marker = if idx == model.selected_discovery { ">" } else { " " };
                format!("{marker} {} ({})\n  {}\n  {}", d.skill_name, d.source_name, d.source_root, d.skill_path)
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    frame.render_widget(
        Paragraph::new(format!(
            "Discover and add skills to your active profile.\n\n{}\n\n[n] Add source  [r] Scan  [a]/[Space] Add to profile  [j/k] Move",
            discoveries
        ))
        .block(Block::default().title("Repositories — register sources and discover skills").borders(Borders::ALL)),
        right,
    );
}

fn render_settings(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let text = format!(
        "App-wide defaults and runtime state.\n\nProfiles stored: {}\nSources registered: {}\nDefault profile: {}\n\nSettings are persisted automatically.",
        model.store.profiles.len(),
        model.store.sources.len(),
        model.store.default_profile_id.as_deref().unwrap_or("(none)"),
    );
    frame.render_widget(
        Paragraph::new(text)
            .block(Block::default().title("System Settings").borders(Borders::ALL)),
        area,
    );
}

fn render_help(frame: &mut Frame, area: ratatui::layout::Rect) {
    let help = "Keyboard reference.\n\n[1]-[5] or [h]  Switch screen\n[Tab]           Next screen\n[q]             Quit\n\nProfiles:       [c] create  [e] edit  [d] dup  [x] delete  [i] import  [o] export\nRepositories:   [n] add source  [r] scan  [a]/[Space] add to profile\nModals:         type text  [Backspace]  [Tab] switch field  [Enter] confirm  [Esc] cancel";
    frame.render_widget(
        Paragraph::new(help)
            .block(Block::default().title("Help").borders(Borders::ALL)),
        area,
    );
}

pub fn render(frame: &mut Frame, model: &AppModel) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(8), Constraint::Length(3)]).split(frame.area());
    let tabs = "[1] Dashboard  [2] Profiles  [3] Repositories  [4] Settings  [5] Help";
    frame.render_widget(
        Paragraph::new(tabs).block(Block::default().borders(Borders::ALL)),
        chunks[0],
    );

    match model.active {
        Screen::Dashboard => render_dashboard(frame, chunks[1], model),
        Screen::Profiles => render_profiles(frame, chunks[1], model),
        Screen::Repositories => render_repositories(frame, chunks[1], model),
        Screen::SystemSettings => render_settings(frame, chunks[1], model),
        Screen::Help => render_help(frame, chunks[1]),
    }

    if let Some(modal) = model.modal {
        let mut text = match modal {
            Modal::CreateProfile => format!("Create Profile\n\nName: {}\n\n[Enter] confirm  [Esc] cancel", model.input),
            Modal::EditProfile => format!("Edit Profile\n\nName: {}\n\n[Enter] confirm  [Esc] cancel", model.input),
            Modal::DeleteProfileConfirm => "Delete selected profile? [Enter] confirm, [Esc] cancel".into(),
            Modal::ImportPath => format!("Import Profiles\n\nFile path: {}\n\n[Enter] confirm  [Esc] cancel", model.input),
            Modal::ExportPath => format!("Export Profiles\n\nFile path: {}\n\n[Enter] confirm  [Esc] cancel", model.input),
            Modal::AddSource => {
                let active_name = if model.input_focus_secondary { "" } else { " <active>" };
                let active_path = if model.input_focus_secondary { " <active>" } else { "" };
                format!(
                    "Add Local Source\n\nName{}: {}\nPath{}: {}\n\n[Tab] switch field  [Enter] confirm  [Esc] cancel",
                    active_name, model.input, active_path, model.input_secondary
                )
            }
        };
        if !model.modal_error.is_empty() {
            text.push_str(&format!("\n\nERROR: {}", model.modal_error));
        }
        frame.render_widget(
            Paragraph::new(text).block(Block::default().title(Line::from("Modal").bold()).borders(Borders::ALL)),
            chunks[1],
        );
    }

    let footer_text = if model.modal.is_some() {
        format!("[Enter] confirm  [Esc] cancel  [q] quit  — {}", model.status)
    } else {
        format!("{}  — [q] quit  [Tab] next screen  [h] help", model.status)
    };
    frame.render_widget(
        Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL)),
        chunks[2],
    );
}
