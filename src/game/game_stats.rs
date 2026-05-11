use std::{
    io::{self},
    path::PathBuf,
    str::FromStr,
};

use chrono::{NaiveDate, Utc};
use serde_json::Error;

use crate::{app::Endings, game::dictionnary::Dictionnary};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct GameStats {
    pub attempts: Vec<String>,
    pub ending: Option<Endings>,
    pub secret_word: String,
    pub date: String,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub(crate) struct GamesStats {
    pub current_game: GameStats,
    pub all_games: Vec<GameStats>,
}

impl GamesStats {
    fn check_paths() -> io::Result<PathBuf> {
        if dirs::data_dir().is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "cannot find data dir",
            ));
        }

        let data_dir = dirs::data_dir().unwrap();
        let wordlr_dir = data_dir.join("wordlr");
        if !wordlr_dir.exists() {
            std::fs::create_dir_all(&wordlr_dir)?;
        }

        let json_path = wordlr_dir.join("games.json");
        if !json_path.exists() {
            std::fs::write(&json_path, "[]")?;
        }

        Ok(json_path)
    }

    pub(crate) fn load() -> io::Result<Self> {
        let json_path = Self::check_paths()?;

        let json_content = std::fs::read_to_string(json_path)?;
        let mut all_games: Vec<GameStats> =
            serde_json::from_str(&json_content).map_err(to_io_err)?;
        let current_game = if let Some(todays_game) = all_games
            .iter()
            .find(|pg| pg.date == Utc::now().format("%Y-%m-%d").to_string())
        {
            todays_game.clone()
        } else {
            GameStats::new(None)
        };

        all_games.sort_by_date();

        Ok(Self {
            current_game,
            all_games,
        })
    }

    pub(crate) fn save(&mut self) -> io::Result<()> {
        let json_path = Self::check_paths()?;

        let mut game_added = false;
        for g in self.all_games.iter_mut() {
            if g.date == self.current_game.date && !self.current_game.attempts.eq(&g.attempts) {
                *g = self.current_game.clone();
                game_added = true;
            }
        }

        if !game_added {
            self.all_games.push(self.current_game.clone());
        }

        self.all_games.sort_by_date();

        let json_content = serde_json::to_string(&self.all_games.clone()).map_err(to_io_err)?;
        std::fs::write(json_path, json_content)?;

        Ok(())
    }

    pub(crate) fn get_total_games(&self) -> usize {
        self.all_games
            .iter()
            .chain(vec![&self.current_game])
            .count()
    }

    pub(crate) fn get_win_rate(&self) -> f32 {
        let all_games_iter = self.all_games.iter().chain(vec![&self.current_game]);

        all_games_iter
            .clone()
            .filter(|g| g.ending == Some(Endings::Victory))
            .count() as f32
            / all_games_iter.count() as f32
    }

    pub(crate) fn get_best_serie(&self) -> u16 {
        let mut max_serie = 0;
        let mut current_serie = 0;

        self.all_games
            .iter()
            .chain(vec![&self.current_game])
            .for_each(|game| {
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
        let mut iter = self.all_games.iter().chain(vec![&self.current_game]).rev();
        let mut serie = 0;

        while let Some(g) = iter.next()
            && g.ending == Some(Endings::Victory)
        {
            serie += 1;
        }

        serie
    }

    pub(crate) fn get_games_by_attempts_count(&self, number_attempts: usize) -> Vec<&GameStats> {
        let iter = self.all_games.iter().chain(vec![&self.current_game]);
        iter.filter(|g| g.attempts.len() == number_attempts)
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

trait SortByDate {
    fn sort_by_date(&mut self);
}

impl SortByDate for Vec<GameStats> {
    fn sort_by_date(&mut self) {
        self.sort_by(|pga, pgb| {
            let pga_date = NaiveDate::from_str(&pga.date);
            let pgb_date = NaiveDate::from_str(&pgb.date);

            if let Ok(pga_date) = pga_date
                && let Ok(pgb_date) = pgb_date
            {
                return pga_date.cmp(&pgb_date);
            }

            std::cmp::Ordering::Equal
        });
    }
}

fn to_io_err(e: Error) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e)
}
