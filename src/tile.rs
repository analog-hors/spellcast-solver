use enumset::{EnumSetType, EnumSet};

pub const BOARD_WIDTH: usize = 5;
pub const BOARD_HEIGHT: usize = 5;

#[derive(Debug, EnumSetType)]
#[enumset(repr = "u32")]
#[repr(u8)]
pub enum Tile {
    A0, B0, C0, D0, E0,
    A1, B1, C1, D1, E1,
    A2, B2, C2, D2, E2,
    A3, B3, C3, D3, E3,
    A4, B4, C4, D4, E4,
}

impl Tile {
    pub const ALL: EnumSet<Tile> = EnumSet::ALL;
    pub const NUM: usize = BOARD_WIDTH * BOARD_HEIGHT;

    pub const fn new(x: i8, y: i8) -> Option<Tile> {
        if x < 0 || (x as usize) >= BOARD_WIDTH || y < 0 || (y as usize) >= BOARD_HEIGHT {
            return None;
        }
        Self::index((y * 5 + x) as u8)
    }

    pub const fn index(i: u8) -> Option<Tile> {
        if (i as usize) >= Self::NUM {
            return None;
        }
        //SAFETY: Tile is repr(u8) and i is guaranteed
        // to be in range due to the above guard
        unsafe { Some(std::mem::transmute(i)) }
    }

    pub fn neighbours(&self) -> EnumSet<Tile> {
        const fn n(x: i8, y: i8) -> u32 {
            match Tile::new(x, y) {
                Some(t) => 1 << t as u8,
                None => 0,
            }
        }
        const TABLE: [u32; Tile::NUM] = {
            let mut table = [0; Tile::NUM];
            let mut y = 0;
            while (y as usize) < BOARD_HEIGHT {
                let mut x = 0;
                while (x as usize) < BOARD_WIDTH {
                    let Some(t) = Tile::new(x, y) else { unreachable!() };
                    table[t as usize] = n(x - 1, y) | n(x + 1, y)
                        | n(x - 1, y - 1) | n(x, y - 1) | n(x + 1, y - 1)
                        | n(x - 1, y + 1) | n(x, y + 1) | n(x + 1, y + 1);
                    x += 1;
                }
                y += 1;
            }
            table
        };
        EnumSet::from_repr(TABLE[*self as usize])
    }
}
