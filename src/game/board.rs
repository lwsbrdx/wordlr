use std::ops::Add;
use std::time::{Duration, Instant};

use crate::game::tile::{Tile, TileState};
use crate::game::validator::Validator;

pub(crate) const MIN_COLS: usize = 0;
pub(crate) const MAX_COLS: usize = 5;
pub(crate) const MIN_LINES: usize = 0;
pub(crate) const MAX_LINES: usize = 6;

const HIGHLIGHT_DURATION: u16 = 300;
const FLIP_STEP_MS: u64 = 16;
const FLIP_WIDTHS: [u16; 9] = [5, 4, 3, 2, 1, 2, 3, 4, 5];

#[derive(Debug)]
pub struct RevealAnimation {
    pub row: usize,
    pub final_states: [TileState; MAX_COLS],
    pub current_col: usize,
    pub flip_step: u8,
    pub next_step_at: Instant,
    pub open_stats_after: bool,
}

impl RevealAnimation {
    pub fn current_width(&self) -> u16 {
        FLIP_WIDTHS[self.flip_step as usize]
    }
}

#[derive(Debug)]
pub struct BoardState {
    pub tiles: [[Tile; MAX_COLS]; MAX_LINES],
    pub current_row: usize,
    pub current_col: usize,
    pub highlight_until: Option<Instant>,
    pub reveal_animation: Option<RevealAnimation>,
}

impl BoardState {
    pub fn new() -> Self {
        Self {
            tiles: [[Tile::default(); MAX_COLS]; MAX_LINES],
            current_row: MIN_LINES,
            current_col: MIN_COLS,
            highlight_until: None,
            reveal_animation: None,
        }
    }

    pub fn set_letter(&mut self, letter: char) {
        let cc = self.current_col;
        let cr = self.current_row;
        let tile = &mut self.tiles[cr][cc];
        tile.letter = Some(letter.to_ascii_uppercase());

        if cc == MAX_COLS - 1 {
            tile.state = TileState::Typing;
        } else {
            tile.state = TileState::Typed;
        }

        if cc < MAX_COLS - 1 {
            self.go_next_tile();
        }
    }

    pub(crate) fn init(&mut self, attempts: &[String], secret_word: String) {
        self.build_current_game(attempts);

        let v = Validator::new(secret_word);
        self.lines().iter().enumerate().for_each(|(row, l)| {
            let r = v.validate(l);
            if let Ok(states) = r {
                for (i, s) in states.iter().enumerate() {
                    self.tiles[row][i].state = *s;
                }
            }
        });
    }

    pub(crate) fn lines(&self) -> Vec<String> {
        let mut words = vec![];
        for i in 0..self.tiles.len() {
            let word = self.tiles[i]
                .iter()
                .filter_map(|t| t.letter)
                .collect::<String>();
            if word.is_empty() {
                continue;
            }
            words.push(word);
        }
        words
    }

    pub fn go_next_line(&mut self) {
        if self.current_row >= MAX_LINES - 1 {
            return;
        }

        self.current_row += 1;
        self.current_col = MIN_COLS;
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
        if self.current_col >= MAX_COLS - 1 {
            return;
        }

        self.current_col += 1;
        self.current_tile().state = TileState::Typing;
    }

    pub fn go_previous_tile(&mut self) {
        if self.current_col == MIN_COLS {
            return;
        }

        self.current_tile().state = TileState::Empty;
        self.current_col -= 1;
        self.current_tile().state = TileState::Typing;
    }

    pub fn get_current_row(&mut self) -> &mut [Tile; MAX_COLS] {
        &mut self.tiles[self.current_row]
    }

    pub fn get_current_row_word(&self) -> String {
        self.tiles[self.current_row]
            .iter()
            .filter_map(|tile| tile.letter)
            .collect::<String>()
    }

    pub fn start_reveal(&mut self, row: usize, final_states: [TileState; MAX_COLS], open_stats_after: bool) {
        self.reveal_animation = Some(RevealAnimation {
            row,
            final_states,
            current_col: 0,
            flip_step: 0,
            next_step_at: Instant::now(),
            open_stats_after,
        });
    }

    /// Avance l'animation d'un tick. Retourne `Some(open_stats)` quand l'animation est terminée.
    pub fn tick_reveal(&mut self) -> Option<bool> {
        let anim = self.reveal_animation.as_mut()?;

        if Instant::now() < anim.next_step_at {
            return None;
        }

        let flip_step = anim.flip_step;
        let col = anim.current_col;
        let row = anim.row;

        // Au milieu du flip (tranche la plus fine), basculer vers l'état final
        if flip_step == 4 {
            let final_state = anim.final_states[col];
            self.tiles[row][col].state = final_state;
        }

        anim.flip_step += 1;

        if anim.flip_step >= 9 {
            // Flip de cette tile terminé → passer à la suivante
            anim.current_col += 1;
            anim.flip_step = 0;
            anim.next_step_at = Instant::now(); // la suivante démarre immédiatement

            if anim.current_col >= MAX_COLS {
                let open_stats = anim.open_stats_after;
                self.reveal_animation = None;
                self.go_next_line();
                return Some(open_stats);
            }
        } else {
            anim.next_step_at = Instant::now() + Duration::from_millis(FLIP_STEP_MS);
        }

        None
    }

    pub fn highlight_all_tiles(&mut self) {
        self.tiles[self.current_row]
            .iter_mut()
            .for_each(|t| t.state = TileState::Highlighted);

        self.highlight_until = Some(Instant::now().add(Duration::from_millis(HIGHLIGHT_DURATION as u64)));
    }

    pub fn highlight_empty_tiles(&mut self) {
        self.tiles[self.current_row].iter_mut().for_each(|t| {
            if t.letter.is_none() {
                t.state = TileState::Highlighted
            }
        });

        self.highlight_until = Some(Instant::now().add(Duration::from_millis(HIGHLIGHT_DURATION as u64)));
    }

    pub fn unhighlight_tiles(&mut self) {
        self.tiles[self.current_row]
            .iter_mut()
            .enumerate()
            .for_each(|(index, t)| {
                if index == self.current_col {
                    t.state = TileState::Typing;
                    return;
                }

                if t.letter.is_none() {
                    t.state = TileState::Empty;
                } else {
                    t.state = TileState::Typed;
                }
            });

        self.highlight_until = None;
    }

    fn build_current_game(&mut self, attempts: &[String]) {
        attempts.iter().enumerate().for_each(|(index, attempt)| {
            let chars = attempt.chars().collect::<Vec<char>>();
            let curr_row = self.get_current_row();

            for i in MIN_COLS..MAX_COLS {
                curr_row[i].letter = Some(chars[i]);
            }

            if index < MAX_LINES - 1 {
                self.go_next_line();
            }
        });
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
