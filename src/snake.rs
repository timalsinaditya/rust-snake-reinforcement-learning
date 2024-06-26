use crate::{Direction, Food, Segment, Snake, SnakeSegment, State, SNAKE_HEAD_COLOR, SNAKE_TAIL_COLOR};
use crate::logic::{eat_food, game_over};

impl SnakeSegment {
    pub fn new(segment: Segment, is_head: bool, direction: Option<Direction>, new_direction: Option<Direction>) -> Self {
        SnakeSegment{
            segment, 
            is_head,
            direction,
            new_direction,
        }
    }
}

impl Snake {
    pub fn new() -> Self {
        let initial_segment = SnakeSegment::new(
            Segment{x: 0.0, y:0.0, color:SNAKE_HEAD_COLOR},
            true,
            None,
            None,
        );
        Snake {
            segments: vec![initial_segment],
        }
    }

    pub fn move_snake(&mut self,mut direction: Direction,food: &mut Food, score: &mut i32, state: &mut State) {
        if self.segments[0].direction == Some(direction.opposite_direction()) {
            direction = self.segments[0].direction.unwrap();
        }
            let len = self.segments.len();
            for i in (1..len).rev() {
                let haina = self.segments[i-1].direction;
                self.segments[i].direction = haina;
            }
            for i in 0..self.segments.len() {
                if !self.segments[i].is_head {
                    let curr_direction = self.segments[i].direction.unwrap();
                    curr_direction.update_position(&mut self.segments[i].segment);
                }
                else {
                    let help = self.segments.last().unwrap().clone();
                    direction.update_position(&mut self.segments[0].segment);
                    self.segments[0].direction=Some(direction);   
                    if eat_food(self.segments[0].segment,food.current_position) && !game_over(&self.segments) {
                        food.random_food(Some(&self.segments));
                        self.add_segment(help);
                        *score += 1;
                    }
                }
            }
            if game_over(&self.segments) {
                *state = State::GameOver;
            }
    }

    pub fn add_segment(&mut self, mut new_tail:SnakeSegment) {
        new_tail.is_head=false;
        new_tail.segment.color=SNAKE_TAIL_COLOR;
        self.segments.push(new_tail);
    }
}
