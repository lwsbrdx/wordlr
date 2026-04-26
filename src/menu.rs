use ratatui::widgets::{Block, Padding, Paragraph, Widget};

#[derive(Debug)]
pub struct Menu;
impl Menu {
    pub(crate) fn new() -> Self {
        Self
    }
}

impl Default for Menu {
    fn default() -> Self {
        Self::new()
    }
}
impl Widget for &Menu {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        // let [zone] = Layout::horizontal([Constraint::Length(10)]).areas(area);
        let menu_block = Block::default().padding(Padding {
            top: 1,
            right: 0,
            bottom: 0,
            left: 1,
        });

        Paragraph::new("Help (?)").block(menu_block).render(area, buf);
    }
}
