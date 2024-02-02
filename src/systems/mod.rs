mod player_input;
mod map_render;
mod entity_render;
mod collisions;
mod random_move;
mod end_turn;

use crate::prelude::*;
use crate::systems::collisions::collisions_system;

/*
These functions creates a Legion Schedule (an execution plan for our systems).
t follows the builder pattern: Schedule::builder starts the system building process, and build() finishes it.
This is very similar to our build_scheduler function, but now we’ve three schedulers.
 Note that flush() is called when a system makes changes to the ECS dataset.
 The systems in each phase are carefully divided by what makes sense:

    While awaiting input, the screen still needs to display the map and entities. It also calls the player_input system.
    When it’s the player’s turn, the game doesn’t accept input, but does check for collisions and renders everything. It finishes with end_turn.
    The monsters’ turn is very similar to the player’s but adds random movement.

 */
pub fn build_input_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        //  The flush() call is new. When a system executes commands, they don’t take effect immediately.
        //  There’s a hidden flush at the end of the systems that tell Legion (the ECS library) to apply changes immediately.
        //  Flushing after collision detection ensures that any deleted entities are really gone before they’re rendered.
        // Flushing also guarantees that all systems up to that point have finished executing before the next one runs.
        //  This is a handy way to tame multithreading issues and ensure that subsequent systems use up-to-date information.
        // It’s a good idea to flush() systems after we make changes, or at least before we rely on them.
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_player_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}

pub fn build_monster_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(random_move::random_move_system())
        .flush()
        .add_system(collisions::collisions_system())
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(end_turn::end_turn_system())
        .build()
}