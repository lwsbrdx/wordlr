use ratatui::{
    layout::{Constraint, Layout},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Gauge, Paragraph, Widget},
};

use crate::{app::Endings, game::game_stats::GamesStats};

pub(crate) struct Popup {
    ending: Endings,
    games_stats: GamesStats,
}

impl Popup {
    pub fn new(ending: Endings, games_stats: GamesStats) -> Self {
        Self {
            ending,
            games_stats,
        }
    }
}

impl Widget for &Popup {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let outer_block = Block::bordered()
            .border_type(BorderType::Rounded)
            .border_style(Style::new().green())
            .style(Style::default().bg(Color::Black));
        let inner_block = outer_block.inner(area);
        let [top, mid, bottom] = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(12),
            Constraint::Length(2),
        ])
        .areas(inner_block);

        outer_block.render(area, buf);

        self.draw_stats(top, buf);
        self.draw_performances(mid, buf);
        Paragraph::new("test").centered().render(bottom, buf);
    }
}

impl Popup {
    fn draw_stats(&self, top_area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let [title_layout, stats_layout] =
            Layout::vertical([Constraint::Length(1), Constraint::Length(7)]).areas(top_area);

        let stat_block = Block::bordered().border_type(BorderType::Rounded);

        Line::from("Statistiques")
            .left_aligned()
            .render(title_layout, buf);

        let [
            top_stats_games,
            top_stats_victories_ratio,
            top_stats_victories_serie,
            top_stats_best_serie,
        ] = Layout::horizontal([Constraint::Length(16); 4])
            .flex(ratatui::layout::Flex::SpaceEvenly)
            .areas(stats_layout);

        let num_games = self.games_stats.get_total_games();
        Paragraph::new(vec![
            Line::from(format!("{num_games}")),
            Line::from(""),
            Line::from("Parties"),
        ])
        .centered()
        .block(stat_block.clone())
        .render(top_stats_games, buf);

        let win_rate = self.games_stats.get_win_rate() * 100.0;
        Paragraph::new(vec![
            Line::from(format!("{win_rate}")),
            Line::from(""),
            Line::from("Victoire (%)"),
        ])
        .centered()
        .block(stat_block.clone())
        .render(top_stats_victories_ratio, buf);

        let actual_serie = 4; // TODO
        Paragraph::new(vec![
            Line::from(format!("{actual_serie}")),
            Line::from("Série"),
            Line::from("Actuelle"),
        ])
        .centered()
        .block(stat_block.clone())
        .render(top_stats_victories_serie, buf);

        let best_serie = 4; // TODO
        Paragraph::new(vec![
            Line::from(format!("{best_serie}")),
            Line::from("Meilleurs"),
            Line::from("Série"),
        ])
        .centered()
        .block(stat_block.clone())
        .render(top_stats_best_serie, buf);
    }

    fn draw_performances(
        &self,
        mid_area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
        let [title_layout, performances_layout] =
            Layout::vertical([Constraint::Length(2), Constraint::Length(14)]).areas(mid_area);

        Line::from("Performances")
            .left_aligned()
            .render(title_layout, buf);

        let performances_layouts = Layout::vertical([Constraint::Length(1); 13])
            .spacing(1)
            .split(performances_layout);

        performances_layouts
            .iter()
            .enumerate()
            .for_each(|(index, l)| {
                let tx = index;
                let s = Span::from(format!("{tx}"));
                let g = Gauge::default().style(Style::new().dark_gray()).percent(10);

                let [attemps_number, stat_gauge] =
                    Layout::horizontal([Constraint::Fill(1), Constraint::Percentage(80)]).areas(*l);

                s.render(attemps_number, buf);
                g.render(stat_gauge, buf);
            });
    }
}
