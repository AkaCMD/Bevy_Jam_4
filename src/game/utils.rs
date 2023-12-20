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
//     pub fn to_vector(self) -> Vec3 {
//         match self {
//             Direction::Up => Vec3::new(0.0, 1.0, 0.0),
//             Direction::Down => Vec3::new(0.0, -1.0, 0.0),
//             Direction::Left => Vec3::new(-1.0, 0.0, 0.0),
//             Direction::Right => Vec3::new(1.0, 0.0, 0.0),
//             Direction::None => Vec3::new(0.0, 0.0, 0.0),
//         }
//     }
// }

// Convert logic position in level to translation
pub fn logic_position_to_translation(logic_position: (usize, usize)) -> Vec3 {
    Vec3::new(
        logic_position.0 as f32 * SPRITE_SIZE + 720.0 / 4.0,
        logic_position.1 as f32 * (-SPRITE_SIZE) + 720.0 * (3.0 / 4.0),
        0.0,
    )
}

// Define a generic Stack struct
pub struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    // Create a new empty stack
    pub fn new() -> Stack<T> {
        Stack { items: Vec::new() }
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    // Get the size of the stack
    pub fn size(&self) -> usize {
        self.items.len()
    }

    // Push an item onto the stack
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }

    // Pop an item from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    // Peek at the top item of the stack without removing it
    pub fn peek(&self) -> Option<&T> {
        self.items.last()
    }

    // Clear the stack
    pub fn clear(&mut self) {
        while !self.is_empty() {
            self.pop();
        }
    }
}
