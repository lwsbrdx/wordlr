use core::fmt;
use std::{
    io::Result,
    time::{Duration, Instant},
};

use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Flex, Layout},
};

use crate::{
    game::{
        board::BoardState,
        tile::TileState,
        validator::Validator,
    },
    ui::{
        board::Board,
        menu::Menu,
        status_bar::StatusBar,
    },
};

#[derive(Debug, Clone, PartialEq)]
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
        if let Ok(true) = event::poll(Duration::from_millis(100)) {
            if let Event::Key(event) = event::read()?
                && event.kind == KeyEventKind::Press
            {
                self.handle_key_pressed(event.code)
            }
        } else if let Some(i) = self.board_state.highlight_until
            && Instant::now() > i
        {
            self.board_state.unhighlight_empty_tiles();
        }

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
        if self.mode == Modes::Insert {
            match code {
                event::KeyCode::Esc => self.normal_mode(),
                event::KeyCode::Char(c) => self.input(c),
                event::KeyCode::Backspace => self.delete(),
                event::KeyCode::Enter => self.submit(),
                _ => {}
            }
        }

        if self.mode == Modes::Normal {
            match code {
                event::KeyCode::Char('q') => self.exit(),
                event::KeyCode::Char('i') => self.insert_mode(),
                _ => {}
            }
        }
    }

    fn input(&mut self, c: char) {
        if !c.is_alphabetic() {
            return;
        }

        let cc = self.board_state.current_col;
        let cr = self.board_state.current_row;
        let tile = &mut self.board_state.tiles[cr][cc];
        tile.letter = Some(c.to_ascii_uppercase());

        if cc == 4 {
            tile.state = TileState::Typing;
        } else {
            tile.state = TileState::Typed;
        }

        if self.board_state.current_col < 4 {
            self.board_state.go_next_tile();
        }
    }

    fn normal_mode(&mut self) {
        self.mode = Modes::Normal;
        self.board_state.current_tile().state = TileState::Empty;
    }

    fn insert_mode(&mut self) {
        self.mode = Modes::Insert;
        self.board_state.current_tile().state = TileState::Typing;
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn new() -> Self {
        Self {
            menu: Menu,
            board_state: BoardState::new(),
            mode: Modes::Normal,
            exit: false,
        }
    }

    fn delete(&mut self) {
        let cc = self.board_state.current_col;

        if self.board_state.current_tile().letter.is_none() && cc > 0 {
            self.board_state.go_previous_tile();
        }

        if self.board_state.current_tile().letter.is_some() {
            self.board_state.empty_current_tile();
        }
    }

    fn submit(&mut self) {
        if self.board_state.current_col < 4 {
            self.board_state.highlight_empty_tiles();
            return;
        }

        let word = self.board_state.get_current_row_word();
        let validator = Validator::new("POMME"); // TODO generate word based on today
        let validation_result = validator.validate(word);
        let current_row = &mut self.board_state.tiles[self.board_state.current_row];

        if let Ok(result_states) = validation_result {
            for index in 0..5 {
                current_row[index].state = result_states[index]
            }
        }

        self.board_state.go_next_line();
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
