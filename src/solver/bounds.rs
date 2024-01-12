use crate::repr::{Letter, Board};
use super::trie::{TrieArena, TrieRef};
use super::score::{letter_score, long_word_bonus};

pub fn calculate_upper_bounds(arena: &TrieArena, board: &Board, swaps: u8) -> Vec<u16> {
    let mut bounds = vec![0; arena.len()];
    collect_bounds(&mut bounds, &mut Vec::new(), board, swaps, arena.root());
    bounds
}

fn collect_bounds(bounds: &mut [u16], word: &mut Vec<Letter>, board: &Board, swaps: u8, node: TrieRef) -> u16 {
    if !satisfied(counts(&word), counts(&board.grid), swaps) {
        return 0;
    }

    let mut max_score = 0;
    for (letter, child) in node.children() {
        word.push(letter);
        max_score = max_score.max(collect_bounds(bounds, word, board, swaps, child));
        word.pop();
    }
    if node.is_end_of_word() {
        max_score = max_score.max(calculate_max_score(word));
    }

    bounds[node.index()] = max_score;
    max_score
}

fn calculate_max_score(word: &[Letter]) -> u16 {
    let mut word_score = 0;
    let mut max_letter_score = 0;
    for &letter in word {
        word_score += letter_score(letter);
        max_letter_score = max_letter_score.max(letter_score(letter));
    }
    word_score += max_letter_score;    
    word_score * 2 + long_word_bonus(word.len())
}

fn counts(letters: &[Letter]) -> [u8; 26] {
    let mut counts = [0; 26];
    for &letter in letters {
        counts[letter as usize] += 1;
    }
    counts
}

fn satisfied(required: [u8; 26], exists: [u8; 26], swaps: u8) -> bool {
    required.iter().zip(exists).map(|(&r, e)| r.saturating_sub(e)).sum::<u8>() <= swaps
}
