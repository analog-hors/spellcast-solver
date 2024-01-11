mod game;
mod words;
mod solver;
mod tile;

use game::Game;
use words::Letter;
use tile::Tile;

fn main() {
    let mut args = std::env::args().skip(1);
    let swaps = args.next().unwrap().parse().unwrap();
    let max_solutions = args.next().unwrap().parse().unwrap();

    let input = std::io::read_to_string(std::io::stdin()).unwrap();
    
    let mut game = Game {
        grid: [Letter::A; Tile::NUM],
        double_word: None,
        double_letter: None,
    };
    for (i, tile_str) in input.split_ascii_whitespace().enumerate() {
        let tile = Tile::index(i as u8).unwrap();
        let mut tile_str = tile_str.chars();
        game.grid[tile as usize] = tile_str.next().unwrap().try_into().unwrap();
        for modifier in tile_str {
            match modifier {
                'l' => game.double_letter = Some(tile),
                'w' => game.double_word = Some(tile),
                _ => {}
            }
        }
    }

    let trie = words::make_word_trie(include_str!("../wordlist.txt").lines());

    let start = std::time::Instant::now();
    let mut solutions = solver::solve(&game, &trie, swaps, max_solutions);
    let elapsed = start.elapsed();

    solutions.sort_unstable_by_key(|s| s.score);
    for solution in solutions.iter() {
        let word = solution.path
            .iter()
            .map(|&(_, l)| char::from(l))
            .collect::<String>();
        println!("{} {} {:?}", word, solution.score, solution.path);
    }
    println!("{} solutions enumerated in {:?}", solutions.len(), elapsed);
}
