use enumset::EnumSet;
use crate::repr::Letter;
use super::large::LargeTrieNode;

#[derive(Debug, Default)]
struct TrieNode {
    children: u32,
    letters: EnumSet<Letter>,
    is_end_of_word: bool,
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
        TrieRef { arena: &self.0, index: 0 }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

fn build_arena(arena: &mut Vec<TrieNode>, index: usize, large: LargeTrieNode) {
    let children = arena.len() as u32;

    let node = &mut arena[index];
    node.children = children;
    node.letters = large.letters();
    node.is_end_of_word = large.is_end_of_word();
    
    arena.extend((0..large.letters().len()).map(|_| TrieNode::default()));
    for (child, (_, large)) in large.into_children().enumerate() {
        build_arena(arena, children as usize + child, large);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TrieRef<'a> {
    arena: &'a [TrieNode],
    index: usize,
}

impl<'a> TrieRef<'a> {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn child(&self, letter: Letter) -> Option<TrieRef<'a>> {
        let node = &self.arena[self.index];
        match node.letters.contains(letter) {
            true => {
                let before_mask = (1 << letter as u8) - 1;
                let before = node.letters.as_repr() & before_mask;
                let index = before.count_ones() as usize;
                Some(TrieRef {
                    arena: self.arena,
                    index: node.children as usize + index,
                })
            }
            false => None,
        }
    }

    pub fn children(&self) -> impl Iterator<Item=(Letter, TrieRef<'a>)> {
        let node = &self.arena[self.index];
        node.letters.iter().enumerate().map(|(i, letter)| {
            (letter, TrieRef {
                arena: self.arena,
                index: node.children as usize + i,
            })
        })
    }

    pub fn is_end_of_word(&self) -> bool {
        self.arena[self.index].is_end_of_word
    }
}
