use enumset::EnumSet;
use crate::repr::Letter;

#[derive(Debug, Default, Clone)]
pub struct LargeTrieNode {
    children: Vec<(Letter, LargeTrieNode)>,
    is_end_of_word: bool,
}

impl LargeTrieNode {
    pub fn insert_word(&mut self, word: &str) {
        let mut current = self;
        for c in word.chars() {
            let letter = Letter::try_from(c).unwrap();
            let (_, next) = match current.children.iter().position(|(l, _)| *l == letter) {
                Some(next) => &mut current.children[next],
                None => {
                    current.children.push((letter, LargeTrieNode::default()));
                    current.children.last_mut().unwrap()
                }
            };
            current = next;
        }
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
}
