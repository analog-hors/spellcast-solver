use super::game::{letter_score, long_word_bonus};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Letter {
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,
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
    pub children: Vec<(Letter, TrieNode)>,
    pub is_end_of_word: bool,
    pub max_score: u32,
}

pub fn make_word_trie<'w>(words: impl Iterator<Item=&'w str>) -> TrieNode {
    let mut root = TrieNode::default();
    for word in words {
        let max_score = max_score(word);
        let mut current = &mut root;
        for c in word.chars() {
            let letter = Letter::try_from(c).unwrap();
            current.max_score = current.max_score.max(max_score);
            let (_, next) = match current.children.iter().position(|(l, _)| *l == letter) {
                Some(next) => &mut current.children[next],
                None => {
                    current.children.push((letter, TrieNode::default()));
                    current.children.last_mut().unwrap()
                }
            };
            current = next;
        }
        current.max_score = current.max_score.max(max_score);
        current.is_end_of_word = true;
    }
    root
}

fn max_score(word: &str) -> u32 {
    let mut word_score = 0;
    let mut max_letter_score = 0;
    for c in word.chars() {
        let letter = Letter::try_from(c).unwrap();
        word_score += letter_score(letter);
        max_letter_score = max_letter_score.max(letter_score(letter));
    }
    word_score += max_letter_score;    
    word_score * 2 + long_word_bonus(word.len())
}
