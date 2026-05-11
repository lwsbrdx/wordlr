use core::fmt;
use std::{
    io::Result,
    time::{Duration, Instant},
};

use chrono::Utc;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Flex, Layout},
};

use crate::{
    game::{
        board::BoardState,
        dictionnary::Dictionnary,
        game_stats::GamesStats,
        tile::TileState,
        validator::{SubmissionError, Validator},
    },
    helpers,
    ui::{board::Board, help::Help, menu::Menu, popup::Popup, status_bar::StatusBar},
};

const MAX_ATTEMPTS: usize = 6;

#[derive(Debug, Clone, PartialEq)]
pub enum InputModes {
    Normal,
    Insert,
}

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum Endings {
    Victory,
    Loss,
}

impl fmt::Display for InputModes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            InputModes::Normal => "Normal",
            InputModes::Insert => "Insert",
        };

        write!(f, "{}", mode)
    }
}

#[derive(Debug)]
pub struct App {
    secret_word: String,
    menu: Menu,
    games_stats: GamesStats,
    board_state: BoardState,
    input_mode: InputModes,
    help_visible: bool,
    exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.games_stats = GamesStats::load()?;

        // init board_state if we already played today
        if self.games_stats.current_game.has_attemps() {
            // add attemps to board_state
            let attempts = &self.games_stats.current_game.attempts;
            self.board_state.build_current_game(attempts);
        }

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
                self.handle_key_pressed(event.code)?
            }
        } else if let Some(i) = self.board_state.highlight_until
            && Instant::now() > i
        {
            self.board_state.unhighlight_tiles();
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

        let sb = StatusBar::new(&self.input_mode);
        frame.render_widget(&sb, bottom);

        if self.games_stats.current_game.ending.is_some() {
            let popup_area = helpers::centered_rect(50, 55, frame.area());
            frame.render_widget(ratatui::widgets::Clear, popup_area);
            frame.render_widget(&Popup::new(self.games_stats.clone()), popup_area);
        }

        if self.help_visible {
            let help_area = helpers::centered_rect(60, 85, frame.area());
            frame.render_widget(ratatui::widgets::Clear, help_area);
            frame.render_widget(&Help, help_area);
        }
    }

    fn handle_key_pressed(&mut self, code: event::KeyCode) -> Result<()> {
        if self.help_visible {
            if code == event::KeyCode::Esc {
                self.help_visible = false;
            }
            return Ok(());
        }

        match self.input_mode {
            InputModes::Insert => match code {
                event::KeyCode::Esc => {
                    self.normal_mode();
                    Ok(())
                }
                event::KeyCode::Char(c) => {
                    self.input(c);
                    Ok(())
                }
                event::KeyCode::Backspace => {
                    self.delete();
                    Ok(())
                }
                event::KeyCode::Enter => {
                    self.submit()?;
                    Ok(())
                }
                _ => Ok(()),
            },
            InputModes::Normal => match code {
                event::KeyCode::Char('q') => {
                    self.exit();
                    Ok(())
                }
                event::KeyCode::Char('i') => {
                    self.insert_mode();
                    Ok(())
                }
                event::KeyCode::Char('?') => {
                    self.help_visible = true;
                    Ok(())
                }
                _ => Ok(()),
            },
        }
    }

    fn input(&mut self, c: char) {
        if self.games_stats.current_game.ending.is_some() {
            return;
        }

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
        self.input_mode = InputModes::Normal;
        self.board_state.current_tile().state = TileState::Empty;
    }

    fn insert_mode(&mut self) {
        if self.games_stats.current_game.ending.is_some() {
            return;
        }

        self.input_mode = InputModes::Insert;
        self.board_state.current_tile().state = TileState::Typing;
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn new() -> Self {
        Self {
            secret_word: Dictionnary::new().get_word_for_day(Utc::now().date_naive()),
            menu: Menu,
            board_state: BoardState::new(),
            input_mode: InputModes::Normal,
            help_visible: false,
            exit: false,
            games_stats: GamesStats::default(),
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

    fn submit(&mut self) -> Result<()> {
        if self.board_state.current_col < 4 {
            self.board_state.highlight_empty_tiles();
            return Ok(());
        }

        let validator = Validator::new(self.secret_word.clone());
        let word = self.board_state.get_current_row_word();
        let validation_result = validator.validate(&word);

        match &validation_result {
            Ok(result) => {
                self.games_stats.current_game.attempts.push(word);

                let has_won = result
                    .iter()
                    .position(|r| *r != TileState::Correct)
                    .is_none();
                let has_lost = !has_won && self.games_stats.current_game.attempts.len() >= MAX_ATTEMPTS;

                if has_won || has_lost {
                    // handle_victory
                    if has_won {
                        self.games_stats.current_game.ending = Some(Endings::Victory);
                    }

                    // handle loss
                    if has_lost {
                        self.games_stats.current_game.ending = Some(Endings::Loss);
                    }

                    self.input_mode = InputModes::Normal;
                    self.games_stats.current_game.secret_word = self.secret_word.clone();
                    self.games_stats.save()?;
                }

                // propagate tiles states
                let current_row = self.board_state.get_current_row();
                for index in 0..5 {
                    current_row[index].state = result[index];
                }

                self.board_state.go_next_line();
                Ok(())
            }
            Err(e) if *e == SubmissionError::NotInDictionnary => {
                self.board_state.highlight_all_tiles();
                Ok(())
            }
            _ => Ok(()),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
