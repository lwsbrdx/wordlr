use chrono::NaiveDate;
use ratatui::{
    layout::{Constraint, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, BorderType, Gauge, Padding, Paragraph, Widget},
};

use crate::game::{
    endings::Endings,
    game_stats::{GameStats, GamesStats},
};

pub(crate) struct Popup {
    games_stats: GamesStats,
    date: NaiveDate,
}

impl Widget for &Popup {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let current_game = self.games_stats.current_game(self.date);
        let current_ending = current_game.and_then(|g| g.ending);

        let outer_block = Block::bordered()
            .padding(Padding::new(2, 2, 1, 1))
            .border_type(BorderType::Rounded)
            .border_style(match current_ending {
                Some(Endings::Victory) => Style::new().light_green(),
                Some(Endings::Loss) => Style::new().red(),
                _ => Style::new().white(),
            });
        let inner_block = outer_block.inner(area);
        let [title, top, mid, bottom] = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(8),
            Constraint::Length(18),
            Constraint::Length(4),
        ])
        .flex(ratatui::layout::Flex::SpaceBetween)
        .areas(inner_block);

        outer_block.render(area, buf);

        Paragraph::new(format!("{}", self.date.format("%A %d %B %Y")))
            .centered()
            .render(title, buf);

        self.draw_stats(top, buf);
        self.draw_performances(current_game, mid, buf);
        self.draw_bottom(current_game, bottom, buf);
    }
}

impl Popup {
    pub fn new(games_stats: GamesStats, date: NaiveDate) -> Self {
        Self { games_stats, date }
    }

    fn draw_stats(&self, top_area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let [title_layout, stats_layout] =
            Layout::vertical([Constraint::Length(2), Constraint::Length(5)]).areas(top_area);

        fn stat(lines: Vec<Line<'_>>) -> Paragraph<'_> {
            Paragraph::new(lines)
                .centered()
                .block(Block::bordered().border_type(BorderType::Rounded))
        }

        Line::from("Statistiques")
            .bold()
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
        stat(vec![
            Line::from(format!("{num_games}")),
            Line::from(""),
            Line::from("Parties"),
        ])
        .render(top_stats_games, buf);

        let win_rate = self.games_stats.get_win_rate() * 100.0;
        stat(vec![
            Line::from(format!("{win_rate:.2}")),
            Line::from(""),
            Line::from("Victoire (%)"),
        ])
        .render(top_stats_victories_ratio, buf);

        let actual_serie = self.games_stats.get_actual_serie();
        stat(vec![
            Line::from(format!("{actual_serie}")),
            Line::from("Série"),
            Line::from("Actuelle"),
        ])
        .render(top_stats_victories_serie, buf);

        let best_serie = self.games_stats.get_best_serie();
        stat(vec![
            Line::from(format!("{best_serie}")),
            Line::from("Meilleurs"),
            Line::from("Série"),
        ])
        .render(top_stats_best_serie, buf);
    }

    fn draw_performances(
        &self,
        current_game: Option<&GameStats>,
        mid_area: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
        let [title_layout, performances_layout] =
            Layout::vertical([Constraint::Length(2), Constraint::Length(14)]).areas(mid_area);

        Line::from("Performances")
            .left_aligned()
            .render(title_layout, buf);

        let performances_layouts =
            Layout::vertical([Constraint::Length(1); 14]).split(performances_layout);

        let all_games_count = self.games_stats.get_total_games();
        let current_game_attempts = current_game.map(|g| g.attempts.len()).unwrap_or(0);

        let current_ending = current_game.and_then(|g| g.ending);
        let losses_count = self.games_stats.get_losses().len();

        performances_layouts
            .iter()
            .enumerate()
            .filter(|(i, _)| i % 2 == 0)
            .for_each(|(index, rect)| {
                let current_line = index / 2 + 1;
                let victories_for_attempts = self
                    .games_stats
                    .get_victories_by_attempts_count(current_line)
                    .len();
                let max_lines = performances_layouts.len() / 2;

                let (prefix, percentage, suffix) = self.get_stats_for_attempts(
                    current_line,
                    max_lines,
                    losses_count,
                    victories_for_attempts,
                    all_games_count as f32,
                );

                let prefix_paragraph = Paragraph::new(prefix).centered();

                let color = match current_ending {
                    Some(Endings::Victory) if current_line == current_game_attempts => {
                        Style::new().light_green()
                    }
                    Some(Endings::Loss) if current_line == max_lines => Style::new().red(),
                    _ => Style::new().dark_gray(),
                };

                let gauge = Gauge::default()
                    .gauge_style(color)
                    .percent(percentage as u16)
                    .label("");

                let suffix_paragraph = Paragraph::new(suffix).style(color).centered();

                let [attempts_number, stat_gauge, gauge_label] = Layout::horizontal([
                    Constraint::Length(3),
                    Constraint::Length(40),
                    Constraint::Length(5),
                ])
                .areas(*rect);

                prefix_paragraph.render(attempts_number, buf);
                gauge.render(stat_gauge, buf);
                suffix_paragraph.render(gauge_label, buf);
            });
    }

    fn draw_bottom(
        &self,
        current_game: Option<&GameStats>,
        bottom: ratatui::prelude::Rect,
        buf: &mut ratatui::prelude::Buffer,
    ) {
        let current_ending = current_game.and_then(|g| g.ending);
        let secret_word = current_game
            .map(|g| g.secret_word.clone())
            .unwrap_or_default();

        let empty_line = Line::from("");
        let buy_me_a_coffe_line = Line::from("☕ https://buymeacoffee.com/lwsbrdx").yellow();

        let lines = if current_ending.is_some() {
            vec![
                Line::from("Le mot était"),
                Line::from(secret_word).bold(),
                empty_line,
                buy_me_a_coffe_line,
            ]
        } else {
            vec![
                Line::from("Oh! Mais tu n'as pas fini!"),
                Line::from("*****").bold(),
                empty_line,
                buy_me_a_coffe_line,
            ]
        };

        Paragraph::new(lines).centered().render(bottom, buf);
    }

    fn get_stats_for_attempts(
        &self,
        current_line: usize,
        max_lines: usize,
        losses_count: usize,
        victories_for_attempts: usize,
        all_games_count: f32,
    ) -> (String, f32, String) {
        let is_last = current_line == max_lines;

        let prefix: String;
        let dividend: f32;
        let suffix: String;

        if is_last {
            prefix = "D".to_owned();
            dividend = losses_count as f32;
            suffix = losses_count.to_string();
        } else {
            prefix = format!("{current_line}");
            dividend = victories_for_attempts as f32;
            suffix = victories_for_attempts.to_string();
        }

        let percentage = (dividend / all_games_count * 100.0).max(2.0);
        (prefix, percentage, suffix)
    }
}
