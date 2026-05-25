mod app;
mod app_state;
mod game;
mod game_event;
mod helpers;
mod input_handler;
mod session;
mod ui;

use crate::app::App;

fn main() {
    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Erreur au démarrage de wordlr : {e}");
            std::process::exit(1);
        }
    };

    if let Err(e) = ratatui::run(|terminal| app.run(terminal)) {
        eprintln!("Erreur fatale : {e}");
        std::process::exit(1);
    }
}
