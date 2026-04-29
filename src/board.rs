use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Rect},
    widgets::{StatefulWidget, Widget},
};

use crate::tile::{Tile, TileState};

#[derive(Debug)]
pub struct Board;
impl Board {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct BoardState {
    pub tiles: [[Tile; 5]; 6], // 6 lignes, 5 colonnes
    pub current_row: usize,
    pub current_col: usize,
}

impl BoardState {
    pub(crate) fn new() -> Self {
        Self {
            tiles: [[Tile::default(); 5]; 6],
            current_row: 0,
            current_col: 0,
        }
    }

    pub(crate) fn current_tile(&mut self) -> &mut Tile {
        let cc = self.current_col;
        let cr = self.current_row;

        &mut self.tiles[cr][cc]
    }

    pub(crate) fn empty_current_tile(&mut self) {
        self.current_tile().letter = None;
    }

    pub(crate) fn go_next_tile(&mut self) {
        if self.current_col >= 4 {
            return;
        }

        self.current_col += 1;
        self.current_tile().state = TileState::Typing;
    }

    pub(crate) fn go_previous_tile(&mut self) {
        self.current_tile().state = TileState::Empty;
        self.current_col -= 1;
        self.current_tile().state = TileState::Typing;
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl StatefulWidget for &Board {
    type State = BoardState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut BoardState)
    where
        Self: Sized,
    {
        let lines = Layout::vertical([Constraint::Length(3); 6])
            .flex(Flex::Center)
            .split(area);

        for (row, line) in lines.iter().enumerate() {
            let line_layout = Layout::horizontal([Constraint::Length(5); 5])
                .flex(Flex::Center)
                .split(*line);

            for (col, el) in line_layout.iter().enumerate() {
                state.tiles[row][col].render(*el, buf);
            }
        }
    }
}
