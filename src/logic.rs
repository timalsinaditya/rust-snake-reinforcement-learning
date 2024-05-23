use crate::{Segment, SnakeSegment};

pub fn eat_food(head:Segment,food:Segment) -> bool {
    if head.x == food.x && head.y == food.y {
        true
    }
    else {
        false
    }
}

pub fn game_over(segments:&Vec<SnakeSegment>) -> bool {
    let head = &segments[0].segment;
    for part in segments.iter().skip(1) {
        if part.segment.x == head.x && part.segment.y == head.y {
            return true;
        }
    }
    false
}