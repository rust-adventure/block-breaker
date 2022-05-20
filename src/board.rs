use bevy::prelude::*;
pub const TILE_X_SIZE: f32 = 80.0;
pub const TILE_Y_SIZE: f32 = 40.0;

// const Level = Board::new(11, 28);
#[derive(Debug, Clone)]
pub struct Size {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone)]
pub struct Physical {
    pub x: f32,
    pub y: f32,
}
#[derive(Debug, Clone)]
pub struct Board {
    pub size: Size,
    pub physical: Physical,
}
pub enum Axis {
    X,
    Y,
}

impl Board {
    pub fn new(x_size: u8, y_size: u8) -> Self {
        let physical_x_size = f32::from(x_size)
            * TILE_X_SIZE
            + f32::from(x_size + 1);
        let physical_y_size = f32::from(y_size)
            * TILE_Y_SIZE
            + f32::from(y_size + 1);
        Board {
            size: Size {
                x: x_size,
                y: y_size,
            },
            physical: Physical {
                x: physical_x_size,
                y: physical_y_size,
            },
        }
    }
    pub fn u8_cell_to_physical(
        &self,
        pos: u8,
        axis: Axis,
    ) -> f32 {
        let (physical_size, tile_size) = match axis {
            Axis::X => (self.physical.x, TILE_X_SIZE),
            Axis::Y => (self.physical.y, TILE_Y_SIZE),
        };
        let offset = -physical_size / 2.0 + 0.5 * tile_size;

        offset
            + f32::from(pos) * tile_size
            + f32::from(pos + 1)
    }
}

#[derive(
    Debug, PartialEq, Copy, Clone, Eq, Hash, Component,
)]
struct Position {
    x: u8,
    y: u8,
}
