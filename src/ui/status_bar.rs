use ratatui::{
    layout::{Constraint, Flex, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Padding, Paragraph, Widget},
};

use crate::app::InputModes;

#[derive(Debug)]
pub struct StatusBar<'a> {
    current_mode: &'a InputModes,
}

impl<'a> StatusBar<'a> {
    pub fn new(mode: &'a InputModes) -> Self {
        Self { current_mode: mode }
    }
}

impl<'a> Widget for &StatusBar<'a> {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let [left, center] = Layout::horizontal([Constraint::Length(10), Constraint::Length(35)])
            .flex(Flex::Start)
            .areas(area);

        let current_mode_block = Block::new()
            .padding(Padding::left(1))
            .style(Style::new().bg(match self.current_mode {
                InputModes::Normal => Color::DarkGray,
                InputModes::Insert => Color::Green,
            }));

        Paragraph::new(Text::styled(
            self.current_mode.to_string(),
            Style::default().bold(),
        ))
        .centered()
        .block(current_mode_block)
        .render(left, buf);

        let help_block = Block::new().padding(Padding::left(1));
        Paragraph::new(Text::styled(
            match self.current_mode {
                InputModes::Normal => "Press i to enter Insert mode",
                InputModes::Insert => "Press Esc to enter Normal mode",
            },
            Style::default().fg(Color::DarkGray),
        ))
        .block(help_block)
        .render(center, buf);
    }
}
