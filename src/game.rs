use super::words::Letter;

#[derive(Debug, Clone)]
pub struct Game {
    pub grid: [[Letter; Game::WIDTH]; Game::HEIGHT],
    pub double_word: Option<(i8, i8)>,
    pub double_letter: Option<(i8, i8)>,
}

impl Game {
    pub const WIDTH: usize = 5;
    pub const HEIGHT: usize = 5;
}
