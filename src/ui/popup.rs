use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Gauge, Padding, Paragraph, Widget},
};

use crate::game::game_stats::GamesStats;

pub(crate) struct Popup {
    games_stats: GamesStats,
}

impl Popup {
    pub fn new(games_stats: GamesStats) -> Self {
        Self { games_stats }
    }
}

impl Widget for &Popup {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let outer_block = Block::bordered()
            .padding(Padding::new(2, 2, 1, 1))
            .border_type(BorderType::Rounded)
            .border_style(match self.games_stats.current_game.ending {
                Some(crate::app::Endings::Victory) => Style::new().green(),
                Some(crate::app::Endings::Loss) => Style::new().red(),
                _ => Style::new().white(),
            });
        let inner_block = outer_block.inner(area);
        let [top, mid, bottom] = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(18),
            Constraint::Length(4),
        ])
        .flex(ratatui::layout::Flex::SpaceBetween)
        .areas(inner_block);

        outer_block.render(area, buf);

        self.draw_stats(top, buf);
        self.draw_performances(mid, buf);

        let secret_word = &self.games_stats.current_game.secret_word;
        Paragraph::new(vec![
            Line::from("Le mot était"),
            Line::from(secret_word.to_string()).bold(),
            Line::from(""),
            Line::from("☕ https://buymeacoffee.com/lwsbrdx").yellow(),
        ])
        .centered()
        .render(bottom, buf);
    }
}

impl Popup {
    fn draw_stats(&self, top_area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let [title_layout, stats_layout] =
            Layout::vertical([Constraint::Length(2), Constraint::Length(5)]).areas(top_area);

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
            Line::from(format!("{win_rate:.2}")),
            Line::from(""),
            Line::from("Victoire (%)"),
        ])
        .centered()
        .block(stat_block.clone())
        .render(top_stats_victories_ratio, buf);

        let actual_serie = self.games_stats.get_actual_serie();
        Paragraph::new(vec![
            Line::from(format!("{actual_serie}")),
            Line::from("Série"),
            Line::from("Actuelle"),
        ])
        .centered()
        .block(stat_block.clone())
        .render(top_stats_victories_serie, buf);

        let best_serie = self.games_stats.get_best_serie();
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
            Layout::vertical([Constraint::Length(2), Constraint::Length(12)]).areas(mid_area);

        Line::from("Performances")
            .left_aligned()
            .render(title_layout, buf);

        let performances_layouts =
            Layout::vertical([Constraint::Length(1); 12]).split(performances_layout);

        let all_games_count = self.games_stats.get_total_games();
        let current_game_attempts = self.games_stats.current_game.attempts.len();

        performances_layouts
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .for_each(|(index, rect)| {
                let tx = index / 2 + 1;
                let s = Paragraph::new(format!("{tx}")).centered();
                let games_w_attempts = self.games_stats.get_games_by_attempts_count(tx).len();
                let l = Paragraph::new(format!("{games_w_attempts}")).centered();

                let percentage =
                    (games_w_attempts as f32 / all_games_count as f32 * 100.0).max(2.0);
                let g = Gauge::default()
                    .gauge_style(if tx == current_game_attempts {
                        Style::new().green()
                    } else {
                        Style::new().dark_gray()
                    })
                    .percent(percentage as u16)
                    .label("");
                // .label(format!("{percentage}"));

                let [attempts_number, stat_gauge, gauge_label] = Layout::horizontal([
                    Constraint::Length(3),
                    Constraint::Length(40),
                    Constraint::Length(5),
                ])
                .areas(*rect);

                s.render(attempts_number, buf);
                g.render(stat_gauge, buf);
                l.render(gauge_label, buf);
            });
    }
}
