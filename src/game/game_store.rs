use std::{
    io::{self},
    path::PathBuf,
    str::FromStr,
};

use chrono::NaiveDate;

use crate::game::game_stats::{GameStats, GamesStats};

pub struct GameStore;

impl GameStore {
    fn data_path() -> io::Result<PathBuf> {
        let data_dir = dirs::data_dir();

        if data_dir.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "cannot find data dir",
            ));
        }

        let data_dir = data_dir.unwrap();
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

    pub fn load() -> io::Result<GamesStats> {
        let json_path = Self::data_path()?;

        let json_content = std::fs::read_to_string(json_path)?;
        let mut all_games: Vec<GameStats> =
            serde_json::from_str(&json_content).map_err(to_io_err)?;

        all_games.sort_by_key(|g| NaiveDate::from_str(&g.date).ok());

        Ok(GamesStats { all_games })
    }

    pub fn save(stats: &GamesStats) -> io::Result<()> {
        let json_path = Self::data_path()?;
        let json_content = serde_json::to_string(&stats.all_games).map_err(to_io_err)?;
        std::fs::write(json_path, json_content)?;
        Ok(())
    }
}

fn to_io_err(e: serde_json::Error) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e)
}
