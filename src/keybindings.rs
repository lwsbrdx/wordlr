pub const QUIT: char = 'q';
pub const INSERT_MODE: char = 'i';
pub const STATS: char = 's';
pub const HELP: char = '?';
pub const DATE_PREV: char = 'h';
pub const DATE_NEXT: char = 'l';

pub const SHORTCUTS: &[(&str, &str)] = &[
    ("i", "Saisir un mot"),
    ("Esc", "Mode normal"),
    ("h / ←", "Date précédente"),
    ("l / →", "Date suivante"),
    ("s", "Statistiques"),
    ("?", "Cette aide"),
    ("q", "Quitter"),
];
