use super::*;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    #[default]
    None,
}

// impl Direction {
//     pub fn to_vector(self) -> Vec2 {
//         match self {
//             Direction::Up => Vec2::new(0.0, 1.0),
//             Direction::Down => Vec2::new(0.0, -1.0),
//             Direction::Left => Vec2::new(-1.0, 0.0),
//             Direction::Right => Vec2::new(1.0, 0.0),
//             Direction::None => Vec2::new(0.0, 0.0),
//         }
//     }
// }

// Convert logic position in level to translation
pub fn logic_position_to_translation(logic_position: (usize, usize)) -> Vec3 {
    Vec3::new(
        logic_position.1 as f32 * SPRITE_SIZE + 720.0 / 4.0,
        logic_position.0 as f32 * (-SPRITE_SIZE) + 720.0 * (3.0 / 4.0),
        0.0,
    )
}

// Define a generic Stack struct
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    pub fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn size(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    pub fn clear(&mut self) {
        while !self.is_empty() {
            self.pop();
        }
    }
}
