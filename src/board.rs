use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Rect},
    widgets::{StatefulWidget, Widget},
};

use crate::tile::Tile;

#[derive(Debug)]
pub struct Board;
impl Board {
    pub(crate) fn new() -> Self {
        Self
    }
}

#[derive(Debug)]
pub struct BoardState {
    tiles: [[Tile; 5]; 6], // 6 lignes, 5 colonnes
    current_row: usize,
    current_col: usize,
}

impl BoardState {
    pub(crate) fn new() -> Self {
        Self {
            tiles: [[Tile::default(); 5]; 6],
            current_row: 0,
            current_col: 0,
        }
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
        let lines = Layout::vertical([Constraint::Length(4); 6])
            .flex(Flex::Center)
            .split(area);

        for (row, line) in lines.iter().enumerate() {
            let line_layout = Layout::horizontal([Constraint::Length(8); 5])
                .flex(Flex::Center)
                .split(*line);

            for (col, el) in line_layout.iter().enumerate() {
                state.tiles[row][col].render(*el, buf);
            }
        }
    }
}
