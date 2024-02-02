use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(MovingRandomly)]
//There’s no intelligence behind this movement. It’s completely random.
pub fn random_move(ecs: &mut SubWorld, #[resource] map: &Map) {
    //This creates a new Query with writable access to Point and read-only access to MovingRandomly.
    let mut movers = <(&mut Point, &MovingRandomly)>::query();

    movers
        .iter_mut(ecs)
        .for_each(|(pos, _)| {
            let mut rng = RandomNumberGenerator::new();
            // This randomly chooses a direction in which to move and store the delta.
            // Add pos (the position) to it to determine the destination.
            let destination = match rng.range(0, 4) {
                0 => Point::new(-1, 0),
                1 => Point::new(1, 0),
                2 => Point::new(0, -1),
                _ => Point::new(0, 1),
            } + *pos;

            if map.can_enter_tile(destination) {
                *pos = destination;
            }
        });
}