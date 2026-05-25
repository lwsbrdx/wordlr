use std::{io::Result, time::Duration};

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};

use crate::{
    app_state::AppState,
    game_event::{Direction, GameEvent},
    session::InputModes,
};

const FPS: u64 = 60;
const FPS_LATENCY_MS: u64 = 1000 / FPS;

pub struct InputHandler;

impl InputHandler {
    pub fn poll(state: &AppState, mode: &InputModes) -> Result<Vec<GameEvent>> {
        if let Ok(true) = event::poll(Duration::from_millis(FPS_LATENCY_MS))
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            Ok(Self::key_to_events(key.code, state, mode))
        } else {
            Ok(vec![])
        }
    }

    fn key_to_events(code: KeyCode, state: &AppState, mode: &InputModes) -> Vec<GameEvent> {
        if code == KeyCode::Char('q') {
            return vec![GameEvent::Quit];
        }

        let event = match state {
            AppState::Error(_) => match code {
                KeyCode::Esc => Some(GameEvent::DismissError),
                _ => None,
            },
            AppState::ViewingHelp => match code {
                KeyCode::Esc | KeyCode::Char('?') => Some(GameEvent::HelpToggled),
                _ => None,
            },
            AppState::ViewingStats => match code {
                KeyCode::Char('s') | KeyCode::Char('S') => Some(GameEvent::StatsToggled),
                _ => None,
            },
            AppState::Playing => match mode {
                InputModes::Normal => match code {
                    KeyCode::Char('i') => Some(GameEvent::EnterInsertMode),
                    KeyCode::Char('?') => Some(GameEvent::HelpToggled),
                    KeyCode::Char('s') | KeyCode::Char('S') => Some(GameEvent::StatsToggled),
                    KeyCode::Left | KeyCode::Char('h') => {
                        Some(GameEvent::DateChanged(Direction::Previous))
                    }
                    KeyCode::Right | KeyCode::Char('l') => {
                        Some(GameEvent::DateChanged(Direction::Next))
                    }
                    _ => None,
                },
                InputModes::Insert => match code {
                    KeyCode::Esc => Some(GameEvent::ExitInsertMode),
                    KeyCode::Char(c) => Some(GameEvent::LetterTyped(c)),
                    KeyCode::Backspace => Some(GameEvent::LetterDeleted),
                    KeyCode::Enter => Some(GameEvent::WordSubmitted),
                    _ => None,
                },
            },
        };

        event.into_iter().collect()
    }
}
