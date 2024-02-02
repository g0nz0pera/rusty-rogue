use crate::prelude::*;

#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
pub fn collisions(ecs: &mut SubWorld, commands: &mut CommandBuffer) {
    let mut player_pos = Point::zero();
    let mut players = <&Point>::query().filter(component::<Player>());

    players.iter(ecs).for_each(|pos| player_pos = *pos);

    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    enemies
        .iter(ecs)
        //  filter removes iterator entries that don’t meet the criteria.
        //  we’re only filtering positions that match the adventurer’s position
        //  By the time pos reaches the filter function, it has the type, &&Point.
        //  It entered the query as a reference, and the iterator references it again.
        //  We want to compare this with its actual value, so ** removes the references.
        .filter(|(_, pos)| **pos == player_pos )
        //  The first tuple entry is the Entity.
        //  We can ignore the position because we only need it for the filter.
        .for_each(|(entity, _)| {
            // ECS commands provide the ability to create and delete entities from within systems.
            // Calling commands.remove() instructs Legion to remove the specified entity from the world at the end of the frame.
            commands.remove(*entity)
        });
}