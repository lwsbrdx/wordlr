use ratatui::{
    symbols,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct Tile {
    letter: Option<char>,
    state: TileState,
}

impl Tile {
    pub(crate) fn new(letter: Option<char>, state: TileState) -> Self {
        Self { letter, state }
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub(crate) enum TileState {
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
        let block = Block::bordered().border_set(symbols::border::ROUNDED);

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
