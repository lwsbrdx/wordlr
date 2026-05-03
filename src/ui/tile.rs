use ratatui::{
    style::{Color, Style},
    symbols,
    widgets::{Block, Paragraph, Widget},
};

use crate::game::tile::{Tile, TileState};

fn get_matching_color(state: TileState) -> Color {
    match state {
        TileState::Correct => Color::Green,
        TileState::Present => Color::Rgb(205, 135, 41),
        TileState::Absent => Color::DarkGray,
        TileState::Highlighted => Color::LightRed,
        _ => Color::Reset,
    }
}

impl Widget for &Tile {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered()
            .border_set(match self.state {
                TileState::Typing => symbols::border::DOUBLE,
                _ => symbols::border::PLAIN,
            })
            .border_style(Style::default().fg(get_matching_color(self.state)));

        Paragraph::new(if let Some(l) = self.letter {
            l.to_string()
        } else {
            "".to_string()
        })
        .centered()
        .style(Style::default().fg(get_matching_color(self.state)))
        .block(block)
        .render(area, buf);
    }
}
