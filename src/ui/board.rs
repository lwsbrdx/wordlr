use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Rect},
    widgets::{StatefulWidget, Widget},
};

use crate::game::board::BoardState;

pub struct Board;

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
