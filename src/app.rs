use std::io::Result;

use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Flex, Layout},
    style::{Style, Stylize},
    text::Line,
    widgets::Paragraph,
};

use crate::{
    app_state::AppState,
    helpers,
    input_handler::InputHandler,
    session::{GameSession, SessionSignal},
    ui::{board::Board, help::Help, menu::Menu, popup::Popup, status_bar::StatusBar},
};

const MIN_WIDTH: u16 = 52;
const MIN_HEIGHT: u16 = 24;

#[derive(Debug)]
pub struct App {
    menu: Menu,
    session: GameSession,
    state: AppState,
    exit: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            menu: Menu,
            session: GameSession::new()?,
            state: AppState::Playing,
            exit: false,
        })
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        let signals = self.session.tick();
        self.apply_signals(signals);

        let events = InputHandler::poll(&self.state, &self.session.input_mode)?;
        let signals = self.session.update(&events)?;
        self.apply_signals(signals);

        Ok(())
    }

    fn apply_signals(&mut self, signals: Vec<SessionSignal>) {
        for signal in signals {
            match signal {
                SessionSignal::Quit => self.exit = true,
                SessionSignal::OpenStats => self.state = AppState::ViewingStats,
                SessionSignal::ToggleStats => {
                    self.state = match self.state {
                        AppState::Playing => AppState::ViewingStats,
                        AppState::ViewingStats => AppState::Playing,
                        _ => return,
                    }
                }
                SessionSignal::ToggleHelp => {
                    self.state = match self.state {
                        AppState::Playing => AppState::ViewingHelp,
                        AppState::ViewingHelp => AppState::Playing,
                        _ => return,
                    }
                }
                SessionSignal::DismissError => self.state = AppState::Playing,
                SessionSignal::SetError(e) => self.state = AppState::Error(e),
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let area = frame.area();

        if area.width < MIN_WIDTH || area.height < MIN_HEIGHT {
            let msg = Paragraph::new(format!(
                "Terminal trop petit\n{}×{} minimum (actuel : {}×{})",
                MIN_WIDTH, MIN_HEIGHT, area.width, area.height
            ))
            .centered()
            .style(Style::new().dark_gray());
            frame.render_widget(msg, area);
            return;
        }

        let [top, mid, bottom] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(50),
            Constraint::Length(1),
        ])
        .flex(Flex::SpaceBetween)
        .areas(area);

        frame.render_widget(&self.menu, top);

        let [date_layout, board_layout] =
            Layout::vertical([Constraint::Length(3), Constraint::Length(47)]).areas(mid);

        let date_line =
            Line::from(format!("{}", self.session.selected_date.format("%A %d %B %Y")))
                .bold()
                .centered();

        frame.render_widget(date_line, date_layout);
        frame.render_stateful_widget(&Board, board_layout, &mut self.session.board_state);
        frame.render_widget(&StatusBar::new(&self.session.input_mode), bottom);

        match &self.state {
            AppState::ViewingStats => {
                let popup_area = helpers::centered_rect(50, 65, frame.area());
                frame.render_widget(ratatui::widgets::Clear, popup_area);
                frame.render_widget(
                    &Popup::new(&self.session.games_stats, self.session.selected_date),
                    popup_area,
                );
            }
            AppState::ViewingHelp => {
                let help_area = helpers::centered_rect(60, 65, frame.area());
                frame.render_widget(ratatui::widgets::Clear, help_area);
                frame.render_widget(&Help, help_area);
            }
            AppState::Error(msg) => {
                use ratatui::{
                    style::Stylize,
                    text::Line,
                    widgets::{Block, BorderType, Paragraph},
                };
                let error_area = helpers::centered_rect(50, 20, frame.area());
                frame.render_widget(ratatui::widgets::Clear, error_area);
                frame.render_widget(
                    Paragraph::new(vec![
                        Line::from(msg.as_str()),
                        Line::from(""),
                        Line::from("Appuyez sur Esc pour fermer").dark_gray(),
                    ])
                    .centered()
                    .block(
                        Block::bordered()
                            .border_type(BorderType::Rounded)
                            .border_style(ratatui::style::Style::new().red())
                            .title(" Erreur "),
                    ),
                    error_area,
                );
            }
            AppState::Playing => {}
        }
    }
}
