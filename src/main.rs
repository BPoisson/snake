use ggez;
use ggez::graphics::{Color, DrawMode, Rect, DrawParam};
use ggez::graphics;
use ggez::event;
use ggez::input::keyboard;
use ggez::audio::{Source, SoundSource};
use ggez::{Context, GameResult, ContextBuilder};
use ggez::event::KeyCode;
use rand::Rng;
use ggez::conf::{WindowSetup, WindowMode};
use std::time::{Instant, Duration};
use ggez::nalgebra::Point2;
use ggez::mint::Vector2;

const GAME_ID: &str = "Snake";
const AUTHOR: &str = "BPoisson";

const GRID_SIZE: (i32, i32) = (30, 30);
const GRID_CELL_DIM: i32 = 25;

const SCREEN_SIZE: (i32, i32) = (
    GRID_SIZE.0 * GRID_CELL_DIM,
    GRID_SIZE.1 * GRID_CELL_DIM
);

const FRAMES_PER_SECOND: f32 = 12.0;
const MILLIS_PER_FRAME: u64 = (1.0 / FRAMES_PER_SECOND * 1000.0) as u64;

#[derive(PartialEq)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
    NONE
}

struct Snake {
    body: Vec<Rect>,
    direction: Direction,
    speed: f32,
    colour: Color
}

impl Snake {
    pub fn new(x_pos: i32, y_pos: i32) -> Self {
        let head = Rect::new_i32(x_pos, y_pos, GRID_CELL_DIM, GRID_CELL_DIM);

        Snake {
            body: vec![head],
            direction: Direction::NONE,
            speed: 1.0,
            colour: graphics::WHITE
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
                head.y = clamp(head.x, y_pos).1;
            },
            Direction::DOWN => {
                let y_pos: f32 = head.y + GRID_CELL_DIM as f32 * &self.speed;
                head.y = clamp(head.x, y_pos).1;
            },
            Direction::LEFT => {
                let x_pos: f32 = head.x - GRID_CELL_DIM as f32 * &self.speed;
                head.x = clamp(x_pos, head.y).0;
            },
            Direction::RIGHT => {
                let x_pos = head.x + GRID_CELL_DIM as f32 * &self.speed;
                head.x = clamp(x_pos, head.y).0;
            },
            _ => ()
        }
    }

    pub fn grow(&mut self) {
        let last_segment = self.body[self.body.len() - 1];
        let segment = Rect::new_i32((last_segment.x + 1.0) as i32, last_segment.y as i32, GRID_CELL_DIM, GRID_CELL_DIM);
        self.body.push(segment);
    }
}

struct Food {
    rect: Rect,
    colour: Color
}

impl Food {
    pub fn new() -> Self {
        let position: (i32, i32) = get_random_position();

        let mut rng = rand::thread_rng();
        let r_val = rng.gen_range(0.0, 1.0);
        let g_val = rng.gen_range(0.0, 1.0);
        let b_val = rng.gen_range(0.0, 1.0);

        Food {
            rect: Rect::new_i32(position.0, position.1, GRID_CELL_DIM, GRID_CELL_DIM),
            colour: Color::new(r_val, g_val, b_val, 1.0)
        }
    }

    pub fn move_food(&mut self) {
        let food_pos: (i32, i32) = get_random_position();
        let color: (f32, f32, f32) = random_color();

        self.rect.x = food_pos.0 as f32;
        self.rect.y = food_pos.1 as f32;
        self.colour = Color::new(color.0, color.1, color.2, 1.0);
    }
}

struct MainState {
    snake: Snake,
    food: Food,
    food_sounds: Vec<Source>,
    music: Source,
    game_over: bool,
    last_update: Instant
}

impl MainState {
    pub fn new(ctx: &mut Context) -> Self {
        MainState {
            snake: Snake::new((SCREEN_SIZE.0 / 2) as i32, (SCREEN_SIZE.1 / 2) as i32),
            food: Food::new(),
            food_sounds: initialize_audio(ctx),
            music: Source::new(ctx, "\\sounds\\music.mp3").unwrap(),
            game_over: false,
            last_update: Instant::now()
        }
    }
}

impl event::EventHandler for MainState {
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
                self.music.stop();
            }
        }
        if !self.music.playing() {
            self.music.set_repeat(true);
            self.music.set_volume(0.5);
            self.music.play().unwrap();
        }
        if !handle_collision(&mut self.snake, &mut self.food, &mut self.food_sounds) {
            self.game_over = true;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        for segment in self.snake.body.iter() {
            let node_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), *segment, self.snake.colour)?;
            graphics::draw(ctx, &node_mesh, graphics::DrawParam::default())?;
        }

        let food_mesh = graphics::Mesh::new_rectangle(ctx, DrawMode::fill(), self.food.rect, self.food.colour)?;
        graphics::draw(ctx, &food_mesh, graphics::DrawParam::default())?;

        if self.game_over {
            let text = graphics::Text::new("You Died.");
            graphics::draw(ctx, &text, DrawParam::default().dest(Point2::new(275.0, 0.0)).scale(Vector2::from([2.0, 2.0])).color(graphics::WHITE))?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

fn handle_collision(snake: &mut Snake, food: &mut Food, food_sounds: &mut Vec<Source>) -> bool {
    let snake_head = snake.body[0];

    if snake_head.x == food.rect.x && snake_head.y == food.rect.y {
        food.move_food();
        snake.grow();

        let sound_index = rand::thread_rng().gen_range(0, food_sounds.len());
        food_sounds[sound_index].play_detached().unwrap();
    }

    for i in 1..snake.body.len() {
        let segment = snake.body[i];
        if snake_head.x == segment.x && snake_head.y == segment.y {
            return false;
        }
    }
    return true;
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

fn random_color() -> (f32, f32, f32) {
    let mut rng = rand::thread_rng();
    let r_val = rng.gen_range(0.0, 1.0);
    let g_val = rng.gen_range(0.0, 1.0);
    let b_val = rng.gen_range(0.0, 1.0);

    (r_val, g_val, b_val)
}

fn get_random_position() -> (i32, i32) {
    let mut rng = rand::thread_rng();

    let x: i32 = rng.gen_range(0, SCREEN_SIZE.0 as i32 / GRID_CELL_DIM) * GRID_CELL_DIM;
    let y: i32 = rng.gen_range(0, SCREEN_SIZE.1 as i32 / GRID_CELL_DIM) * GRID_CELL_DIM;

    (x, y)
}

fn initialize_audio(ctx: &mut Context) -> Vec<Source> {
    let food1: Source = Source::new(ctx, "\\sounds\\food1.mp3").unwrap();
    let food2: Source = Source::new(ctx, "\\sounds\\food2.mp3").unwrap();
    let food3: Source = Source::new(ctx, "\\sounds\\food3.mp3").unwrap();
    let food4: Source = Source::new(ctx, "\\sounds\\food4.mp3").unwrap();
    let food5: Source = Source::new(ctx, "\\sounds\\food5.mp3").unwrap();

    vec![food1, food2, food3, food4, food5]
}

fn main() -> GameResult {
    let (ctx, events_loop) = &mut ContextBuilder::new(&GAME_ID, &AUTHOR)
        .window_setup(WindowSetup::default().title(&GAME_ID))
        .window_mode(WindowMode::default().dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32))
        .add_resource_path("resources")
        .build()?;

    let mut state = MainState::new(ctx);

    return event::run(ctx, events_loop, &mut state)
}
