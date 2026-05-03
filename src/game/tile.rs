#[derive(Debug, Default, Clone, Copy)]
pub struct Tile {
    pub letter: Option<char>,
    pub state: TileState,
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub enum TileState {
    #[default]
    Empty,
    Typing,
    Typed,
    Highlighted,
    Absent,
    Present,
    Correct,
}
