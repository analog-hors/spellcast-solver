use std::cmp::Reverse;
use std::collections::BinaryHeap;

use crate::repr::{Board, Letter, Tile};
use super::TrieArena;
use super::score::{letter_score, long_word_bonus};
use super::trie::TrieRef;
use super::path::{Path, PathState};
use super::bounds::calculate_upper_bounds;

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

pub fn solve(board: &Board, trie: &TrieArena, swaps: u8, max_solutions: usize) -> Vec<Solution> {
    let mut solutions = BinaryHeap::new();
    let upper_bounds = calculate_upper_bounds(trie, board, swaps);
    search(board, &upper_bounds, &mut solutions, max_solutions, &mut PathState::default(), trie.root(), swaps);
    solutions.into_sorted_vec().into_iter().map(|Reverse(s)| s).collect()
}

fn search(
    board: &Board,
    upper_bounds: &[u16],
    solutions: &mut BinaryHeap<Reverse<Solution>>,
    max_solutions: usize,
    current: &mut PathState,
    trie: TrieRef,
    swaps: u8,
) {
    let max_score = upper_bounds[trie.index()];
    let min_solution = solutions.peek().map(|Reverse(s)| s.score);
    if solutions.len() >= max_solutions && Some(max_score) <= min_solution {
        return;
    }
    if trie.is_end_of_word() {
        solutions.push(Reverse(Solution {
            path: current.path().clone(),
            score: score(board, current.path()),
        }));
        if solutions.len() > max_solutions {
            solutions.pop();
        }
    }

    for next in current.neighbours() {
        let existing = board.grid[next as usize];
        if let Some(trie) = trie.child(existing) {
            current.push(next, existing);
            search(board, upper_bounds, solutions, max_solutions, current, trie, swaps);
            current.pop();
        }
    }
    if swaps > 0 {
        for next in current.neighbours() {
            let existing = board.grid[next as usize];
            for (letter, trie) in trie.children().filter(|&(l, _)| l != existing) {
                current.push(next, letter);
                search(board, upper_bounds, solutions, max_solutions, current, trie, swaps - 1);
                current.pop();
            }
        }
    }
}

fn score(board: &Board, path: &[(Tile, Letter)]) -> u16 {
    let mut word_score = 0;
    let mut word_multiplier = 1;
    for &(tile, letter) in path {
        let mut letter_score = letter_score(letter);
        if Some(tile) == board.double_letter {
            letter_score *= 2;
        }
        if Some(tile) == board.double_word {
            word_multiplier = 2;
        }
        word_score += letter_score;
    }
    word_score * word_multiplier + long_word_bonus(path.len())
}
