use std::cmp::Reverse;
use std::collections::BinaryHeap;

use arrayvec::ArrayVec;
use super::words::{Letter, TrieNode};
use super::game::{Game, letter_score, long_word_bonus};

type Path = ArrayVec<(i8, i8, Letter), {5 * 5}>;

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

fn score(game: &Game, path: &[(i8, i8, Letter)]) -> u16 {
    let mut word_score = 0;
    let mut word_multiplier = 1;
    for &(x, y, letter) in path {
        let mut letter_score = letter_score(letter);
        if game.double_letter == Some((x, y)) {
            letter_score *= 2;
        }
        if game.double_word == Some((x, y)) {
            word_multiplier = 2;
        }
        word_score += letter_score;
    }
    word_score * word_multiplier + long_word_bonus(path.len())
}

pub fn solve(game: &Game, trie: &TrieNode, swaps: u8, max_solutions: usize) -> Vec<Solution> {
    let mut solutions = BinaryHeap::new();
    search(game, &mut solutions, max_solutions, &mut Path::default(), trie, swaps);
    solutions.into_iter().map(|Reverse(s)| s).collect()
}

fn search(
    game: &Game,
    solutions: &mut BinaryHeap<Reverse<Solution>>,
    max_solutions: usize,
    current: &mut Path,
    trie: &TrieNode,
    swaps: u8,
) {
    let min_solution = solutions.peek().map(|Reverse(s)| s.score);
    if solutions.len() >= max_solutions && Some(trie.max_score()) <= min_solution {
        return;
    }
    if trie.is_end_of_word() {
        solutions.push(Reverse(Solution {
            path: current.clone(),
            score: score(game, current),
        }));
        if solutions.len() > max_solutions {
            solutions.pop();
        }
    }

    let (moves_x, moves_y) = match current.last() {
        Some(&(x, y, _)) => (
            (x - 1).max(0)..(x + 2).min(Game::WIDTH as i8),
            (y - 1).max(0)..(y + 2).min(Game::HEIGHT as i8),
        ),
        None => (0..Game::WIDTH as i8, 0..Game::HEIGHT as i8),
    };

    for nx in moves_x {
        for ny in moves_y.clone() {
            let existing = game.grid[ny as usize][nx as usize];
            if let Some(trie) = trie.child(existing) {
                if current.iter().any(|&(px, py, _)| (px, py) == (nx, ny)) {
                    continue;
                }
                current.push((nx, ny, existing));
                search(game, solutions, max_solutions, current, trie, swaps);
                current.pop();
            }
            if swaps > 0 {
                for (letter, trie) in trie.children() {
                    if letter == existing {
                        continue;
                    }
                    if current.iter().any(|&(px, py, _)| (px, py) == (nx, ny)) {
                        break;
                    }
                    current.push((nx, ny, letter));
                    search(game, solutions, max_solutions, current, trie, swaps - 1);
                    current.pop();
                }
            }
        }
    }
}
