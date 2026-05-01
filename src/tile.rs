use ratatui::{
    style::{Color, Style},
    symbols,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub letter: Option<char>,
    pub state: TileState,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum TileState {
    #[default]
    Empty,
    Typing,
    Typed,
    Highlighted,
    Absent,
    Present,
    Correct,
}

impl Widget for &Tile {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        fn get_matching_color(state: TileState) -> Color {
            match state {
                TileState::Correct => Color::Green,
                TileState::Present => Color::Rgb(205, 135, 41),
                TileState::Absent => Color::DarkGray,
                TileState::Highlighted => Color::LightRed,
                _ => Color::Reset,
            }
        }

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
