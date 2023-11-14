#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Letter {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
}

impl Letter {
    pub const ALL: [Letter; 26] = [
        Letter::A,
        Letter::B,
        Letter::C,
        Letter::D,
        Letter::E,
        Letter::F,
        Letter::G,
        Letter::H,
        Letter::I,
        Letter::J,
        Letter::K,
        Letter::L,
        Letter::M,
        Letter::N,
        Letter::O,
        Letter::P,
        Letter::Q,
        Letter::R,
        Letter::S,
        Letter::T,
        Letter::U,
        Letter::V,
        Letter::W,
        Letter::X,
        Letter::Y,
        Letter::Z,
    ];
}

impl From<Letter> for char {
    fn from(value: Letter) -> Self {
        (value as u8 + 'A' as u8) as char
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

#[derive(Debug, Default, Clone)]
pub struct TrieNode {
    pub next: Box<[Option<TrieNode>; 26]>,
    pub is_end_of_word: bool,
}

pub fn make_word_trie<'w>(words: impl Iterator<Item=&'w str>) -> TrieNode {
    let mut root = TrieNode::default();
    for word in words {
        let mut current = &mut root;
        for c in word.chars() {
            let letter = Letter::try_from(c).unwrap();
            let next = &mut current.next[letter as usize];
            if next.is_none() {
                *next = Some(TrieNode::default());
            }
            current = next.as_mut().unwrap();
        }
        current.is_end_of_word = true;
    }
    root
}
