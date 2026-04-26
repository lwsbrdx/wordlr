mod app;
mod menu;
mod status_bar;

use crate::app::App;

fn main() -> std::io::Result<()> {
    let mut app = App::new();
    ratatui::run(|terminal| app.run(terminal))
}
