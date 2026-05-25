#[derive(Debug, Clone)]
pub enum Direction {
    Previous,
    Next,
}

#[derive(Debug, Clone)]
pub enum GameEvent {
    LetterTyped(char),
    WordSubmitted,
    LetterDeleted,
    EnterInsertMode,
    ExitInsertMode,
    StatsToggled,
    HelpToggled,
    DateChanged(Direction),
    DismissError,
    Quit,
}
