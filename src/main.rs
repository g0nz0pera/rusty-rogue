mod map;

mod prelude {
    //Publicly using the bracket_lib prelude re-exports it inside our prelude.
    // Anything that uses our prelude also uses bracket_lib.
    pub use bracket_lib::prelude::*;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub use crate::map::*;
}

use crate::prelude::*;
//Rust commonly uses usize to index collections and arrays.
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT ) as usize;

fn main() {
    println!("Hello, world!");
}
