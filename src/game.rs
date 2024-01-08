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

pub const fn letter_score(letter: Letter) -> u16 {
    use Letter::*;

    match letter {
        A | E | I | O => 1,
        N | R | S | T => 2,
        D | G | L => 3,
        B | H | M | P | U | Y => 4,
        C | F | V | W => 5,
        K => 6,
        J | X => 7,
        Q | Z => 8,
    }
}

pub const fn long_word_bonus(length: usize) -> u16 {
    if length >= 6 { 10 } else { 0 }
}
