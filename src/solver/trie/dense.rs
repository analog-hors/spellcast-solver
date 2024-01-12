use enumset::EnumSet;
use crate::repr::Letter;
use super::super::score::{letter_score, long_word_bonus};

#[derive(Debug, Default, Clone)]
pub struct DenseTrieNode {
    pub children: Box<[Option<DenseTrieNode>; 26]>,
    pub letters: EnumSet<Letter>,
    pub is_end_of_word: bool,
    pub max_score: u16,
}

impl DenseTrieNode {
    pub fn insert_word(&mut self, word: &str) {
        let max_score = max_score(word);
        let mut current = self;
        for c in word.chars() {
            let letter = Letter::try_from(c).unwrap();
            current.max_score = current.max_score.max(max_score);
            let next = &mut current.children[letter as usize];
            if next.is_none() {
                *next = Some(DenseTrieNode::default());
                current.letters.insert(letter);
            }
            current = next.as_mut().unwrap();
        }
        current.max_score = current.max_score.max(max_score);
        current.is_end_of_word = true;
    }
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
