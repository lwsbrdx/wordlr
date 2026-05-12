use core::fmt;
use std::{
    io::Result,
    time::{Duration, Instant},
};

use chrono::{Days, NaiveDate, Utc};
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event, KeyEventKind},
    layout::{Constraint, Flex, Layout},
    style::Stylize,
    text::Line,
};

use crate::{
    game::{
        board::BoardState,
        endings::Endings,
        game_stats::GamesStats,
        game_store::GameStore,
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
    menu: Menu,
    selected_date: NaiveDate,
    games_stats: GamesStats,
    board_state: BoardState,
    input_mode: InputModes,
    help_visible: bool,
    stats_visible: bool,
    error: Option<String>,
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

    fn init_board_state(&mut self) {
        self.board_state = BoardState::new();

        if let Some(game) = self.games_stats.current_game(self.selected_date)
            && game.has_attempts()
        {
            let attempts = game.attempts.clone();
            let secret_word = game.secret_word.clone();
            self.board_state.init(&attempts, secret_word);
        }
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

        let [date_layout, board_layout] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(47)]).areas(mid);
        let date_line = Line::from(format!("{}", self.selected_date.format("%A %d %B %Y")))
            .bold()
            .centered();

        frame.render_widget(date_line, date_layout);

        frame.render_stateful_widget(&Board, board_layout, &mut self.board_state);

        let sb = StatusBar::new(&self.input_mode);
        frame.render_widget(&sb, bottom);

        if self.stats_visible {
            let popup_area = helpers::centered_rect(50, 65, frame.area());
            frame.render_widget(ratatui::widgets::Clear, popup_area);
            frame.render_widget(
                &Popup::new(self.games_stats.clone(), self.selected_date),
                popup_area,
            );
        }

        if self.help_visible {
            let help_area = helpers::centered_rect(60, 65, frame.area());
            frame.render_widget(ratatui::widgets::Clear, help_area);
            frame.render_widget(&Help, help_area);
        }

        if let Some(ref msg) = self.error {
            use ratatui::{
                style::Stylize,
                text::Line,
                widgets::{Block, BorderType, Paragraph},
            };
            let error_area = helpers::centered_rect(50, 20, frame.area());
            frame.render_widget(ratatui::widgets::Clear, error_area);
            frame.render_widget(
                Paragraph::new(vec![
                    Line::from(msg.as_str()),
                    Line::from(""),
                    Line::from("Appuyez sur Esc pour fermer").dark_gray(),
                ])
                .centered()
                .block(
                    Block::bordered()
                        .border_type(BorderType::Rounded)
                        .border_style(ratatui::style::Style::new().red())
                        .title(" Erreur "),
                ),
                error_area,
            );
        }
    }

    fn handle_key_pressed(&mut self, code: event::KeyCode) -> Result<()> {
        if self.error.is_some() {
            if code == event::KeyCode::Esc {
                self.error = None;
            }
            return Ok(());
        }

        if self.help_visible {
            if code == event::KeyCode::Esc || code == event::KeyCode::Char('?') {
                self.help_visible = false;
            }
            return Ok(());
        }

        if self.stats_visible {
            if code == event::KeyCode::Char('s') || code == event::KeyCode::Char('S') {
                self.stats_visible = false;
            }
            return Ok(());
        }

        match self.input_mode {
            InputModes::Insert => self.handle_insert_mode(code),
            InputModes::Normal => self.handle_normal_mode(code),
        }
    }

    fn handle_normal_mode(&mut self, code: event::KeyCode) -> Result<()> {
        match code {
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
            event::KeyCode::Char('s') | event::KeyCode::Char('S') => {
                self.stats_visible = true;
                Ok(())
            }
            event::KeyCode::Left | event::KeyCode::Char('h') => {
                self.previous_date();
                Ok(())
            }
            event::KeyCode::Right | event::KeyCode::Char('l') => {
                self.next_date();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn handle_insert_mode(&mut self, code: event::KeyCode) -> Result<()> {
        match code {
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
        }
    }

    fn input(&mut self, c: char) {
        if self
            .games_stats
            .current_game(self.selected_date)
            .and_then(|g| g.ending)
            .is_some()
        {
            return;
        }

        if !c.is_alphabetic() {
            return;
        }

        self.board_state.set_letter(c.to_ascii_uppercase());
    }

    fn normal_mode(&mut self) {
        self.input_mode = InputModes::Normal;
        self.board_state.current_tile().state = TileState::Empty;
    }

    fn insert_mode(&mut self) {
        if self
            .games_stats
            .current_game(self.selected_date)
            .and_then(|g| g.ending)
            .is_some()
        {
            return;
        }

        self.input_mode = InputModes::Insert;
        self.board_state.current_tile().state = TileState::Typing;
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn new() -> Result<Self> {
        let games_stats = GameStore::load()?;

        let mut s = Self {
            menu: Menu,
            selected_date: Utc::now().date_naive(),
            board_state: BoardState::new(),
            input_mode: InputModes::Normal,
            help_visible: false,
            stats_visible: false,
            error: None,
            exit: false,
            games_stats,
        };

        // init board_state if we already played today
        s.init_board_state();

        Ok(s)
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

        self.validate()
    }

    fn validate(&mut self) -> Result<()> {
        let date = self.selected_date;
        let secret_word = self.games_stats.current_game_mut(date).secret_word.clone();
        let validator = Validator::new(secret_word);
        let word = self.board_state.get_current_row_word();
        let validation_result = validator.validate(&word);

        match &validation_result {
            Ok(result) => {
                self.games_stats.current_game_mut(date).add_attempts(word);

                self.handle_ending(result)?;

                // propagate tiles states
                self.update_board_tiles(result);

                Ok(())
            }
            Err(e) if *e == SubmissionError::NotInDictionnary => {
                self.board_state.highlight_all_tiles();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn update_board_tiles(&mut self, result: &[TileState]) {
        let current_row = self.board_state.get_current_row();
        for index in 0..5 {
            current_row[index].state = result[index];
        }
        self.board_state.go_next_line();

        if self.input_mode == InputModes::Insert {
            self.board_state.current_tile().state = TileState::Typing;
        }
    }

    fn handle_ending(&mut self, result: &[TileState]) -> Result<()> {
        let has_won = result
            .iter()
            .position(|r| *r != TileState::Correct)
            .is_none();
        let attempts_len = self
            .games_stats
            .current_game(self.selected_date)
            .map(|g| g.attempts.len())
            .unwrap_or(0);
        let has_lost = !has_won && attempts_len >= MAX_ATTEMPTS;

        if has_won || has_lost {
            let date = self.selected_date;
            let ending = if has_won {
                Endings::Victory
            } else {
                Endings::Loss
            };
            self.games_stats.current_game_mut(date).ending = Some(ending);

            self.normal_mode();
            if let Err(e) = GameStore::save(&self.games_stats) {
                self.error = Some(format!("Impossible de sauvegarder la partie : {e}"));
            }
        }

        Ok(())
    }

    fn previous_date(&mut self) {
        if let Some(substracted_date) = self.selected_date.checked_sub_days(Days::new(1)) {
            self.selected_date = substracted_date;
            self.on_date_changed();
        }
    }

    fn next_date(&mut self) {
        let min_date = Some(Utc::now().date_naive());
        if let Some(added_date) = self
            .selected_date
            .checked_add_days(Days::new(1))
            .min(min_date)
        {
            self.selected_date = added_date;
            self.on_date_changed();
        }
    }

    fn on_date_changed(&mut self) {
        self.init_board_state();
    }
}
