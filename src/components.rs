pub use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    // ColorPair is a helper class from bracket-lib that stores both a foreground and background color in a single struct.
    pub color: ColorPair,
    // FontCharType is defined in bracket-lib to store a single character or glyph.
    pub glyph: FontCharType,
}

/*
Hero Tag Component
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;

/*
Enemy tag component.
 */
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Enemy;

//anything that has the component will wander aimlessly around the map
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MovingRandomly;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WantsToMove {
    pub entity: Entity,
    pub destination : Point,
}