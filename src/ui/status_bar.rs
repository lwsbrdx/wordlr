use ratatui::{
    layout::{Constraint, Flex, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::app::Modes;

#[derive(Debug)]
pub struct StatusBar<'a> {
    current_mode: &'a Modes,
}

impl<'a> StatusBar<'a> {
    pub fn new(mode: &'a Modes) -> Self {
        Self { current_mode: mode }
    }
}

impl<'a> Widget for &StatusBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [left] = Layout::horizontal([Constraint::Length(10)])
            .flex(Flex::Start)
            .areas(area);

        let current_mode_block = Block::new()
            .padding(Padding::left(1))
            .style(Style::new().bg(match self.current_mode {
                Modes::Normal => Color::DarkGray,
                Modes::Insert => Color::Green,
            }));

        Paragraph::new(Text::styled(
            self.current_mode.to_string(),
            Style::default().bold(),
        ))
        .centered()
        .block(current_mode_block)
        .render(left, buf);
    }
}
