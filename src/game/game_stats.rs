use std::{
    io::{self},
    path::PathBuf,
    str::FromStr,
};

use chrono::{NaiveDate, Utc};
use serde_json::Error;

use crate::app::Endings;

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
    pub previous_games: Vec<GameStats>,
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
        let previous_games: Vec<GameStats> =
            serde_json::from_str(&json_content).map_err(to_io_err)?;
        let current_game = if let Some(todays_game) = previous_games
            .iter()
            .find(|pg| pg.date == Utc::now().format("%Y-%m-%d").to_string())
        {
            todays_game.clone()
        } else {
            GameStats::new()
        };

        let mut previous_games: Vec<GameStats> = previous_games
            .iter()
            .filter(|pg| pg.date != current_game.date)
            .cloned()
            .collect();

        previous_games.sort_by(|pga, pgb| {
            let pga_date = NaiveDate::from_str(&pga.date);
            let pgb_date = NaiveDate::from_str(&pgb.date);

            if let Ok(pga_date) = pga_date
                && let Ok(pgb_date) = pgb_date
            {
                return pga_date.cmp(&pgb_date);
            }

            std::cmp::Ordering::Equal
        });

        Ok(Self {
            current_game,
            previous_games,
        })
    }

    pub(crate) fn save(&self) -> io::Result<()> {
        let json_path = Self::check_paths()?;
        let mut games = self.previous_games.clone();
        games.push(self.current_game.clone());

        let json_content = serde_json::to_string(&games).map_err(to_io_err)?;
        std::fs::write(json_path, json_content)?;

        Ok(())
    }

    pub(crate) fn get_total_games(&self) -> usize {
        self.previous_games.iter().chain(vec![&self.current_game]).count()
    }

    pub(crate) fn get_win_rate(&self) -> f32 {
        let all_games_iter = self.previous_games.iter().chain(vec![&self.current_game]);

        all_games_iter
            .clone()
            .filter(|g| g.ending == Some(Endings::Victory))
            .count() as f32
            / all_games_iter.count() as f32
    }

    pub(crate) fn get_best_serie(&self) -> u16 {
        let mut max_serie = 0;
        let mut current_serie = 0;

        self.previous_games.iter().chain(vec![&self.current_game]).for_each(|game| {
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
        let mut iter = self.previous_games.iter().chain(vec![&self.current_game]).rev();
        let mut serie = 0;

        while let Some(g) = iter.next() && g.ending == Some(Endings::Victory) {
            serie += 1;
        }

        serie
    }
}

impl GameStats {
    pub(crate) fn new() -> Self {
        Self {
            attempts: Vec::new(),
            ending: None,
            secret_word: "".to_owned(),
            date: Utc::now().date_naive().format("%Y-%m-%d").to_string(),
        }
    }

    pub(crate) fn has_attemps(&self) -> bool {
        !self.attempts.is_empty()
    }
}

fn to_io_err(e: Error) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e)
}
