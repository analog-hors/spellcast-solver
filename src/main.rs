mod game;
mod words;
mod solver;

use game::Game;
use words::Letter;

fn main() {
    let swaps = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut parts = std::io::stdin()
        .lines()
        .flat_map(|l| l.unwrap().split_ascii_whitespace().map(|s| s.to_owned()).collect::<Vec<_>>());
    
    let mut game = Game {
        grid: [[Letter::A; Game::WIDTH]; Game::HEIGHT],
        double_word: None,
        double_letter: None,
    };
    for y in 0..Game::WIDTH {
        for x in 0..Game::HEIGHT {
            let tile = parts.next().unwrap();
            let mut tile = tile.chars();
            let letter = tile.next().unwrap().try_into().unwrap();
            game.grid[y][x] = letter;
            while let Some(modifier) = tile.next() {
                match modifier {
                    'l' => game.double_letter = Some((x as i8, y as i8)),
                    'w' => game.double_word = Some((x as i8, y as i8)),
                    _ => {}
                }
            }
        }
    }

    let trie = words::make_word_trie(include_str!("../wordlist.txt").lines());

    let start = std::time::Instant::now();
    let mut solutions = solver::solve(&game, &trie, swaps);
    let elapsed = start.elapsed();

    solutions.sort_unstable_by_key(|s| s.score);
    for solution in &solutions {
        let word = solution.path
            .iter()
            .map(|&(_, _, l)| char::from(l))
            .collect::<String>();
        println!("{} {} {:?}", word, solution.score, solution.path);
    }
    println!("{} solutions enumerated in {:?}", solutions.len(), elapsed);
}
