use core::fmt;
use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Flex, Layout},
};

use crate::{
    board::{Board, BoardState},
    menu::Menu,
    status_bar::StatusBar,
};

#[derive(Debug, Clone)]
pub enum Modes {
    Normal,
    Insert,
}

impl fmt::Display for Modes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Modes::Normal => "Normal",
            Modes::Insert => "Insert",
        };

        write!(f, "{}", mode)
    }
}

#[derive(Debug)]
pub struct App {
    menu: Menu,
    board_state: BoardState,
    mode: Modes,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?
        }

        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_pressed(key_event.code)
            }
            _ => {}
        };

        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let [top, mid, bottom] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(50),
            Constraint::Length(1),
        ])
        .flex(Flex::SpaceBetween)
        .areas(area);

        frame.render_widget(&self.menu, top);

        let board = Board::new();
        frame.render_stateful_widget(&board, mid, &mut self.board_state);

        let sb = StatusBar::new(&self.mode);
        frame.render_widget(&sb, bottom);
    }

    fn handle_key_pressed(&mut self, code: event::KeyCode) {
        match code {
            event::KeyCode::Char('q') => self.exit(),
            event::KeyCode::Char('i') | event::KeyCode::Char('a') => self.insert_mode(),
            event::KeyCode::Esc => self.normal_mode(),
            _ => {}
        }
    }

    fn normal_mode(&mut self) {
        self.mode = Modes::Normal;
    }

    fn insert_mode(&mut self) {
        self.mode = Modes::Insert;
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub(crate) fn new() -> Self {
        Self {
            menu: Menu,
            board_state: BoardState::new(),
            mode: Modes::Normal,
            exit: false,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
