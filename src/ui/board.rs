use ratatui::{
    layout::{Constraint, Flex, Layout},
    prelude::{Buffer, Rect},
    widgets::{StatefulWidget, Widget},
};

use crate::game::board::{BoardState, MAX_COLS, MAX_LINES};

pub struct Board;

impl StatefulWidget for &Board {
    type State = BoardState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut BoardState)
    where
        Self: Sized,
    {
        let lines = Layout::vertical([Constraint::Length(3); MAX_LINES])
            .flex(Flex::Center)
            .split(area);

        for (row, line) in lines.iter().enumerate() {
            let line_layout = Layout::horizontal([Constraint::Length(5); MAX_COLS])
                .flex(Flex::Center)
                .split(*line);

            for (col, el) in line_layout.iter().enumerate() {
                let tile = state.tiles[row][col]; // Tile est Copy

                let render_area = match &state.reveal_animation {
                    Some(anim) if anim.row == row && anim.current_col == col => {
                        let w = anim.current_width().min(el.width);
                        let x_offset = (el.width.saturating_sub(w)) / 2;
                        Rect { x: el.x + x_offset, width: w, ..*el }
                    }
                    _ => *el,
                };

                tile.render(render_area, buf);
            }
        }
    }
}
