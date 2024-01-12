use enumset::EnumSet;
use crate::repr::Letter;
use super::large::LargeTrieNode;

#[derive(Debug, Default)]
struct TrieNode {
    children: u32,
    letters: EnumSet<Letter>,
    is_end_of_word: bool,
    max_score: u16,
}

pub struct TrieArena(Vec<TrieNode>);

impl TrieArena {
    pub fn new<'w>(words: impl Iterator<Item=&'w str>) -> Self {
        let mut root = LargeTrieNode::default();
        for word in words {
            root.insert_word(word);
        }

        let mut arena = vec![TrieNode::default()];
        build_arena(&mut arena, 0, root);

        Self(arena)
    }

    pub fn root(&self) -> TrieRef {
        TrieRef { arena: &self.0, node: &self.0[0] }
    }
}

fn build_arena(arena: &mut Vec<TrieNode>, index: usize, large: LargeTrieNode) {
    let children = arena.len() as u32;

    let node = &mut arena[index];
    node.children = children;
    node.letters = large.letters();
    node.is_end_of_word = large.is_end_of_word();
    node.max_score = large.max_score();
    
    arena.extend((0..large.letters().len()).map(|_| TrieNode::default()));
    for (child, (_, large)) in large.into_children().enumerate() {
        build_arena(arena, children as usize + child, large);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TrieRef<'a> {
    arena: &'a [TrieNode],
    node: &'a TrieNode,
}

impl<'a> TrieRef<'a> {
    pub fn child(&self, letter: Letter) -> Option<TrieRef<'a>> {
        match self.node.letters.contains(letter) {
            true => {
                let before_mask = (1 << letter as u8) - 1;
                let before = self.node.letters.as_repr() & before_mask;
                let index = before.count_ones() as usize;
                Some(TrieRef {
                    arena: self.arena,
                    node: &self.arena[self.node.children as usize + index],
                })
            }
            false => None,
        }
    }

    pub fn children(&self) -> impl Iterator<Item=(Letter, TrieRef<'a>)> {
        self.node.letters.iter().enumerate().map(|(i, letter)| {
            (letter, TrieRef {
                arena: self.arena,
                node: &self.arena[self.node.children as usize + i],
            })
        })
    }

    pub fn is_end_of_word(&self) -> bool {
        self.node.is_end_of_word
    }

    pub fn max_score(&self) -> u16 {
        self.node.max_score
    }
}
