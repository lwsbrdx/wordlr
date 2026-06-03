use core::fmt;
use std::{io::Result, time::Instant};

use chrono::{Days, NaiveDate, Utc};

use crate::{
    game::{
        board::{BoardState, MAX_COLS, MAX_LINES},
        endings::Endings,
        game_stats::GamesStats,
        game_store::GameStore,
        tile::TileState,
        validator::{SubmissionError, Validator},
    },
    game_event::{Direction, GameEvent},
};

const BEFORE_OPEN_STATS_DURATION: u128 = 500;

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

/// Signaux que la session envoie à App pour modifier l'état UI.
#[derive(Debug)]
pub enum SessionSignal {
    OpenStats,
    ToggleStats,
    ToggleHelp,
    DismissError,
    SetError(String),
    Quit,
}

#[derive(Debug)]
pub struct GameSession {
    pub board_state: BoardState,
    pub games_stats: GamesStats,
    pub selected_date: NaiveDate,
    pub input_mode: InputModes,
    game_ended_at: Option<Instant>,
}

impl GameSession {
    pub fn new() -> Result<Self> {
        let games_stats = GameStore::load()?;
        let selected_date = Utc::now().date_naive();

        let mut s = Self {
            board_state: BoardState::new(),
            games_stats,
            selected_date,
            input_mode: InputModes::Normal,
            game_ended_at: None,
        };

        if s.games_stats.current_game(s.selected_date).is_some() {
            s.init_board_state();
        } else {
            s.games_stats.new_game_mut(s.selected_date);
        }

        Ok(s)
    }

    /// Vérifie les timers à chaque frame. Retourne des signaux si nécessaire.
    pub fn tick(&mut self) -> Vec<SessionSignal> {
        let mut signals = vec![];

        if let Some(i) = self.board_state.highlight_until
            && Instant::now() > i
        {
            self.board_state.unhighlight_tiles();
        }

        if let Some(open_stats) = self.board_state.tick_reveal() {
            if self.input_mode == InputModes::Insert {
                self.board_state.current_tile().state = TileState::Typing;
            }
            if open_stats {
                self.game_ended_at = Some(Instant::now());
            }
        }

        if let Some(i) = self.game_ended_at
            && Instant::now().duration_since(i).as_millis() >= BEFORE_OPEN_STATS_DURATION
        {
            signals.push(SessionSignal::OpenStats);
            self.game_ended_at = None;
        }

        signals
    }

    /// Traite une liste d'events. Retourne des signaux pour App.
    pub fn update(&mut self, events: &[GameEvent]) -> Result<Vec<SessionSignal>> {
        let mut all_signals = vec![];
        for event in events {
            let mut signals = self.handle_event(event)?;
            all_signals.append(&mut signals);
        }
        Ok(all_signals)
    }

    fn handle_event(&mut self, event: &GameEvent) -> Result<Vec<SessionSignal>> {
        match event {
            GameEvent::Quit => Ok(vec![SessionSignal::Quit]),
            GameEvent::StatsToggled => Ok(vec![SessionSignal::ToggleStats]),
            GameEvent::HelpToggled => Ok(vec![SessionSignal::ToggleHelp]),
            GameEvent::DismissError => Ok(vec![SessionSignal::DismissError]),
            GameEvent::LetterTyped(c) => {
                self.input(*c);
                Ok(vec![])
            }
            GameEvent::WordSubmitted => self.submit(),
            GameEvent::LetterDeleted => {
                self.delete();
                Ok(vec![])
            }
            GameEvent::EnterInsertMode => {
                self.insert_mode();
                Ok(vec![])
            }
            GameEvent::ExitInsertMode => {
                self.normal_mode();
                Ok(vec![])
            }
            GameEvent::DateChanged(dir) => {
                match dir {
                    Direction::Previous => self.previous_date(),
                    Direction::Next => self.next_date(),
                }
                Ok(vec![])
            }
        }
    }

    fn init_board_state(&mut self) {
        self.board_state = BoardState::new();

        if let Some(game) = self.games_stats.current_game(self.selected_date)
            && game.has_attempts()
        {
            let attempts = &game.attempts;
            let secret_word = game.secret_word.clone();
            self.board_state.init(attempts, secret_word);
        }
    }

    fn input(&mut self, c: char) {
        if self.board_state.reveal_animation.is_some() {
            return;
        }
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

    fn insert_mode(&mut self) {
        if self.board_state.reveal_animation.is_some() {
            return;
        }
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

    fn normal_mode(&mut self) {
        if self.board_state.reveal_animation.is_some() {
            return;
        }
        self.input_mode = InputModes::Normal;
        self.board_state.current_tile().state = TileState::Empty;
    }

    fn delete(&mut self) {
        if self.board_state.reveal_animation.is_some() {
            return;
        }
        let cc = self.board_state.current_col;

        if self.board_state.current_tile().letter.is_none() && cc > 0 {
            self.board_state.go_previous_tile();
        }

        if self.board_state.current_tile().letter.is_some() {
            self.board_state.empty_current_tile();
        }
    }

    fn submit(&mut self) -> Result<Vec<SessionSignal>> {
        self.validate()
    }

    fn validate(&mut self) -> Result<Vec<SessionSignal>> {
        if self.board_state.reveal_animation.is_some() {
            return Ok(vec![]);
        }

        let date = self.selected_date;
        let secret_word = self.games_stats.current_game_mut(date).secret_word.clone();
        let validator = Validator::new(secret_word);
        let word = self.board_state.get_current_row_word();
        let validation_result = validator.validate(&word);

        match &validation_result {
            Ok(result) => {
                self.games_stats.current_game_mut(date).add_attempts(word);
                let (signals, open_stats) = self.handle_ending(result)?;
                self.start_reveal_animation(result, open_stats);
                Ok(signals)
            }
            Err(e) if *e == SubmissionError::NotInDictionnary => {
                self.board_state.highlight_all_tiles();
                Ok(vec![])
            }
            Err(e) if *e == SubmissionError::TooShort => {
                self.board_state.highlight_empty_tiles();
                Ok(vec![])
            }
            _ => Ok(vec![]),
        }
    }

    fn start_reveal_animation(&mut self, result: &[TileState], open_stats_after: bool) {
        let row = self.board_state.current_row;
        let mut final_states = [TileState::Empty; MAX_COLS];
        for (i, s) in result.iter().enumerate().take(MAX_COLS) {
            final_states[i] = *s;
        }
        self.board_state.start_reveal(row, final_states, open_stats_after);
    }

    fn handle_ending(&mut self, result: &[TileState]) -> Result<(Vec<SessionSignal>, bool)> {
        let has_won = result.iter().all(|r| *r == TileState::Correct);
        let attempts_len = self
            .games_stats
            .current_game(self.selected_date)
            .map(|g| g.attempts.len())
            .unwrap_or(0);
        let has_lost = !has_won && attempts_len >= MAX_LINES;

        if has_won || has_lost {
            let date = self.selected_date;
            let ending = if has_won { Endings::Victory } else { Endings::Loss };
            self.games_stats.current_game_mut(date).ending = Some(ending);

            self.normal_mode();

            if let Err(e) = GameStore::save(&self.games_stats) {
                return Ok((
                    vec![SessionSignal::SetError(format!(
                        "Impossible de sauvegarder la partie : {e}"
                    ))],
                    false,
                ));
            }

            return Ok((vec![], true));
        }

        Ok((vec![], false))
    }

    fn previous_date(&mut self) {
        if let Some(d) = self.selected_date.checked_sub_days(Days::new(1)) {
            self.selected_date = d;
            self.on_date_changed();
        }
    }

    fn next_date(&mut self) {
        let min_date = Some(Utc::now().date_naive());
        if let Some(d) = self
            .selected_date
            .checked_add_days(Days::new(1))
            .min(min_date)
        {
            self.selected_date = d;
            self.on_date_changed();
        }
    }

    fn on_date_changed(&mut self) {
        self.init_board_state();
    }
}
