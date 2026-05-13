use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::{Line, Span},
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

use crate::game::{board::{MAX_COLS, MAX_LINES}, tile::{Tile, TileState}};

pub(crate) struct Help;

impl Widget for &Help {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let outer_block = Block::bordered()
            .padding(Padding::new(2, 2, 1, 1))
            .border_type(BorderType::Rounded)
            .border_style(Style::new().white());
        let inner = outer_block.inner(area);
        outer_block.render(area, buf);

        let [
            title,
            desc,
            ex1,
            ex1_label,
            ex2,
            ex2_label,
            ex3,
            ex3_label,
            shortcuts,
            footer,
        ] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Length(3),
            Constraint::Length(2),
            Constraint::Fill(1),
            Constraint::Length(1),
        ])
        .areas(inner);

        Line::from("Comment jouer ?").bold().render(title, buf);

        Paragraph::new(vec![
            Line::from(format!("Devinez le WORDLR en {} essais. Chaque essai doit être", MAX_LINES)),
            Line::from(format!("un mot valide de {} lettres. La couleur des tuiles change", MAX_COLS)),
            Line::from("pour indiquer à quel point votre essai est proche."),
        ])
        .render(desc, buf);

        render_example_row(
            ex1,
            buf,
            [
                ('F', TileState::Correct),
                ('R', TileState::Empty),
                ('U', TileState::Empty),
                ('I', TileState::Empty),
                ('T', TileState::Empty),
            ],
        );
        Paragraph::new(vec![Line::from(vec![
            Span::from("La lettre "),
            Span::from("F").green().bold(),
            Span::from(" est dans le mot, à la bonne place."),
        ])])
        .render(ex1_label, buf);

        render_example_row(
            ex2,
            buf,
            [
                ('P', TileState::Empty),
                ('O', TileState::Empty),
                ('C', TileState::Present),
                ('H', TileState::Empty),
                ('E', TileState::Empty),
            ],
        );
        Paragraph::new(vec![Line::from(vec![
            Span::from("La lettre "),
            Span::from("C")
                .fg(ratatui::style::Color::Rgb(205, 135, 41))
                .bold(),
            Span::from(" est dans le mot, mais pas à la bonne place."),
        ])])
        .render(ex2_label, buf);

        render_example_row(
            ex3,
            buf,
            [
                ('S', TileState::Empty),
                ('O', TileState::Empty),
                ('E', TileState::Empty),
                ('U', TileState::Empty),
                ('R', TileState::Absent),
            ],
        );
        Paragraph::new(vec![Line::from(vec![
            Span::from("La lettre "),
            Span::from("R").dark_gray().bold(),
            Span::from(" n'est pas dans le mot."),
        ])])
        .render(ex3_label, buf);

        let shortcut = |key: &'static str, desc: &'static str| {
            Line::from(vec![
                Span::from(key).bold(),
                Span::from(format!("  {desc}")),
            ])
        };

        Paragraph::new(vec![
            Line::from("Raccourcis").bold(),
            shortcut("i", "Saisir un mot"),
            shortcut("Esc", "Mode normal"),
            shortcut("h / ←", "Date précédente"),
            shortcut("l / →", "Date suivante"),
            shortcut("s", "Statistiques"),
            shortcut("?", "Cette aide"),
            shortcut("q", "Quitter"),
        ])
        .render(shortcuts, buf);

        Line::from("Esc pour fermer")
            .dark_gray()
            .right_aligned()
            .render(footer, buf);
    }
}

fn render_example_row(
    area: ratatui::prelude::Rect,
    buf: &mut ratatui::prelude::Buffer,
    letters: [(char, TileState); 5],
) {
    let cols = Layout::horizontal([Constraint::Length(5); 5])
        .flex(ratatui::layout::Flex::Start)
        .split(area);

    for (i, (letter, state)) in letters.iter().enumerate() {
        let tile = Tile {
            letter: Some(*letter),
            state: *state,
        };
        tile.render(cols[i], buf);
    }
}
