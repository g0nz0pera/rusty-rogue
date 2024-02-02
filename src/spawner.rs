use crate::prelude::*;

/*
The function requires a mutable reference to the World and the location in which it should spawn the adventurer.
    We create components by calling push, just like a vector. The components are separated in a tuple.
Calling push() creates a new entity composed of the listed components.
This adds a tag component, indicating that this is the player.
    Tag components are treated like other components.
 */
pub fn spawn_player(ecs: &mut World, pos: Point) {
    ecs.push(
        (
            Player,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                glyph: to_cp437('@')
            }
        )
    );
}

pub fn spawn_monster(ecs: &mut World,
                     rng: &mut RandomNumberGenerator,
                     pos: Point) {
    ecs.push(
        (Enemy,
            pos,
            Render {
                color: ColorPair::new(WHITE, BLACK),
                //the spawning code randomly selects one of four monster types
                glyph: match rng.range(0,4) {
                    0 => to_cp437('E'),
                    1 => to_cp437('O'),
                    2 => to_cp437('o'),
                    _ => to_cp437('g'),
                }
            },
            MovingRandomly{}
        )
    );
}