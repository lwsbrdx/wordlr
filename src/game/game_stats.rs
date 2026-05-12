use chrono::{NaiveDate, Utc};

use crate::game::endings::Endings;
use crate::game::dictionnary::Dictionnary;

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct GameStats {
    pub attempts: Vec<String>,
    pub ending: Option<Endings>,
    pub secret_word: String,
    pub date: String,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct GamesStats {
    pub all_games: Vec<GameStats>,
}

impl GamesStats {
    pub(crate) fn current_game(&self, date: NaiveDate) -> Option<&GameStats> {
        let date_str = date.format("%Y-%m-%d").to_string();
        self.all_games.iter().find(|g| g.date == date_str)
    }

    pub(crate) fn current_game_mut(&mut self, date: NaiveDate) -> &mut GameStats {
        let date_str = date.format("%Y-%m-%d").to_string();
        if let Some(pos) = self.all_games.iter().position(|g| g.date == date_str) {
            return &mut self.all_games[pos];
        }
        self.all_games.push(GameStats::new(Some(date)));
        self.all_games.last_mut().unwrap()
    }

    pub(crate) fn get_total_games(&self) -> usize {
        self.all_games.len()
    }

    pub(crate) fn get_win_rate(&self) -> f32 {
        let total = self.all_games.len();
        if total == 0 {
            return 0.0;
        }
        self.all_games
            .iter()
            .filter(|g| g.ending == Some(Endings::Victory))
            .count() as f32
            / total as f32
    }

    pub(crate) fn get_best_serie(&self) -> u16 {
        let mut max_serie = 0;
        let mut current_serie = 0;

        self.all_games.iter().for_each(|game| {
            if game.ending == Some(Endings::Victory) {
                current_serie += 1;
            } else {
                current_serie = 0;
            }

            if current_serie > max_serie {
                max_serie = current_serie;
            }
        });

        max_serie
    }

    pub(crate) fn get_actual_serie(&self) -> u16 {
        let mut iter = self.all_games.iter().rev();
        let mut serie = 0;

        while let Some(g) = iter.next()
            && g.ending == Some(Endings::Victory)
        {
            serie += 1;
        }

        serie
    }

    pub(crate) fn get_games_by_attempts_count(&self, number_attempts: usize) -> Vec<&GameStats> {
        self.all_games
            .iter()
            .filter(|g| g.attempts.len() == number_attempts)
            .collect()
    }
}

impl GameStats {
    pub(crate) fn new(date: Option<NaiveDate>) -> Self {
        let date = if let Some(date) = date {
            date
        } else {
            Utc::now().date_naive()
        };

        Self {
            attempts: Vec::new(),
            ending: None,
            secret_word: Dictionnary::new().get_word_for_day(date).to_owned(),
            date: date.format("%Y-%m-%d").to_string(),
        }
    }

    pub(crate) fn add_attempts(&mut self, attempt: String) {
        self.attempts.push(attempt);
    }

    pub(crate) fn has_attempts(&self) -> bool {
        !self.attempts.is_empty()
    }
}
