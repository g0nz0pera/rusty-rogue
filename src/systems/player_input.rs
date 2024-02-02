use crate::prelude::*;

/*
This is our first nested module—the module is inside our systems module.
The #[system] line annotates the player_input function with a procedural macro named system.
This macro transforms the function name into player_input_system
and wraps it with all the extra code Legion requires to construct a system.
 */
#[system]
//write_component requests writable access to a component type,
//We must request write access if we intend to change the contents of a component in your system.
#[write_component(Point)]
//read_component requests read-only access to a component type.
//We must request read access to use the values stored in a component of this type, but we can’t make changes to the stored value.
#[read_component(Player)]
pub fn player_input(
    //A SubWorld is like a World but can only see the components we request.
    ecs: &mut SubWorld,
    //#[resource] requests access to types we stored in Legion’s Resource handler. It’s also a procedural macro.
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
){
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            //We access components with a query
            //Queries list one or more components and return references, mutable if we use &mut to each instance of that component type.
            //If we request more than one component type, only entities that have all those components are returned.
            //The components are grouped together to ensure that each returned set of components only operates on the entity that owns the components.
            //Legion queries include a filter() function to further refine the set of components required for a query to match an entity.
            //
            //This line specifies that only entities with a Point component and a Player tag component should be included in the query.
            let mut players = <&mut Point>::query().filter(component::<Player>());
            //The query doesn’t become an iterator until we call iter() or iter_mut(); it’s still a Query
            //Adding filters before the iterator call limits the types included in the query.
            //Query filters can require that a component exists but can’t refer to its content.
            // If we need to filter on the component’s content, we can use the iterator’s filter() function instead.
            //Calling iter_mut() runs the query we’ve defined and places the results in an iterator.

            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination){
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            });
        }
    }
}