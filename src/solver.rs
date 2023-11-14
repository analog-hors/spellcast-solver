use arrayvec::ArrayVec;
use super::words::{Letter, TrieNode};
use super::game::Game;

type Path = ArrayVec<(i8, i8, Letter), {5 * 5}>;

#[derive(Debug, Clone, Default)]
pub struct Solution {
    pub path: Path,
    pub score: u32,
}

fn score(game: &Game, path: &[(i8, i8, Letter)]) -> u32 {
    use Letter::*;

    let mut score = 0;
    let mut word_multiplier = 1;
    for &(x, y, letter) in path {
        let mut letter_score = match letter {
            A | E | I | O => 1,
            N | R | S | T => 2,
            D | G | L => 3,
            B | H | M | P | U | Y => 4,
            C | F | V | W => 5,
            K => 6,
            J | X => 7,
            Q | Z => 8,
        };
        if game.double_letter == Some((x, y)) {
            letter_score *= 2;
        }
        if game.double_word == Some((x, y)) {
            word_multiplier = 2;
        }
        score += letter_score;
    }
    let long_word_bonus = if path.len() >= 6 { 10 } else { 0 };
    score * word_multiplier + long_word_bonus
}

pub fn solve(game: &Game, trie: &TrieNode, swaps: u8) -> Vec<Solution> {
    let mut solutions = Vec::new();
    for x in 0..5 {
        for y in 0..5 {
            for (letter, trie) in trie.children() {

                let existing = game.grid[y as usize][x as usize];
                let swaps = if letter == existing {
                    swaps
                } else if swaps > 0 {
                    swaps - 1
                } else {
                    continue;
                };

                let mut path = Path::default();
                path.push((x, y, letter));
                visit_tile(&game, &mut solutions, &mut path, &trie, swaps, x, y);
            }
        }
    }

    solutions
}

fn visit_tile(
    game: &Game,
    solutions: &mut Vec<Solution>,
    current: &mut Path,
    trie: &TrieNode,
    swaps: u8,
    x: i8,
    y: i8,
) {
    if trie.is_end_of_word() {
        let score = score(game, &current);
        solutions.push(Solution {
            path: current.clone(),
            score,
        });
    }
    for (nx, ny) in neighbours(x, y) {
        if nx < 0 || nx >= Game::WIDTH as i8 || ny < 0 || ny >= Game::HEIGHT as i8 {
            continue;
        }

        let existing = game.grid[ny as usize][nx as usize];
        if let Some(trie) = trie.child(existing) {
            if current.iter().any(|&(px, py, _)| (px, py) == (nx, ny)) {
                continue;
            }
            current.push((x, y, existing));
            visit_tile(game, solutions, current, trie, swaps, nx, ny);
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
                current.push((x, y, letter));
                visit_tile(game, solutions, current, trie, swaps - 1, nx, ny);
                current.pop();
            }
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
