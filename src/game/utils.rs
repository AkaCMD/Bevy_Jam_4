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
