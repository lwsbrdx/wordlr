use ratatui::{
    symbols,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub letter: Option<char>,
    pub state: TileState,
}

#[derive(Debug, Default, Copy, Clone)]
pub enum TileState {
    #[default]
    Empty,
    Typing,
    Absent,
    Present,
    Correct,
}

impl Widget for &Tile {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = Block::bordered().border_set(match self.state {
            TileState::Typing => symbols::border::DOUBLE,
            _ => symbols::border::PLAIN,
        });

        Paragraph::new(if let Some(l) = self.letter {
            l.to_string()
        } else {
            "".to_string()
        })
        .centered()
        .block(block)
        .render(area, buf);
    }
}
