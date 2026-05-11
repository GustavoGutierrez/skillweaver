use ratatui::layout::{Constraint, Layout};
use ratatui::style::Stylize;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Frame;

use crate::model::{AppModel, Modal, Screen};

fn render_dashboard(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let [left, right] = Layout::horizontal([Constraint::Percentage(35), Constraint::Fill(1)]).areas(area);

    let profile_items: Vec<ListItem> = if model.store.profiles.is_empty() {
        vec![ListItem::new(Span::from("No profiles yet. Go to Profiles screen [2] to create one.").dim())]
    } else {
        model.store.profiles.iter().enumerate().map(|(idx, p)| {
            let marker = if idx == model.selected_profile { "▸" } else { " " };
            let span = if idx == model.selected_profile {
                Span::from(format!("{marker} {}", p.name)).cyan().bold()
            } else {
                Span::from(format!("{marker} {}", p.name))
            };
            ListItem::new(span)
        }).collect()
    };
    frame.render_widget(
        List::new(profile_items).block(Block::default().title(" Profiles ").borders(Borders::ALL).cyan()),
        left,
    );

    let detail: Vec<Line> = if let Some(profile) = model.store.profiles.get(model.selected_profile) {
        let default_mark = if model.store.default_profile_id.as_deref() == Some(profile.id.as_str()) {
            Span::from("yes").green()
        } else {
            Span::from("no").dim()
        };
        let mut lines = vec![
            Line::from(vec![Span::from("Active profile: ").bold(), Span::from(&profile.name).cyan().bold()]),
            Line::from(vec![Span::from("Default: ").bold(), default_mark]),
            Line::from(""),
            Line::from(Span::from("Skills:").bold().green()),
        ];
        if profile.skills.is_empty() {
            lines.push(Line::from(Span::from("  (none — press [s] to add)").dim()));
        } else {
            for skill in &profile.skills {
                lines.push(Line::from(format!("  • {skill}")));
            }
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::from("Rules:").bold().yellow()));
        if profile.rules.is_empty() {
            lines.push(Line::from(Span::from("  (none — press [r] to add)").dim()));
        } else {
            for rule in &profile.rules {
                lines.push(Line::from(format!("  • {rule}")));
            }
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::from("[2] Profiles  [3] Repositories  [q] Quit").dim()));
        lines
    } else {
        vec![
            Line::from(Span::from("Welcome!").cyan().bold()),
            Line::from(""),
            Line::from("Create a profile in [2] Profiles, then add skills from [3] Repositories."),
        ]
    };
    frame.render_widget(
        Paragraph::new(detail).block(Block::default().title(" Dashboard — overview ").borders(Borders::ALL).cyan()),
        right,
    );
}

fn render_profiles(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let [left, right] = Layout::horizontal([Constraint::Percentage(45), Constraint::Fill(1)]).areas(area);
    let items: Vec<ListItem> = if model.store.profiles.is_empty() {
        vec![ListItem::new(Span::from("No profiles configured. Press [c] to create one.").dim())]
    } else {
        model.store.profiles.iter().enumerate().map(|(idx, p)| {
            let marker = if idx == model.selected_profile { "▸" } else { " " };
            let span = if idx == model.selected_profile {
                Span::from(format!("{marker} {} ({})", p.name, p.id)).cyan().bold()
            } else {
                Span::from(format!("{marker} {} ({})", p.name, p.id))
            };
            ListItem::new(span)
        }).collect()
    };
    frame.render_widget(
        List::new(items).block(Block::default().title(" Profile List ").borders(Borders::ALL).cyan()),
        left,
    );

    let detail: Vec<Line> = if let Some(profile) = model.store.profiles.get(model.selected_profile) {
        let mut lines = vec![
            Line::from(vec![Span::from("Profile: ").bold(), Span::from(&profile.name).cyan().bold()]),
            Line::from(""),
            Line::from(Span::from("Skills:").bold().green()),
        ];
        if profile.skills.is_empty() {
            lines.push(Line::from(Span::from("  (none — press [s] to add)").dim()));
        } else {
            for skill in &profile.skills {
                lines.push(Line::from(format!("  • {skill}")));
            }
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::from("Rules:").bold().yellow()));
        if profile.rules.is_empty() {
            lines.push(Line::from(Span::from("  (none — press [r] to add)").dim()));
        } else {
            for rule in &profile.rules {
                lines.push(Line::from(format!("  • {rule}")));
            }
        }
        lines.push(Line::from(""));
        lines.push(Line::from(Span::from("━".repeat(40)).dim()));
        lines.push(Line::from(Span::from("[c] Create  [e] Edit  [s] Add skill  [r] Add rule").bold()));
        lines.push(Line::from(Span::from("[d] Duplicate  [x] Delete  [i] Import  [o] Export").bold()));
        lines.push(Line::from(Span::from("[Enter]/[Space] Set default  [j/k] Move").dim()));
        lines
    } else {
        vec![Line::from(Span::from("No profile selected.").dim())]
    };
    frame.render_widget(
        Paragraph::new(detail).block(Block::default().title(" Profiles — manage profiles ").borders(Borders::ALL).cyan()),
        right,
    );
}

fn render_repositories(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let [left, right] = Layout::horizontal([Constraint::Percentage(45), Constraint::Fill(1)]).areas(area);

    let source_items: Vec<ListItem> = if model.store.sources.is_empty() {
        vec![ListItem::new(Span::from("No sources. Press [n] to register a local skill directory.").dim())]
    } else {
        model.store.sources.iter().enumerate().map(|(idx, s)| {
            let marker = if idx == model.selected_source { "▸" } else { " " };
            let span = if idx == model.selected_source {
                Span::from(format!("{marker} {} → {}", s.name, s.root)).cyan().bold()
            } else {
                Span::from(format!("{marker} {} → {}", s.name, s.root))
            };
            ListItem::new(span)
        }).collect()
    };
    frame.render_widget(
        List::new(source_items).block(Block::default().title(" Registered Sources ").borders(Borders::ALL).cyan()),
        left,
    );

    let discoveries: Vec<Line> = if model.discoveries.is_empty() {
        vec![Line::from(Span::from("No discoveries yet. Press [r] to scan.").dim())]
    } else {
        model.discoveries.iter().enumerate().map(|(idx, d)| {
            let marker = if idx == model.selected_discovery { "▸" } else { " " };
            let name_span = if idx == model.selected_discovery {
                Span::from(format!("{marker} {}", d.skill_name)).cyan().bold()
            } else {
                Span::from(format!("{marker} {}", d.skill_name))
            };
            Line::from(vec![name_span, Span::from(format!("  ({})", d.source_name)).dim()])
        }).collect()
    };

    let mut lines = vec![
        Line::from(Span::from("Discover and add skills to your active profile.").green()),
        Line::from(""),
    ];
    lines.extend(discoveries);
    lines.push(Line::from(""));
    lines.push(Line::from(Span::from("━".repeat(40)).dim()));
    lines.push(Line::from(Span::from("[n] Add source  [r] Scan  [a]/[Space] Add to profile").bold()));
    lines.push(Line::from(Span::from("[j/k] Move selection").dim()));

    frame.render_widget(
        Paragraph::new(lines).block(Block::default().title(" Repositories — discover skills ").borders(Borders::ALL).cyan()),
        right,
    );
}

fn render_settings(frame: &mut Frame, area: ratatui::layout::Rect, model: &AppModel) {
    let text = vec![
        Line::from(Span::from("App-wide defaults and runtime state.").bold()),
        Line::from(""),
        Line::from(vec![Span::from("Profiles stored: ").bold(), Span::from(model.store.profiles.len().to_string()).cyan()]),
        Line::from(vec![Span::from("Sources registered: ").bold(), Span::from(model.store.sources.len().to_string()).cyan()]),
        Line::from(vec![Span::from("Default profile: ").bold(), Span::from(model.store.default_profile_id.as_deref().unwrap_or("(none)")).green()]),
        Line::from(""),
        Line::from(Span::from("Settings are persisted automatically.").dim()),
    ];
    frame.render_widget(
        Paragraph::new(text).block(Block::default().title(" System Settings ").borders(Borders::ALL).cyan()),
        area,
    );
}

fn render_help(frame: &mut Frame, area: ratatui::layout::Rect) {
    let help = vec![
        Line::from(Span::from("Keyboard reference").bold().cyan()),
        Line::from(""),
        Line::from(Span::from("Navigation").bold().green()),
        Line::from(Span::from("  [1]-[5] or [h]  Switch screen  |  [Tab]  Next screen  |  [q]  Quit").dim()),
        Line::from(""),
        Line::from(Span::from("Profiles [2]").bold().green()),
        Line::from(Span::from("  [c] Create  [e] Edit name  [s] Add skill  [r] Add rule  [d] Duplicate  [x] Delete  [i] Import  [o] Export").dim()),
        Line::from(""),
        Line::from(Span::from("Repositories [3]").bold().green()),
        Line::from(Span::from("  [n] Add source  [r] Scan  [a]/[Space] Add to profile").dim()),
        Line::from(""),
        Line::from(Span::from("Modals").bold().yellow()),
        Line::from(Span::from("  Type text  [Backspace] erase  [Tab] switch field  [Enter] confirm  [Esc] cancel  [q] quit").dim()),
    ];
    frame.render_widget(
        Paragraph::new(help).block(Block::default().title(" Help ").borders(Borders::ALL).cyan()),
        area,
    );
}

fn tab_label(screen: Screen, active: Screen, name: &str, num: usize) -> Span<'static> {
    let marker = if active == screen { "●" } else { "○" };
    let label = format!(" [{num}] {name} ");
    if active == screen {
        Span::from(format!("{marker}{label}")).cyan().bold()
    } else {
        Span::from(format!("{marker}{label}")).dim()
    }
}

pub fn render(frame: &mut Frame, model: &AppModel) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(8), Constraint::Length(3)]).split(frame.area());

    let tabs = Line::from(vec![
        tab_label(Screen::Dashboard, model.active, "Dashboard", 1),
        Span::from(" │ ").dim(),
        tab_label(Screen::Profiles, model.active, "Profiles", 2),
        Span::from(" │ ").dim(),
        tab_label(Screen::Repositories, model.active, "Repositories", 3),
        Span::from(" │ ").dim(),
        tab_label(Screen::SystemSettings, model.active, "Settings", 4),
        Span::from(" │ ").dim(),
        tab_label(Screen::Help, model.active, "Help", 5),
    ]);
    frame.render_widget(Paragraph::new(tabs).block(Block::default().borders(Borders::ALL)), chunks[0]);

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
            Modal::AddSkill => format!("Add Skill to Profile\n\nSkill name: {}\n\n[Enter] confirm  [Esc] cancel", model.input),
            Modal::AddRule => format!("Add Rule to Profile\n\nRule: {}\n\n[Enter] confirm  [Esc] cancel", model.input),
            Modal::AddSource => {
                let an = if model.input_focus_secondary { "" } else { " <active>" };
                let ap = if model.input_focus_secondary { " <active>" } else { "" };
                format!("Add Local Source\n\nName{an}: {}\nPath{ap}: {}\n\n[Tab] switch field  [Enter] confirm  [Esc] cancel", model.input, model.input_secondary)
            }
        };
        if !model.modal_error.is_empty() {
            text.push_str(&format!("\n\nERROR: {}", model.modal_error));
        }
        frame.render_widget(
            Paragraph::new(text).block(Block::default().title(Line::from(" Modal ").bold().yellow()).borders(Borders::ALL).yellow()),
            chunks[1],
        );
    }

    let footer_text: Line = if model.modal.is_some() {
        Line::from(vec![
            Span::from("[Enter] confirm  ").green(),
            Span::from("[Esc] cancel  ").red(),
            Span::from("[q] quit").dim(),
            Span::from(format!("  — {}", model.status)).dim(),
        ])
    } else {
        Line::from(vec![
            Span::from(model.status.clone()),
            Span::from("  —  ").dim(),
            Span::from("[q] quit  ").red(),
            Span::from("[Tab] next  ").dim(),
            Span::from("[h] help").dim(),
        ])
    };
    frame.render_widget(Paragraph::new(footer_text).block(Block::default().borders(Borders::ALL)), chunks[2]);
}
