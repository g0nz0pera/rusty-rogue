mod player_input;

use crate::prelude::*;

/*
This function creates a Legion Schedule (an execution plan for our systems).
t follows the builder pattern: Schedule::builder starts the system building process, and build() finishes it.
 */
pub fn build_scheduler() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .build()
}