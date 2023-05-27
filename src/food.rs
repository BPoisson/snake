use ggez::graphics::{Color, Rect};
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::constants::{GRID_CELL_DIM, SCREEN_SIZE};

pub struct Food {
    pub rect: Rect,
    pub colour: Color
}

impl Food {
    pub fn new() -> Self {
        let position: (i32, i32) = Food::get_random_position();

        let mut rng: ThreadRng = rand::thread_rng();
        let r_val: f32 = rng.gen_range(0.0..=1.0);
        let g_val: f32 = rng.gen_range(0.0..=1.0);
        let b_val: f32 = rng.gen_range(0.0..=1.0);

        Food {
            rect: Rect::new_i32(position.0, position.1, GRID_CELL_DIM, GRID_CELL_DIM),
            colour: Color::new(r_val, g_val, b_val, 1.0)
        }
    }

    pub fn move_food(&mut self) {
        let food_pos: (i32, i32) = Food::get_random_position();
        let color: (f32, f32, f32) = Food::random_color();

        self.rect.x = food_pos.0 as f32;
        self.rect.y = food_pos.1 as f32;
        self.colour = Color::new(color.0, color.1, color.2, 1.0);
    }

    fn get_random_position() -> (i32, i32) {
        let mut rng = rand::thread_rng();

        let x: i32 = rng.gen_range(0..SCREEN_SIZE.0 as i32 / GRID_CELL_DIM) * GRID_CELL_DIM;
        let y: i32 = rng.gen_range(0..SCREEN_SIZE.1 as i32 / GRID_CELL_DIM) * GRID_CELL_DIM;

        (x, y)
    }

    fn random_color() -> (f32, f32, f32) {
        let mut rng = rand::thread_rng();
        let r_val: f32 = rng.gen_range(0.0..=1.0);
        let g_val: f32 = rng.gen_range(0.0..=1.0);
        let b_val: f32 = rng.gen_range(0.0..=1.0);

        (r_val, g_val, b_val)
    }
}