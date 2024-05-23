use crate::Direction;

use piston::input::Button;
use piston::input::Key;

impl Direction {
    pub fn update_position(&self, position: &mut super::Segment) {
        match self {
            Direction::Up => {
                position.y -= 1.0;
            }
            Direction::Down => {
                position.y += 1.0;
            }
            Direction::Left => {
                position.x -= 1.0;
            }
            Direction::Right => {
                position.x += 1.0;
            }
        }

        if position.x < 0.0 {
            position.x = super::GRID_SIZE.0 as f64 - 1.0;
        } else if position.x >= super::GRID_SIZE.0 as f64 {
            position.x = 0.0;
        }

        if position.y < 0.0 {
            position.y = super::GRID_SIZE.1 as f64 - 1.0;
        } else if position.y >= super::GRID_SIZE.1 as f64 {
            position.y = 0.0;
        }
    }

    pub fn from_button(button: &Button) -> Option<Direction> {
        match button {
            &Button::Keyboard(Key::Up) => Some(Direction::Up),
            &Button::Keyboard(Key::Down) => Some(Direction::Down),
            &Button::Keyboard(Key::Left) => Some(Direction::Left),
            &Button::Keyboard(Key::Right) => Some(Direction::Right),
            _ => None,
        }
    }

    pub fn opposite_direction(&self)-> Direction {
        match self {
            Direction::Down=>Direction::Up,
            Direction::Up=>Direction::Down,
            Direction::Left=>Direction::Right,
            Direction::Right=>Direction::Left,
        }
    }
}


