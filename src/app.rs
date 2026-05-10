use crossterm::event::{self, Event, KeyCode};
use ratatui::DefaultTerminal;

use crate::AppResult;
use crate::model::{AppModel, Modal, Screen};

pub fn update(model: &mut AppModel, key: KeyCode) {
    if model.modal.is_some() {
        match key {
            KeyCode::Esc => {
                model.modal = None;
                model.status = "Modal closed".into();
            }
            KeyCode::Char('i') if model.modal == Some(Modal::Preview) => {
                model.modal = Some(Modal::InstallConfirm);
                model.status = "Install confirmation opened".into();
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
        KeyCode::Char('p') => {
            model.modal = Some(Modal::Preview);
            model.status = "Preview opened".into();
        }
        _ => {}
    }
}

pub fn run(mut terminal: DefaultTerminal) -> AppResult<()> {
    let mut model = AppModel::default();
    while !model.quit {
        terminal.draw(|frame| crate::ui::render(frame, &model))?;
        if let Event::Key(k) = event::read()? {
            update(&mut model, k.code);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crossterm::event::KeyCode;

    use crate::app::update;
    use crate::model::{AppModel, Modal};

    #[test]
    fn modal_lifecycle_preview_to_install_to_close() {
        let mut model = AppModel::default();
        update(&mut model, KeyCode::Char('p'));
        assert_eq!(model.modal, Some(Modal::Preview));

        update(&mut model, KeyCode::Char('i'));
        assert_eq!(model.modal, Some(Modal::InstallConfirm));

        update(&mut model, KeyCode::Esc);
        assert_eq!(model.modal, None);
    }

    #[test]
    fn modal_open_blocks_screen_navigation_until_closed() {
        let mut model = AppModel::default();
        update(&mut model, KeyCode::Char('p'));
        let before = model.active;
        update(&mut model, KeyCode::Tab);
        assert_eq!(model.active, before);

        update(&mut model, KeyCode::Esc);
        update(&mut model, KeyCode::Tab);
        assert_ne!(model.active, before);
    }
}
