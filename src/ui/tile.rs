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
        TileState::Typed => Color::Blue,
        TileState::Typing => Color::LightBlue,
        _ => Color::Reset,
    }
}

impl Widget for &Tile {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let color = get_matching_color(self.state);

        let block = Block::bordered()
            .border_set(match self.state {
                TileState::Typing => symbols::border::DOUBLE,
                _ => symbols::border::PLAIN,
            })
            .border_style(Style::default().fg(color));

        Paragraph::new(if let Some(l) = self.letter {
            l.to_string()
        } else {
            "".to_string()
        })
        .centered()
        .style(Style::default().fg(color))
        .block(block)
        .render(area, buf);
    }
}
