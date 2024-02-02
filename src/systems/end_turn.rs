use crate::prelude::*;

#[system]
pub fn end_turn(#[resource] turn_state: &mut TurnState) {
    let new_state = match turn_state {
        //If the game awaits input, there’s nothing to do, so exit the function immediately with the return.
        TurnState::AwaitingInput => return,
        // If it’s currently the player’s turn, the next phase is the monsters’ turn.
        TurnState::PlayerTurn => TurnState::MonsterTurn,
        // If the monsters’ turn is ending, return to waiting for input.
        TurnState::MonsterTurn => TurnState::AwaitingInput
    };

    // This sets the turn resource to the chosen value.
    // The asterisk (*) dereferences the variable, allowing us to write directly to the stored resource.
    *turn_state = new_state;
}