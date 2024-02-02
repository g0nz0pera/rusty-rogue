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
