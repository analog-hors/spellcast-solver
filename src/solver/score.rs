use crate::repr::Letter;

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
