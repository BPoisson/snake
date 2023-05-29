use std::time::{Duration, Instant};
use ggez::{Context, event, GameResult, graphics};
use ggez::glam::Vec2;
use ggez::graphics::{Canvas, Color, DrawMode, DrawParam};
use ggez::input::keyboard;
use ggez::input::keyboard::KeyCode;
use ggez::mint::Vector2;
use crate::constants::{MILLIS_PER_FRAME, SCREEN_SIZE};
use crate::food::Food;
use crate::snake::{Direction, Snake};
use crate::sounds::Sounds;

pub struct Game {
    snake: Snake,
    food: Food,
    sounds: Sounds,
    game_over: bool,
    last_update: Instant
}

impl Game {
    pub fn new(ctx: &Context) -> Self {
        Game {
            snake: Snake::new((SCREEN_SIZE.0 / 2) as i32, (SCREEN_SIZE.1 / 2) as i32),
            food: Food::new(),
            sounds: Sounds::new(ctx),
            game_over: false,
            last_update: Instant::now()
        }
    }

    fn handle_collision(ctx: &Context, snake: &mut Snake, food: &mut Food, sounds: &mut Sounds) -> bool {
        let snake_head = snake.body[0];

        if snake_head.x == food.rect.x && snake_head.y == food.rect.y {
            food.move_food();
            snake.grow();

            sounds.play_food_sound(ctx);
        }

        for i in 1..snake.body.len() {
            let segment = snake.body[i];
            if snake_head.x == segment.x && snake_head.y == segment.y {
                return false;
            }
        }
        return true;
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_FRAME) {
            if !&self.game_over {
                if keyboard::is_key_pressed(ctx, KeyCode::Up) && self.snake.direction != Direction::DOWN {
                    self.snake.direction = Direction::UP;
                } else if keyboard::is_key_pressed(ctx, KeyCode::Down) && self.snake.direction != Direction::UP {
                    self.snake.direction = Direction::DOWN;
                } else if keyboard::is_key_pressed(ctx, KeyCode::Left) && self.snake.direction != Direction::RIGHT {
                    self.snake.direction = Direction::LEFT;
                } else if keyboard::is_key_pressed(ctx, KeyCode::Right) && self.snake.direction != Direction::LEFT {
                    self.snake.direction = Direction::RIGHT;
                }
                self.snake.move_segments();
                self.last_update = Instant::now();
            } else {
                self.sounds.stop_music(ctx);
            }
        }

        self.sounds.play_music(ctx);

        if !Game::handle_collision(ctx, &mut self.snake, &mut self.food, &mut self.sounds) {
            self.game_over = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas: Canvas = Canvas::from_frame(ctx, Color::BLACK);

        for segment in self.snake.body.iter() {
            let node_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), *segment, self.snake.colour)?;
            canvas.draw(&node_mesh, DrawParam::default());
        }

        let food_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), self.food.rect, self.food.colour)?;
        canvas.draw(&food_mesh, DrawParam::default());

        if self.game_over {
            let text = graphics::Text::new("You Died.");
            canvas.draw(&text, DrawParam::default().dest(Vec2::new(275.0, 0.0)).scale(Vector2::from([2.0, 2.0])).color(Color::WHITE));
        }

        canvas.finish(ctx)?;
        Ok(())
    }
}