mod player_input;
mod map_render;
mod entity_render;
mod collisions;
mod random_move;
mod end_turn;

use crate::prelude::*;

/*
This function creates a Legion Schedule (an execution plan for our systems).
t follows the builder pattern: Schedule::builder starts the system building process, and build() finishes it.
 */
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        //  The flush() call is new. When a system executes commands, they don’t take effect immediately.
        //  There’s a hidden flush at the end of the systems that tell Legion (the ECS library) to apply changes immediately.
        //  Flushing after collision detection ensures that any deleted entities are really gone before they’re rendered.
        // Flushing also guarantees that all systems up to that point have finished executing before the next one runs.
        //  This is a handy way to tame multithreading issues and ensure that subsequent systems use up-to-date information.
        // It’s a good idea to flush() systems after we make changes, or at least before we rely on them.
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())

        .build()
}