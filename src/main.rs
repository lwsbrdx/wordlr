mod app;
mod game;
mod helpers;
mod ui;

use crate::app::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))
}
