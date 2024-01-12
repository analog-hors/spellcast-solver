use arrayvec::ArrayVec;
use enumset::EnumSet;
use crate::repr::{Letter, Tile};

pub type Path = ArrayVec<(Tile, Letter), {Tile::NUM}>;

#[derive(Debug, Default)]
pub struct PathState {
    path: Path,
    occupied: EnumSet<Tile>,
}

impl PathState {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn push(&mut self, tile: Tile, letter: Letter) {
        self.path.push((tile, letter));
        self.occupied ^= tile;
    }

    pub fn pop(&mut self) {
        if let Some((tile, _)) = self.path.pop() {
            self.occupied ^= tile;
        }
    }

    pub fn neighbours(&self) -> EnumSet<Tile> {
        match self.path.last() {
            Some(&(t, _)) => t.neighbours() - self.occupied,
            None => Tile::ALL,
        }
    }
}
