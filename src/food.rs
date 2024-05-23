use rand::prelude::*;
use crate::SnakeSegment;

use super::{Segment, Food, FOOD_COLOR, GRID_SIZE};

impl Food {
    pub fn new() -> Food{
        let mut rng = rand::thread_rng();
        let current_position = Segment { x: rng.gen_range(0..GRID_SIZE.0) as f64, y: rng.gen_range(0..GRID_SIZE.1) as f64, color: FOOD_COLOR};
        let prev_position = Segment { x: 0.0, y: 0.0, color: FOOD_COLOR};

        let food = Food {
            current_position,
            prev_position,
        };
        return food;
    }

    pub fn random_food(&mut self, segments:Option<&Vec<SnakeSegment>>) {
        let prev_position_clone = self.prev_position.clone(); 
        let temp = self.current_position.clone();
        self.current_position = random_food_position(GRID_SIZE, prev_position_clone,segments);
        self.prev_position = temp;
    }
}

pub fn random_food_position(grid_size: (i16, i16), prev_position: Segment, segments: Option<&Vec<SnakeSegment>>) -> Segment {
    let mut rng = rand::thread_rng();

    loop {
        let x = rng.gen_range(0..grid_size.0) as f64;
        let y = rng.gen_range(0..grid_size.1) as f64;
        let new_position = Segment { x, y, color: FOOD_COLOR };

        let collision = segments.map_or(false, |s| {
            s.iter().any(|segment| {
                new_position.x == segment.segment.x && new_position.y == segment.segment.y
            })
        }) || (new_position.x == prev_position.x && new_position.y == prev_position.y);

        if !collision {
            return new_position;
        }
    }
}