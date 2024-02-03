use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(MovingRandomly)]
//There’s no intelligence behind this movement. It’s completely random.
pub fn random_move(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    //This creates a new Query with writable access to Point and read-only access to MovingRandomly.
    let mut movers = <(Entity, &Point, &MovingRandomly)>::query();

    movers.iter(ecs).for_each(|(entity, pos, _)| {
        let mut rng = RandomNumberGenerator::new();
        let destination = match rng.range(0,4){
            0 => Point::new(-1, 0),
            1 => Point::new(1, 0),
            2 => Point::new(0, -1),
            _ => Point::new(0, 1),
        } + *pos;

        commands
            .push(((), WantsToMove{ entity: *entity, destination })
        );
    })
}