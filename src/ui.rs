use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::{Frame, style::Stylize};

use crate::model::{AppModel, Modal, Screen};

pub fn render(frame: &mut Frame, model: &AppModel) {
    let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(5), Constraint::Length(2)])
        .split(frame.area());
    let tabs = "[1] Dashboard  [2] Profiles  [3] Repositories  [4] System Settings  [5] Help";
    frame.render_widget(Paragraph::new(tabs).block(Block::default().borders(Borders::ALL)), chunks[0]);
    let title = match model.active {
        Screen::Dashboard => "Dashboard",
        Screen::Profiles => "Profiles",
        Screen::Repositories => "Repositories",
        Screen::SystemSettings => "System Settings",
        Screen::Help => "Help",
    };
    frame.render_widget(
        Paragraph::new(Line::from(title).bold())
            .block(Block::default().title("SkillWeaver").borders(Borders::ALL)),
        chunks[1],
    );

    if let Some(modal) = model.modal {
        let modal_text = match modal {
            Modal::Preview => "Preview modal (i: install, Esc: close)",
            Modal::InstallConfirm => "Install confirm modal (Esc: close)",
        };
        frame.render_widget(
            Paragraph::new(modal_text)
                .block(Block::default().title("Modal").borders(Borders::ALL)),
            chunks[1],
        );
    }

    frame.render_widget(
        Paragraph::new("[Tab] Next Screen  [p] Preview  [q] Quit  [h] Help")
            .block(Block::default().title(model.status.clone()).borders(Borders::ALL)),
        chunks[2],
    );
}
