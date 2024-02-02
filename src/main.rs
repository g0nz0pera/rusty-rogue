

#![warn(clippy::pedantic)]

// START: prelude
mod map;
mod map_builder;
mod camera;
mod components;
mod spawner;
mod systems;

mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::*;
    pub use legion::world::SubWorld;
    pub use legion::systems::CommandBuffer;
    pub use crate::components::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;

    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH /2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT /2;

    pub use crate::map::*;
    pub use crate::player::*;
    pub use crate::map_builder::*;
    pub use crate::camera::*;

}

use std::task::Poll::Ready;
use crate::prelude::*;
// END: prelude


//Rust commonly uses usize to index collections and arrays.

struct State{
    ecs: World,
    resources: Resources,
    systems: Schedule,

}

impl State {
    fn new() -> Self {
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        //The map builder is set up in the same way we did before, but rather than storing it in State,
        // it’s injected into the world’s resources with insert().
        let map_builder = MapBuilder::new(&mut rng);
        spawn_player(&mut ecs, map_builder.player_start);
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));
        Self {
            ecs,
            resources,
            systems: build_scheduler()
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.set_active_console(0);
        ctx.cls();
        ctx.set_active_console(1);
        ctx.cls();
        //We added ctx.key (which holds the keyboard state) as a resource in our tick() function.
        //This makes the current keyboard state available to any system that requests it.
        self.resources.insert(ctx.key);
        self.systems.execute(&mut self.ecs, &mut self.resources);
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
