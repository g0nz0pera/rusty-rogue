// START: header
use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT ) as usize;

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

    /*
    The function receives a borrowed Camera and uses the boundaries from the camera to render only the visible part of the map.
    Notice that it now calls in_bounds to ensure that each tile exists.
    The screen coordinates sent to the set function have left_x and top_y subtracted from them, moving them to be relative to the camera.
    Notice that it calls set_active_console(0).
     */
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera ){
        ctx.set_active_console(0);
        for y in camera.top_y .. camera.bottom_y {
            for x in camera.left_x .. camera.right_x {
                if self.in_bounds(Point::new(x,y)) {
                    let idx = map_idx(x,y);
                    match self.tiles[idx] {
                        TileType::Floor => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('.')
                            );
                        }
                        TileType::Wall => {
                            ctx.set(
                                x - camera.left_x,
                                y - camera.top_y,
                                WHITE,
                                BLACK,
                                to_cp437('#')
                            );
                        }
                    }
                }
            }
        }
    }

    /*
    This function checks that the location specified in point is greater
    than 0 on both the x and y axes and that it’s less than the screen height and width.
     */
    pub fn in_bounds(&self, point : Point ) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    /*
        We need a second function to determine if the player can enter a tile.
        Players can walk on floors, but not through walls. This function should call the in_bounds
        function we just wrote, to ensure that the move is valid both dimensionally and for the TileType.
        If both are true, the adventurer may enter the tile.
     */
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_idx(point.x, point.y)] == TileType::Floor
    }

    /*
    It’d be useful to have a way to determine a tile’s index coordinates,
    failing if the requested coordinates fall outside of the map boundaries.
     */
    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point){
            None
        } else {
            Some(map_idx(point.x, point.y))
        }
    }
}