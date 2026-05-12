use ratatui::
    widgets::{Block, Padding, Paragraph, Widget}
;

#[derive(Debug)]
pub struct Menu;

impl Menu {
    pub fn new() -> Self {
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
        let menu_block = Block::default().padding(Padding::horizontal(2));

        Paragraph::new("Quitter (q) | Aide (?) | Statistiques (s)")
            .block(menu_block)
            .render(area, buf);
    }
}
