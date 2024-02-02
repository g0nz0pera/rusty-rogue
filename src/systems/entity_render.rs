use crate::prelude::*;

/*
    The system requests read-only access to Point and Render components and read-only access to the Camera resource.
    The camera calculates the offset to apply to our entity’s screen position, just like we did before.
    The Point component tells us where the entity is, and the Render component describes its appearance.
 */
#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera){
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    // We can perform a query with multiple types with the following syntax:
    // The outer < and > denote that the contents contain types.
    // The parentheses indicate a tuple (a collection of data accessed together).
    // We then list each component type we want as a reference, separated by commas.
    // This query looks for entities that contain both a Point and a Render component,
    //  returning only entities that have both.
    <(&Point, &Render)>::query()
        .iter(ecs)
        //This uses for_each() that works the same on a query as it does on a vector.
        // Each call receives the query’s components in a tuple. Destructure these to use the components by name.
        .for_each(|(pos, render)| {

            //This sets the screen character at the position in pos to the glyph and color specified in the Render component
            draw_batch.set(
                *pos -offset,
                render.color,
                render.glyph,
            );
        });

    //This submits the rendered batch. The number 5,000 is used as a sort order because the map may include 4,000 elements,
    // and it’s a good idea to leave some room if that changes or if we add some user interface elements.
    draw_batch.submit(5000).expect("Batch Error");

}
