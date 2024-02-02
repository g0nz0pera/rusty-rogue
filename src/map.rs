// START: header
use crate::NUM_TILES;
use crate::prelude::*;
// END: header

/*We marked TileType as public,
    so the wildcard in our prelude allows any part of the program
    that uses the prelude to use the TileType enumeration.
    */
#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

pub fn map_idx(x: i32, y: i32)-> usize {
    ((y * SCREEN_WIDTH) + x ) as usize
}
impl Map {

    /*
    The constructor uses an extended form of the vec! macro to create a NUM_TILES number of entries,
    each set to TileType::Floor, creating a map that consists entirely of floors.
     */
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }


    pub fn render(&self, ctx: &mut BTerm ){
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let idx = map_idx(x,y);

                match self.tiles[idx] {
                    TileType::Floor => {
                        ctx.set(x,y,YELLOW, BLACK, to_cp437('.'))
                    },
                    TileType::Wall => {
                        ctx.set(x,y,GREEN, BLACK, to_cp437('#'))
                    }
                }
            }
        }
    }
}