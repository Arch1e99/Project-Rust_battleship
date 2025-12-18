use macroquad::prelude::Color;

pub const CELL_SIZE: f32 = 35.0;
pub const BOARD_SIZE: usize = 10;
pub const GAP: f32 = 80.0;
pub const MARGIN_TOP: f32 = 100.0;
pub const MARGIN_LEFT: f32 = 50.0;

pub const COL_WATER: Color = Color::new(0.0, 0.4, 0.8, 1.0);
pub const COL_SHIP: Color = Color::new(0.5, 0.5, 0.5, 1.0);
pub const COL_HIT: Color = Color::new(0.9, 0.1, 0.1, 1.0);
pub const COL_SUNK: Color = Color::new(0.1, 0.1, 0.1, 1.0);
pub const COL_MISS: Color = Color::new(0.2, 0.2, 0.2, 0.4);
pub const COL_GHOST: Color = Color::new(0.0, 1.0, 0.0, 0.5);
