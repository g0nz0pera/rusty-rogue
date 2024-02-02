

#![warn(clippy::pedantic)]

// START: prelude
mod map;
mod player;
mod map_builder;
mod camera;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH /2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT /2;

    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;

}

use crate::prelude::*;
// END: prelude


//Rust commonly uses usize to index collections and arrays.

struct State{
    map: Map,
    player: Player,
    camera: Camera

}

impl State {
    fn new() -> Self {
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);
        Self {
            map: map_builder.map,
            player: Player::new(map_builder.player_start),
            camera: Camera::new(map_builder.player_start)
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        self.player.update(ctx, &self.map, &mut self.camera);
        self.map.render(ctx, &self.camera);
        self.player.render(ctx, &self.camera);
    }

}

fn main() -> BError {
    let context = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        .with_fps_cap(30.0)
        //This uses with_dimensions to specify the size of subsequent consoles we add.
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        // This uses with_tile_dimension that specifies the size of each character in our font file, in this case, 32x32.
        .with_tile_dimensions(32, 32)
        //This specifies the directory in which we placed the graphics file.
        .with_resource_path("resources/")
        //This is the name of the font file to load and the character dimensions. These are usually the same as tile dimensions but can be different in some advanced forms of rendering.
        .with_font("dungeonfont.png", 32, 32)
        //This adds a console using the dimensions already specified and the named tile graphics file.
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // This adds a second console, with no background so that transparency shows through it.
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(context, State::new())


}
