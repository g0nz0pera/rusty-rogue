

#![warn(clippy::pedantic)]

// START: prelude
mod map;
mod player;
mod map_builder;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH /2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT /2;

    pub const DUNGEON_FONT: &str = "dungeonfont.png";

    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
}

use crate::prelude::*;
// END: prelude


//Rust commonly uses usize to index collections and arrays.
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT ) as usize;

struct State{
    map: Map,
    player: Player,
}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: Map::new(),
            player: Player::new(
                map_builder.player_start,
            )
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        self.player.update(ctx, &self.map);
        self.map.render(ctx);
        self.player.render(ctx);
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        //This uses with_dimensions to specify the size of subsequent consoles we add.
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        // This uses with_tile_dimension that specifies the size of each character in our font file, in this case, 32x32.
        .with_tile_dimensions(32, 32)
        //This specifies the directory in which we placed the graphics file.
        .with_resource_path("resources/")
        //This is the name of the font file to load and the character dimensions. These are usually the same as tile dimensions but can be different in some advanced forms of rendering.
        .with_font(DUNGEON_FONT, 32, 32)
        //This adds a console using the dimensions already specified and the named tile graphics file.
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, DUNGEON_FONT)
        // This adds a second console, with no background so that transparency shows through it.
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, DUNGEON_FONT)
        .build()?;

    main_loop(context, State::new())
}
