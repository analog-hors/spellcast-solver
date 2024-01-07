use std::cmp::Reverse;
use std::collections::BinaryHeap;

use arrayvec::ArrayVec;
use super::words::{Letter, TrieNode};
use super::game::{Game, letter_score, long_word_bonus};

type Path = ArrayVec<(i8, i8, Letter), {5 * 5}>;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Solution {
    pub path: Path,
    pub score: u32,
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

fn score(game: &Game, path: &[(i8, i8, Letter)]) -> u32 {
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
    for x in 0..5 {
        for y in 0..5 {
            for (letter, trie) in &trie.children {

                let existing = game.grid[y as usize][x as usize];
                let swaps = if *letter == existing {
                    swaps
                } else if swaps > 0 {
                    swaps - 1
                } else {
                    continue;
                };

                let mut path = Path::default();
                path.push((x, y, *letter));
                visit_tile(&game, &mut solutions, max_solutions, &mut path, &trie, swaps, x, y);
            }
        }
    }

    solutions.into_iter().map(|Reverse(s)| s).collect()
}

fn visit_tile(
    game: &Game,
    solutions: &mut BinaryHeap<Reverse<Solution>>,
    max_solutions: usize,
    current: &mut Path,
    trie: &TrieNode,
    swaps: u8,
    x: i8,
    y: i8,
) {
    if let Some(Reverse(lower_bound)) = solutions.peek() {
        if solutions.len() >= max_solutions && trie.max_score <= lower_bound.score {
            return;
        }
    }
    if trie.is_end_of_word {
        let score = score(game, &current);
        solutions.push(Reverse(Solution {
            path: current.clone(),
            score,
        }));
        if solutions.len() > max_solutions {
            solutions.pop();
        }
    }
    for (nx, ny) in neighbours(x, y) {
        if nx < 0 || nx >= Game::WIDTH as i8 || ny < 0 || ny >= Game::HEIGHT as i8 {
            continue;
        }

        for (letter, trie) in &trie.children {
            let existing = game.grid[ny as usize][nx as usize];
            let swaps = if *letter == existing {
                swaps
            } else if swaps > 0 {
                swaps - 1
            } else {
                continue;
            };
        
            if current.iter().any(|&(px, py, _)| (px, py) == (nx, ny)) {
                continue;
            }

            current.push((nx, ny, *letter));
            visit_tile(game, solutions, max_solutions, current, trie, swaps, nx, ny);
            current.pop();
        }
    }
}

fn neighbours(x: i8, y: i8) -> impl Iterator<Item=(i8, i8)> {
    [
        (x + 1, y + 1),
        (x    , y + 1),
        (x - 1, y + 1),
        (x + 1, y    ),
        (x - 1, y    ),
        (x + 1, y - 1),
        (x    , y - 1),
        (x - 1, y - 1),
    ].into_iter()
}
