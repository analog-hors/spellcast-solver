use super::Tile;
use super::Letter;

#[derive(Debug, Clone)]
pub struct Board {
    pub grid: [Letter; Tile::NUM],
    pub double_word: Option<Tile>,
    pub double_letter: Option<Tile>,
}
