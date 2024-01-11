use std::cmp::Reverse;
use std::collections::BinaryHeap;

use arrayvec::ArrayVec;
use enumset::EnumSet;
use crate::repr::{Board, Letter, Tile};
use super::score::{letter_score, long_word_bonus};
use super::trie::TrieNode;

type Path = ArrayVec<(Tile, Letter), {Tile::NUM}>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Solution {
    pub path: Path,
    pub score: u16,
}

impl Ord for Solution {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score)
    }
}

impl PartialOrd for Solution {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default)]
struct PathState {
    path: Path,
    occupied: EnumSet<Tile>,
}

impl PathState {
    fn push(&mut self, tile: Tile, letter: Letter) {
        self.path.push((tile, letter));
        self.occupied |= tile;
    }

    fn pop(&mut self) {
        if let Some((tile, _)) = self.path.pop() {
            self.occupied -= tile;
        }
    }

    fn neighbours(&self) -> EnumSet<Tile> {
        match self.path.last() {
            Some(&(t, _)) => t.neighbours() - self.occupied,
            None => Tile::ALL,
        }
    }
}

pub fn solve(game: &Board, trie: &TrieNode, swaps: u8, max_solutions: usize) -> Vec<Solution> {
    let mut solutions = BinaryHeap::new();
    search(game, &mut solutions, max_solutions, &mut PathState::default(), trie, swaps);
    solutions.into_iter().map(|Reverse(s)| s).collect()
}

fn search(
    game: &Board,
    solutions: &mut BinaryHeap<Reverse<Solution>>,
    max_solutions: usize,
    current: &mut PathState,
    trie: &TrieNode,
    swaps: u8,
) {
    let min_solution = solutions.peek().map(|Reverse(s)| s.score);
    if solutions.len() >= max_solutions && Some(trie.max_score()) <= min_solution {
        return;
    }
    if trie.is_end_of_word() {
        solutions.push(Reverse(Solution {
            path: current.path.clone(),
            score: score(game, &current.path),
        }));
        if solutions.len() > max_solutions {
            solutions.pop();
        }
    }

    for next in current.neighbours() {
        let existing = game.grid[next as usize];
        if let Some(trie) = trie.child(existing) {
            current.push(next, existing);
            search(game, solutions, max_solutions, current, trie, swaps);
            current.pop();
        }
        if swaps > 0 {
            for (letter, trie) in trie.children().filter(|&(l, _)| l != existing) {
                current.push(next, letter);
                search(game, solutions, max_solutions, current, trie, swaps - 1);
                current.pop();
            }
        }
    }
}

fn score(game: &Board, path: &[(Tile, Letter)]) -> u16 {
    let mut word_score = 0;
    let mut word_multiplier = 1;
    for &(tile, letter) in path {
        let mut letter_score = letter_score(letter);
        if Some(tile) == game.double_letter {
            letter_score *= 2;
        }
        if Some(tile) == game.double_word {
            word_multiplier = 2;
        }
        word_score += letter_score;
    }
    word_score * word_multiplier + long_word_bonus(path.len())
}
