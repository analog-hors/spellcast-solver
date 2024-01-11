mod repr;
mod solver;

use repr::{Board, Tile, Letter};
use solver::{make_word_trie, solve};

fn main() {
    let mut args = std::env::args().skip(1);
    let swaps = args.next().unwrap().parse().unwrap();
    let max_solutions = args.next().unwrap().parse().unwrap();

    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    
    let mut board = Board {
        grid: [Letter::A; Tile::NUM],
        double_word: None,
        double_letter: None,
    };
    for (i, tile_str) in input.split_ascii_whitespace().enumerate() {
        let tile = Tile::index(i as u8).unwrap();
        let mut tile_str = tile_str.chars();
        board.grid[tile as usize] = tile_str.next().unwrap().try_into().unwrap();
        for modifier in tile_str {
            match modifier {
                'l' => board.double_letter = Some(tile),
                'w' => board.double_word = Some(tile),
                _ => {}
            }
        }
    }

    let trie = make_word_trie(include_str!("../wordlist.txt").lines());

    let start = std::time::Instant::now();
    let solutions = solve(&board, &trie, swaps, max_solutions);
    let elapsed = start.elapsed();

    for solution in solutions.iter().rev() {
        let word = solution.path
            .iter()
            .map(|&(_, l)| char::from(l))
            .collect::<String>();
        println!("{} {} {:?}", word, solution.score, solution.path);
    }
    println!("{} solutions enumerated in {:?}", solutions.len(), elapsed);
}
