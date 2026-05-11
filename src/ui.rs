use ratatui::layout::{Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::model::{AppModel, Modal, Screen};

fn render_dashboard(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let [left, right] = Layout::horizontal([Constraint::Percentage(35), Constraint::Fill(1)]).areas(area);

    let profile_items: Vec<ListItem> = if model.store.profiles.is_empty() {
        vec![ListItem::new("No profiles yet. Go to Profiles and press [c].".dim())]
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
    frame.render_widget(List::new(profile_items).block(Block::default().title("Profiles").borders(Borders::ALL)), left);

    let detail = if let Some(profile) = model.store.profiles.get(model.selected_profile) {
        let default_mark = if model.store.default_profile_id.as_deref() == Some(profile.id.as_str()) {
            "yes"
        } else {
            "no"
        };
        format!(
            "Active profile: {}\nDefault: {}\n\nSkills:\n{}\n\nRules:\n{}\n\nActions: [2] Profiles [3] Repositories [q] Quit",
            profile.name,
            default_mark,
            if profile.skills.is_empty() { "(none)".into() } else { profile.skills.join("\n") },
            if profile.rules.is_empty() { "(none)".into() } else { profile.rules.join("\n") },
        )
    } else {
        "No active profile selected. Create one in Profiles screen.".to_string()
    };
    frame.render_widget(Paragraph::new(detail).block(Block::default().title("Dashboard").borders(Borders::ALL)), right);
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
    frame.render_widget(List::new(items).block(Block::default().title("Profile List").borders(Borders::ALL)), left);

    let right_text = "Actions:\n[c] Create\n[e] Edit name\n[d] Duplicate\n[x] Delete (confirm)\n[i] Import profiles\n[o] Export profiles\n[Enter]/[Space] Select as default\n[j/k] Move\n\nEmpty state is explicit and keyboard-first.";
    frame.render_widget(Paragraph::new(right_text).block(Block::default().title("Profile Actions").borders(Borders::ALL)), right);
}

fn render_repositories(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let [left, right] = Layout::horizontal([Constraint::Percentage(45), Constraint::Fill(1)]).areas(area);

    let source_items: Vec<ListItem> = if model.store.sources.is_empty() {
        vec![ListItem::new("No local sources registered. Press [n] to add one.")]
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
    frame.render_widget(List::new(source_items).block(Block::default().title("Registered Sources").borders(Borders::ALL)), left);

    let discoveries = if model.discoveries.is_empty() {
        "No discoveries yet. Press [r] to scan local sources.".to_string()
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
            "Discovery Results\n\n{}\n\nActions:\n[n] Add source\n[r] Scan\n[a]/[Space] Add selected discovery to active profile\n[j/k] Move selection",
            discoveries
        ))
        .block(Block::default().title("Repositories").borders(Borders::ALL)),
        right,
    );
}

fn render_settings(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let text = format!(
        "System Settings\n\nProfiles stored: {}\nSources registered: {}\nDefault profile: {}\n\nThis screen reflects real runtime state.\nUse Profiles and Repositories to mutate data.",
        model.store.profiles.len(),
        model.store.sources.len(),
        model.store.default_profile_id.as_deref().unwrap_or("(none)"),
    );
    frame.render_widget(Paragraph::new(text).block(Block::default().title("System Settings").borders(Borders::ALL)), area);
}

fn render_help(frame: &mut Frame, area: ratatui::layout::Rect) {
    let help = "Help\n\nGlobal: [1..5]/[h] switch screens, [Tab] next, [q] quit\nProfiles: [c] create, [e] edit, [d] duplicate, [x] delete, [i] import, [o] export, [Enter]/[Space] set default\nRepositories: [n] add source, [r] scan, [a]/[Space] add discovery to profile\nModals: type text (including spaces), [Backspace], [Tab] field switch, [Enter] confirm, [Esc] cancel";
    frame.render_widget(Paragraph::new(help).block(Block::default().title("Help").borders(Borders::ALL)), area);
}

pub fn render(frame: &mut Frame, model: &AppModel) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(8), Constraint::Length(2)]).split(frame.area());
    let tabs = "[1] Dashboard  [2] Profiles  [3] Repositories  [4] System Settings  [5] Help";
    frame.render_widget(Paragraph::new(tabs).block(Block::default().borders(Borders::ALL)), chunks[0]);

    match model.active {
        Screen::Dashboard => render_dashboard(frame, chunks[1], model),
        Screen::Profiles => render_profiles(frame, chunks[1], model),
        Screen::Repositories => render_repositories(frame, chunks[1], model),
        Screen::SystemSettings => render_settings(frame, chunks[1], model),
        Screen::Help => render_help(frame, chunks[1]),
    }

    if let Some(modal) = model.modal {
        let text = match modal {
            Modal::CreateProfile => format!("Create Profile\n\nName: {}", model.input),
            Modal::EditProfile => format!("Edit Profile\n\nName: {}", model.input),
            Modal::DeleteProfileConfirm => "Delete selected profile? [Enter] confirm, [Esc] cancel".into(),
            Modal::AddSource => {
                let active_name = if model.input_focus_secondary { "" } else { " <active>" };
                let active_path = if model.input_focus_secondary { " <active>" } else { "" };
                format!(
                    "Add Local Source\n\nName{}: {}\nPath{}: {}\n\n[Tab] switch field",
                    active_name, model.input, active_path, model.input_secondary
                )
            }
        };
        frame.render_widget(
            Paragraph::new(text).block(Block::default().title(Line::from("Modal").bold()).borders(Borders::ALL)),
            chunks[1],
        );
    }

    frame.render_widget(
        Paragraph::new("Keyboard-first runtime. Empty states are explicit.")
            .block(Block::default().title(model.status.clone()).borders(Borders::ALL)),
        chunks[2],
    );
}
