extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::video::Window;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn move_in_dir(&self, dir: Direction, length: i32) -> Vector2 {
        match dir {
            Direction::Up => {
                Vector2 { x: self.x, y: self.y - length }
            }
            Direction::Down => {
                Vector2 { x: self.x, y: self.y + length }
            }
            Direction::Left => {
                Vector2 { x: self.x - length, y: self.y }
            }
            Direction::Right => {
                Vector2 { x: self.x + length, y: self.y }
            }
        }
    }
}

struct Model {
    block_pos: Vector2,
    running: bool,
}

fn init() -> Model {
    Model {
        block_pos: Vector2{ x: 0, y: 0 },
        running: true
    }
}

enum Msg {
    MoveBlock(Direction),
    Quit
}

impl Model {
    fn update(self, msg: Msg) -> Model {
        match msg {
            Msg::MoveBlock(dir) => {
                Model { block_pos: self.block_pos.move_in_dir(dir, 1), ..self }
            }
            Msg::Quit => {
                Model { running: false, ..self }
            }
        }
    }
}

fn action_keydown(event: &Event, msg: Msg, action_keys: Vec<Keycode>) -> Option<Msg> {
    if let Event::KeyDown { keycode: Some(pressed_key), .. } = event {
        if action_keys.contains(pressed_key) {
            return Some(msg);
        }
    }
    None
}

fn action_quit(event: &Event, msg: Msg) -> Option<Msg> {
    match event {
        Event::Quit { .. } => Some(msg),
        _ => None,
    }
}

struct View {
    canvas: sdl2::render::Canvas<Window>
}

impl View {
    fn update(&mut self, model: &Model) -> Vec<Box<dyn FnOnce(&Event) -> Option<Msg>>> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(
            sdl2::rect::Rect::new(model.block_pos.x, model.block_pos.y, 100, 100)
        ).expect("Could not fill rect for model.block_pos");
        self.canvas.present();

        vec![
            Box::new(|event| action_quit(event, Msg::Quit)),
            Box::new(|event| action_keydown(event, Msg::Quit, vec![Keycode::Escape])),
            Box::new(|event| action_keydown(event, Msg::MoveBlock(Direction::Up), vec![Keycode::Up])),
            Box::new(|event| action_keydown(event, Msg::MoveBlock(Direction::Down), vec![Keycode::Down])),
            Box::new(|event| action_keydown(event, Msg::MoveBlock(Direction::Left), vec![Keycode::Left])),
            Box::new(|event| action_keydown(event, Msg::MoveBlock(Direction::Right), vec![Keycode::Right])),
        ]
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut model = init();
    let mut view = View {
        canvas: window.into_canvas().build().unwrap()
    };
    let mut actions = view.update(&model);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            while let Some(action) = actions.pop() {
                if let Some(msg) = action(&event) {
                    model = model.update(msg);
                    if !model.running {
                        break 'running;
                    }
                }
            }
        }
        actions = view.update(&model);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
