use crate::prelude::*;

/*
This is our first nested module—the module is inside our systems module.
The #[system] line annotates the player_input function with a procedural macro named system.
This macro transforms the function name into player_input_system
and wraps it with all the extra code Legion requires to construct a system.
 */
#[system]
#[read_component(Point)]
//read_component requests read-only access to a component type.
//We must request read access to use the values stored in a component of this type, but we can’t make changes to the stored value.
#[read_component(Player)]
pub fn player_input(
    //A SubWorld is like a World but can only see the components we request.
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState
){
    let mut players =
        <(Entity, &Point)>::query().filter(component::<Player>());
    //END:top

    //START: delta
    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        //END: delta

        //START: message
        players.iter(ecs).for_each(| (entity, pos) | {
            let destination = *pos + delta;
            commands
                .push(((), WantsToMove{ entity: *entity, destination }));
        });
        *turn_state = TurnState::PlayerTurn;
        //END: message
    }
}