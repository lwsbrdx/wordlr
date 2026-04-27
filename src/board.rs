use ratatui::{
    layout::{Constraint, Flex, Layout},
    widgets::{Paragraph, Widget},
};

use crate::tile::Tile;

#[derive(Debug)]
pub struct Board;
impl Board {
    pub(crate) fn new() -> Self {
        Self
    }
}

pub(crate) struct BoardState {
    tiles: [[Tile; 5]; 6], // 6 lignes, 5 colonnes
    current_row: usize,
    current_col: usize,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl Widget for &Board {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let lines = Layout::vertical([Constraint::Length(4); 6])
            .flex(Flex::Center)
            .split(area);

        lines.iter().for_each(|line| {
            let line_layout = Layout::horizontal([Constraint::Length(8); 5])
                .flex(Flex::Center)
                .split(*line);

            line_layout.iter().for_each(|el| {
                Tile::default().render(*el, buf);
            });
        });
    }
}
