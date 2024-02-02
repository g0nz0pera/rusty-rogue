use crate::prelude::*;
const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start : Point,
}

impl MapBuilder {
    /*
        This function obtains a mutable iterator with iter_mut() and then uses for_each()
        to change each tile into a wall. The asterisk (*) before t is a dereference.
        The iterator passes t (tile type) as a reference to an &TileType.
        DeReferencing indicates that we want to write to the referenced variable,
        not the reference itself.
     */
    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }


    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // Loop that continually generates rooms until NUM_ROOMS rooms exist.
        while self.rooms.len() < NUM_ROOMS {
            //This generates a randomly positioned room, with random sizes.
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH -10),
                rng.range(1, SCREEN_HEIGHT -10),
                rng.range(2,10,),
                rng.range(2,10),
            );

            let mut overlap = false;
            //The tests the new room against each previously placed room and flags it as overlapping if they intersect.
            // also  uses the Rect type that includes a function named for_each().
            // It runs the provided closure on every x/y coordinate inside the rectangle it represents.
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            //This verifies that the rooms don’t overlap and
            // that they’re within the map boundaries and sets their contents to floors.
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0
                        && p.y <SCREEN_HEIGHT
                    {
                        let idx = map_idx(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });

                self.rooms.push(room)
            }
        }
    }


    /*
    The basic map will use dog-leg corridors (corridors with a horizontal and vertical section),
    joined by a single corner.
    This next function creates a vertical tunnel between two points on the map:
    Range iterators expect that the starting value of a range is the minimum value, the destination the maximum.
    This function uses min() and max() to find the lowest and highest of a pair of values;
    the starting position, in this case.
    It then iterates y from the start to the end of the corridor,
    carving the tunnel along the way.
     */
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x:i32){
        use std::cmp::{min,max};
        for y in min(y1,y2) ..= max(y1,y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32){
        use std::cmp::{min,max};
        for x in min(x1,x2) ..= max(x1,x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x,y)){
                self.map.tiles[idx as usize] = TileType::Floor;
            }
        }
    }


    /*
    This function uses the last two functions to generate complete corridors between rooms
     Vectors include a sort_by() to sort their contents.
     It requires the closure of an inline function that calls the cmp() function on two elements of the vector’s contents.
     The cmp() function returns an indicator if two elements are the same, or if one is greater than the other.
     Sorting the rooms by their center point before allocating corridors makes it more likely that corridors will connect adjacent rooms without snaking across the whole map.
     The sort_by() function sends pairs of rooms to the closure.
     The closure receives these as a and b. a.center().x finds the x coordinate of room A.
      This is then compared with room B center using the cmp() function.
      This reorders the rooms to be sorted by the x order of their central points.
      Doing this shortens corridors between the rooms.
      If we don’t sort our rooms, we may receive very long corridors that likely overlap with other rooms.
     */
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a,b| a.center().x.cmp(&b.center().x));

        // The skip() function allows us to ignore some entries in the iterator. We’re ignoring the first one,
        for (i,room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let new = room.center();

            if rng.range(0,2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }



    /*
    The constructor calls all the elements we just created.
     */
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder{
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero()
        };
        mb.fill(TileType::Wall);
        mb.build_random_rooms(rng);
        mb.build_corridors(rng);
        //This sets player_start to the center of the first room in the rooms list.
        // This ensures that they start in a valid, walkable tile.
        mb.player_start = mb.rooms[0].center();
        mb
    }
}