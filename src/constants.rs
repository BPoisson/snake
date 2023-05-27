pub const GRID_SIZE: (i32, i32) = (30, 30);
pub const GRID_CELL_DIM: i32 = 25;

pub const SCREEN_SIZE: (i32, i32) = (
    GRID_SIZE.0 * GRID_CELL_DIM,
    GRID_SIZE.1 * GRID_CELL_DIM
);

pub const FRAMES_PER_SECOND: f32 = 12.0;
pub const MILLIS_PER_FRAME: u64 = (1.0 / FRAMES_PER_SECOND * 1000.0) as u64;
