extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;
extern crate find_folder;
extern crate rusttype;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics :: {GlGraphics, OpenGL, GlyphCache, TextureSettings};
use graphics::*;
use std::error::Error;

mod food;
mod direction;
mod snake;
mod logic;

const GRID_SIZE: (i16, i16) = (20, 20);
const CELL_SIZE: (i16, i16) = (30, 30);

const WINDOW_SIZE: (f64, f64) = (
    (GRID_SIZE.0 * CELL_SIZE.0) as f64, 
    (GRID_SIZE.1 * CELL_SIZE.1) as f64, 
);

const BACKGROUND_COLOR: [f32;4] = [0.235, 0.235, 0.235, 1.0];
const LINE_COLOR : [f32;4] = [0.663, 0.647, 0.655, 0.7];
const FOOD_COLOR: [f32;4] = [1.0, 0.0, 0.0, 1.0];
const SNAKE_HEAD_COLOR: [f32;4] = [0.8, 1.0, 1.0, 1.0];
const SNAKE_TAIL_COLOR: [f32;4] = [0.0, 1.0, 0.0, 1.0];
const GAME_WINDOW_COLOR: [f32;4] = [0.0, 0.0, 0.0, 0.4];
const OVERLAY_COLOR: [f32; 4] = [0.9, 0.9, 0.9, 0.1];
const SCORE_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
const FPS: u64 = 30;
const SHIFT: f64 = 60.0;
const UPDATE_TIMER: f64 = 0.1;

const OVERLAY_RECT: graphics::types::Rectangle = [
    SHIFT,
    SHIFT,
    WINDOW_SIZE.0,
    WINDOW_SIZE.1,
];

#[derive(Debug, Clone, Copy, PartialEq,Default)]
pub struct Segment {
    pub x: f64,
    pub y: f64,
    pub color: [f32;4],
}

impl Segment {
    pub fn set_viewport(&mut self) -> graphics::types::Rectangle{
        let x = (self.x * CELL_SIZE.0 as f64) + SHIFT;
        let y = (self.y * CELL_SIZE.1 as f64) + SHIFT;
        [x+2.5, y+2.5, CELL_SIZE.0 as f64-5.0, CELL_SIZE.1 as f64-5.0]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy)]
pub struct Food {
    current_position: Segment,
    prev_position: Segment,
}

#[derive(Clone,Debug,Copy, PartialEq, Default)]
pub struct SnakeSegment {
    segment: Segment,
    is_head: bool,
    direction: Option<Direction>,
    new_direction: Option<Direction>,
}

#[derive(Clone)]
pub struct Snake {
    segments: Vec<SnakeSegment>,
}

pub enum State {
    Playing, 
    Paused, 
    GameOver,
}

pub struct App {
    gl :GlGraphics,
    glyph_cache: GlyphCache<'static>,
    food: Food,
    score : i32,
    update_timer: f64,
    snake: Snake,
    state: State,
}

impl App {
    pub fn render(&mut self, args: &RenderArgs) {
        self.gl.draw(args.viewport(), |c, gl| {
            clear(BACKGROUND_COLOR, gl); 
            rectangle(GAME_WINDOW_COLOR, OVERLAY_RECT, c.transform, gl);

            for i in 0..GRID_SIZE.0+1{
                let  line_width = if i==0 || i==GRID_SIZE.0 { 1.0} else { 0.4};
                let x = (i * CELL_SIZE.0) as f64;
                line(
                    LINE_COLOR, 
                    line_width,
                    [x+SHIFT, SHIFT, x+SHIFT, WINDOW_SIZE.1+SHIFT],
                    c.transform,
                    gl,
                );
            }

            for i in 0..GRID_SIZE.1+1{
                let  line_width = if i==0 || i==GRID_SIZE.1 { 1.0} else { 0.4};
                let y = (i * CELL_SIZE.1) as f64;
                line(
                    LINE_COLOR, 
                    line_width,
                    [SHIFT, y+SHIFT, WINDOW_SIZE.0+SHIFT, y+SHIFT],
                    c.transform,
                    gl,
                );
            }

            rectangle(self.food.current_position.color, self.food.current_position.set_viewport(), c.transform, gl);
            
            for i in 0..self.snake.segments.len() {
                rectangle(self.snake.segments[i].segment.color, self.snake.segments[i].segment.set_viewport(),c.transform,gl);
            }

            let score_text = format!("Score: {}", self.score);
            let transform = c.transform.trans(WINDOW_SIZE.0/2.0, 40.0);
            text(
                SCORE_COLOR,
                32,
                &score_text,
                &mut self.glyph_cache,
                transform,
                gl,
            ).unwrap();

            match self.state {
                State::GameOver => {
                    rectangle(OVERLAY_COLOR, OVERLAY_RECT, c.transform, gl);
                    let game_over_text = "Game Over! Press Enter to Restart";
                    let transform = c.transform.trans(WINDOW_SIZE.0 / 2.0 - 210.0, WINDOW_SIZE.1 / 2.0 + 60.0);
                    text(
                        SCORE_COLOR,
                        32,
                        game_over_text,
                        &mut self.glyph_cache,
                        transform,
                        gl,
                    ).unwrap();
                }
                State::Paused => {
                    rectangle(OVERLAY_COLOR, OVERLAY_RECT, c.transform, gl);
                    let paused_text = "Paused! Press Space to Continue";
                    let transform = c.transform.trans(WINDOW_SIZE.0 / 2.0 - 200.0, WINDOW_SIZE.1 / 2.0 + 60.0);
                    text(
                        SCORE_COLOR,
                        32,
                        paused_text,
                        &mut self.glyph_cache,
                        transform,
                        gl,
                    ).unwrap();
                }
                _ => {}
            }

        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        if let State::Playing = self.state{
        self.update_timer += args.dt;
        if self.update_timer >= UPDATE_TIMER {
           //if self.is_key_pressed{
                if let Some(direction) = self.snake.segments[0].new_direction.clone() {
                    self.snake.move_snake(direction,&mut self.food, &mut self.score, &mut self.state);
                    self.update_timer = 0.0;
               // }
           }
        }
     }
    }  

    pub fn press(&mut self, args: &Button) {
        match self.state{
            State::Playing => {
                if let Some(direction) = Direction::from_button(args) {
                    self.snake.segments[0].new_direction = Some(direction);
                    self.update_timer = 0.0;
                    }
                
                if let Button::Keyboard(Key::Space) = args {
                    self.state = State::Paused;
                }
                
                }

            State::Paused => {
                if let Button::Keyboard(Key::Space) = args {
                    self.state = State::Playing;
                }
            }

            State::GameOver => {
                if let Button::Keyboard(Key::Return) = args {
                    self.reset();
                    self.state = State::Playing;
                }
            }
        }
    }

    fn reset(&mut self) {
        self.snake = Snake::new();
        self.food = Food::new();
        self.score = 0;
        self.update_timer = 0.0;
        self.state = State::Playing;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let opengl = OpenGL::V3_2;

    let mut window: Window = WindowSettings::new(
        "Snake Game",
        [WINDOW_SIZE.0+100.0,WINDOW_SIZE.1+100.0],
    )
    .graphics_api(opengl)
    .exit_on_esc(true)
    .build()?;

    let assets = find_folder::Search::ParentsThenKids(3, 3)
    .for_folder("assets")?;

    let font_path = assets.join("FiraSans-Regular.ttf");

    let glyph_cache = GlyphCache::new(font_path, (), TextureSettings::new())?;
   
    let mut app = App {
        gl: GlGraphics::new(opengl),
        glyph_cache,
        food: Food::new(),
        score: 0,
        update_timer: 0.0,
        snake: Snake::new(),
        state: State::Playing,
    };

    let mut events = Events::new(EventSettings::new());
    events.set_max_fps(FPS);
    while let Some(event) = events.next(&mut window) {
        if let Some(args) = event.render_args() {
            app.render(&args);
        }

        if let Some(Button::Keyboard(key)) = event.press_args() {
            app.press(&Button::Keyboard(key));
        }

        if let Some(args) = event.update_args() {
            app.update(&args);
        }
    }
    Ok(())
}
