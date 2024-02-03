pub use crate::prelude::*;


//  Legion provides a shorthand for systems that only run a single query.
//  Declaring a system as a system(for_each) derives the query from the system parameters and runs
//  the system once for every matching entity.
//  This is the same as declaring a query that reads Entity and WantsToMove and iterating it,
//  as we have with other systems.
#[system(for_each)]
#[read_component(Player)]
pub fn movement (
    entity: &Entity,
    want_move: &WantsToMove,
    #[resource] map: &Map,
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
    )
{
    if map.can_enter_tile(want_move.destination) {
        //  This uses commands, rather than directly modifying the component.
        //  Legion can batch the updates and perform them all at once very quickly.
        //  Adding a component that already exists replaces the old one.
        commands.add_component(want_move.entity, want_move.destination);
        if ecs.entry_ref(want_move.entity).unwrap().get_component::<Player>().is_ok() {
            camera.on_player_move(want_move.destination);
        }
    }
    commands.remove(*entity);
}