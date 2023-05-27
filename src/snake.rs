use ggez::graphics::{Color, Rect};
use crate::constants::{GRID_CELL_DIM, SCREEN_SIZE};

#[derive(PartialEq)]
pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE
}

pub struct Snake {
    pub body: Vec<Rect>,
    pub direction: Direction,
    pub speed: f32,
    pub colour: Color
}

impl Snake {
    pub fn new(x_pos: i32, y_pos: i32) -> Self {
        let head = Rect::new_i32(x_pos, y_pos, GRID_CELL_DIM, GRID_CELL_DIM);

        Snake {
            body: vec![head],
            direction: Direction::NONE,
            speed: 1.0,
            colour: Color::WHITE
        }
    }

    pub fn move_segments(&mut self) {
        for i in (1..self.body.len()).rev() {
            self.body[i].x = self.body[i - 1].x;
            self.body[i].y = self.body[i - 1].y;
        }

        let head = &mut self.body[0];
        match &self.direction {
            Direction::UP => {
                let y_pos = head.y - GRID_CELL_DIM as f32 * &self.speed;
                head.y = Snake::clamp(head.x, y_pos).1;
            },
            Direction::DOWN => {
                let y_pos: f32 = head.y + GRID_CELL_DIM as f32 * &self.speed;
                head.y = Snake::clamp(head.x, y_pos).1;
            },
            Direction::LEFT => {
                let x_pos: f32 = head.x - GRID_CELL_DIM as f32 * &self.speed;
                head.x = Snake::clamp(x_pos, head.y).0;
            },
            Direction::RIGHT => {
                let x_pos = head.x + GRID_CELL_DIM as f32 * &self.speed;
                head.x = Snake::clamp(x_pos, head.y).0;
            },
            _ => ()
        }
    }

    fn clamp(x_pos: f32, y_pos: f32) -> (f32, f32) {
        let mut result_x: f32 = x_pos;
        let mut result_y: f32 = y_pos;

        if x_pos < 0.0 {
            result_x = (SCREEN_SIZE.0 - GRID_CELL_DIM) as f32;
        } else if x_pos > (SCREEN_SIZE.0 - GRID_CELL_DIM) as f32 {
            result_x = 0.0
        }
        if y_pos < 0.0 {
            result_y = (SCREEN_SIZE.1 - GRID_CELL_DIM) as f32;
        } else if y_pos > (SCREEN_SIZE.1 - GRID_CELL_DIM) as f32 {
            result_y = 0.0;
        }
        return (result_x, result_y)
    }

    pub fn grow(&mut self) {
        let last_segment = self.body[self.body.len() - 1];
        let segment = Rect::new_i32((last_segment.x + 1.0) as i32, last_segment.y as i32, GRID_CELL_DIM, GRID_CELL_DIM);
        self.body.push(segment);
    }
}