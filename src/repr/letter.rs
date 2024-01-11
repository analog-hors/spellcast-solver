use enumset::EnumSetType;

#[derive(Debug, EnumSetType, PartialOrd, Ord, Hash)]
pub enum Letter {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

impl From<Letter> for char {
    fn from(value: Letter) -> Self {
        (value as u8 + b'A') as char
    }
}

impl TryFrom<char> for Letter {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value.to_ascii_uppercase() {
            'A' => Letter::A,
            'B' => Letter::B,
            'C' => Letter::C,
            'D' => Letter::D,
            'E' => Letter::E,
            'F' => Letter::F,
            'G' => Letter::G,
            'H' => Letter::H,
            'I' => Letter::I,
            'J' => Letter::J,
            'K' => Letter::K,
            'L' => Letter::L,
            'M' => Letter::M,
            'N' => Letter::N,
            'O' => Letter::O,
            'P' => Letter::P,
            'Q' => Letter::Q,
            'R' => Letter::R,
            'S' => Letter::S,
            'T' => Letter::T,
            'U' => Letter::U,
            'V' => Letter::V,
            'W' => Letter::W,
            'X' => Letter::X,
            'Y' => Letter::Y,
            'Z' => Letter::Z,
            _ => return Err(()),
        })
    }
}
