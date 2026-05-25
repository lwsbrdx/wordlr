#[derive(Debug)]
pub enum AppState {
    Playing,
    ViewingStats,
    ViewingHelp,
    Error(String),
}
