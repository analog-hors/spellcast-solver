use arrayvec::ArrayVec;
use super::words::{Letter, TrieNode};
use super::game::Game;

type Bitboard = u32;

type Path = ArrayVec<(i8, i8, Letter), {5 * 5}>;

#[derive(Debug, Default, Clone)]
struct PathState {
    path: Path,
    occupied: Bitboard,
}

impl PathState {
    pub fn push(&mut self, x: i8, y: i8, l: Letter) {
        self.path.push((x, y, l));
        self.occupied ^= bb(x, y);
    }

    pub fn pop(&mut self) {
        let (x, y, _) = self.path.pop().unwrap();
        self.occupied ^= bb(x, y);
    }
}

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

                let mut path = PathState::default();
                path.push(x, y, letter);
                visit_tile(&game, &mut solutions, &mut path, &trie, swaps, x, y);
            }
        }
    }

    solutions
}

fn visit_tile(
    game: &Game,
    solutions: &mut Vec<Solution>,
    current: &mut PathState,
    trie: &TrieNode,
    swaps: u8,
    x: i8,
    y: i8,
) {
    if trie.is_end_of_word() {
        let score = score(game, &current.path);
        solutions.push(Solution {
            path: current.path.clone(),
            score,
        });
    }
    for (nx, ny) in iter_bb(neighbours(x, y) & !current.occupied) {
        let existing = game.grid[ny as usize][nx as usize];
        if let Some(trie) = trie.child(existing) {
            current.push(nx, ny, existing);
            visit_tile(game, solutions, current, trie, swaps, nx, ny);
            current.pop();
        }
        if swaps > 0 {
            for (letter, trie) in trie.children() {
                if letter == existing {
                    continue;
                }
                current.push(nx, ny, letter);
                visit_tile(game, solutions, current, trie, swaps - 1, nx, ny);
                current.pop();
            }
        }
    }
}

fn neighbours(x: i8, y: i8) -> u32 {
    const TABLE: [[u32; Game::WIDTH]; Game::HEIGHT] = {
        const fn n(x: i8, y: i8) -> u32 {
            if x < 0 || x >= Game::WIDTH as i8 || y < 0 || y >= Game::HEIGHT as i8 {
                return 0;
            }
            bb(x, y)
        }

        let mut table = [[0; Game::WIDTH]; Game::HEIGHT];
        let mut y = 0;
        while y < Game::HEIGHT as i8 {
            let mut x = 0;
            while x < Game::WIDTH as i8 {
                table[y as usize][x as usize] = n(x + 1, y + 1)
                    | n(x    , y + 1)
                    | n(x - 1, y + 1)
                    | n(x + 1, y    )
                    | n(x - 1, y    )
                    | n(x + 1, y - 1)
                    | n(x    , y - 1)
                    | n(x - 1, y - 1);
                x += 1;
            }
            y += 1;
        }
        table
    };
    TABLE[y as usize][x as usize]
}

const fn bb(x: i8, y: i8) -> u32 {
    1 << (y as usize * Game::WIDTH + x as usize)
}

fn iter_bb(mut bb: u32) -> impl Iterator<Item=(i8, i8)> {
    std::iter::from_fn(move || {
        if bb == 0 {
            return None;
        }

        let index = bb.trailing_zeros();
        bb &= bb - 1;

        let x = index as usize % Game::WIDTH;
        let y = index as usize / Game::WIDTH;
        Some((x as i8, y as i8))
    })
}
