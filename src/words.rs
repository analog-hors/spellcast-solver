use enumset::{EnumSetType, EnumSet};

use super::game::{letter_score, long_word_bonus};

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

#[derive(Debug, Default, Clone)]
pub struct TrieNode {
    children: Box<[Option<TrieNode>; 26]>,
    letters: EnumSet<Letter>,
    is_end_of_word: bool,
    max_score: u16,
}

impl TrieNode {
    pub fn child(&self, letter: Letter) -> Option<&TrieNode> {
        self.children[letter as usize].as_ref()
    }

    pub fn children(&self) -> impl Iterator<Item=(Letter, &TrieNode)> {
        self.letters.iter().map(|l| (l, self.child(l).unwrap()))
    }

    pub fn is_end_of_word(&self) -> bool {
        self.is_end_of_word
    }

    pub fn max_score(&self) -> u16 {
        self.max_score
    }
}

pub fn make_word_trie<'w>(words: impl Iterator<Item=&'w str>) -> TrieNode {
    let mut root = TrieNode::default();
    for word in words {
        let max_score = max_score(word);
        let mut current = &mut root;
        for c in word.chars() {
            let letter = Letter::try_from(c).unwrap();
            current.max_score = current.max_score.max(max_score);
            let next = &mut current.children[letter as usize];
            if next.is_none() {
                *next = Some(TrieNode::default());
                current.letters.insert(letter);
            }
            current = next.as_mut().unwrap();
        }
        current.max_score = current.max_score.max(max_score);
        current.is_end_of_word = true;
    }
    root
}

fn max_score(word: &str) -> u16 {
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
