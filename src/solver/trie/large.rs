use enumset::EnumSet;
use crate::repr::Letter;
use super::super::score::{letter_score, long_word_bonus};

#[derive(Debug, Default, Clone)]
pub struct LargeTrieNode {
    children: Vec<(Letter, LargeTrieNode)>,
    is_end_of_word: bool,
    max_score: u16,
}

impl LargeTrieNode {
    pub fn insert_word(&mut self, word: &str) {
        let max_score = max_score(word);
        let mut current = self;
        for c in word.chars() {
            let letter = Letter::try_from(c).unwrap();
            current.max_score = current.max_score.max(max_score);
            let (_, next) = match current.children.iter().position(|(l, _)| *l == letter) {
                Some(next) => &mut current.children[next],
                None => {
                    current.children.push((letter, LargeTrieNode::default()));
                    current.children.last_mut().unwrap()
                }
            };
            current = next;
        }
        current.max_score = current.max_score.max(max_score);
        current.is_end_of_word = true;
    }

    pub fn into_children(mut self) -> impl Iterator<Item=(Letter, LargeTrieNode)> {
        self.children.sort_by_key(|(l, _)| *l);
        self.children.into_iter()
    }

    pub fn letters(&self) -> EnumSet<Letter> {
        self.children.iter().map(|(l, _)| *l).collect()
    }

    pub fn is_end_of_word(&self) -> bool {
        self.is_end_of_word
    }

    pub fn max_score(&self) -> u16 {
        self.max_score
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
