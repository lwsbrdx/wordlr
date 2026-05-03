use std::time::{Duration, Instant};
use std::ops::Add;

use crate::game::tile::{Tile, TileState};

#[derive(Debug)]
pub struct BoardState {
    pub tiles: [[Tile; 5]; 6],
    pub current_row: usize,
    pub current_col: usize,
    pub highlight_until: Option<Instant>,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::default(); 5]; 6],
            current_row: 0,
            current_col: 0,
            highlight_until: None,
        }
    }

    pub fn go_next_line(&mut self) {
        if self.current_row >= 5 {
            return;
        }

        self.current_row += 1;
        self.current_col = 0;

        self.current_tile().state = TileState::Typing;
    }

    pub fn current_tile(&mut self) -> &mut Tile {
        let cc = self.current_col;
        let cr = self.current_row;

        &mut self.tiles[cr][cc]
    }

    pub fn empty_current_tile(&mut self) {
        self.current_tile().letter = None;
    }

    pub fn go_next_tile(&mut self) {
        if self.current_col >= 4 {
            return;
        }

        self.current_col += 1;
        self.current_tile().state = TileState::Typing;
    }

    pub fn go_previous_tile(&mut self) {
        self.current_tile().state = TileState::Empty;
        self.current_col -= 1;
        self.current_tile().state = TileState::Typing;
    }

    pub fn get_current_row_word(&self) -> String {
        self.tiles[self.current_row]
            .iter()
            .filter_map(|tile| tile.letter)
            .collect::<String>()
    }

    pub fn highlight_empty_tiles(&mut self) {
        self.tiles[self.current_row].iter_mut().for_each(|t| {
            if t.letter.is_none() {
                t.state = TileState::Highlighted
            }
        });

        self.highlight_until = Some(Instant::now().add(Duration::from_secs(1)));
    }

    pub fn unhighlight_empty_tiles(&mut self) {
        self.tiles[self.current_row].iter_mut().for_each(|t| {
            if t.letter.is_none() {
                t.state = TileState::Empty
            }
        });

        self.highlight_until = None;
    }
}

impl Default for BoardState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::BoardState;

    #[test]
    fn test_get_current_word() {
        let mut state = BoardState::new();
        assert_eq!(state.get_current_row_word(), "");

        state.current_tile().letter = Some('T');
        state.go_next_tile();
        state.current_tile().letter = Some('T');
        state.go_next_tile();
        state.current_tile().letter = Some('T');
        assert_eq!(state.get_current_row_word(), "TTT");

        state.go_next_tile();
        state.current_tile().letter = Some('T');
        state.go_next_tile();
        state.current_tile().letter = Some('T');
        assert_eq!(state.get_current_row_word(), "TTTTT");
    }
}
