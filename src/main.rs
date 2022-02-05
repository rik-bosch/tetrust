extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;
use sdl2::rect::Rect;
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
    fn update(&self, msg: Msg) -> Model {
        match msg {
            Msg::MoveBlock(dir) => {
                Model { block_pos: self.block_pos.move_in_dir(dir, 1), ..self.clone() }
            }
            Msg::Quit => {
                Model { running: false, ..self.clone() }
            }
        }
    }
}

trait Action {
    fn check_action(&self, event: &Event) -> Option<Msg>;
}

struct KeyDownAction {
    msg: Msg,
    action_keys: Vec<Keycode>,
}

impl Action for KeyDownAction {
    fn check_action(&self, event: &Event) -> Option<Msg> {
        if let Event::KeyDown { keycode: Some(pressed_key), .. } = event {
            if self.action_keys.contains(pressed_key) {
                return Some(self.msg);
            }
        }
        None
    }
}

struct QuitAction {
    msg: Msg,
}

impl Action for QuitAction {
    fn check_action(&self, event: &Event) -> Option<Msg> {
        match event {
            Event::Quit { .. } => Some(self.msg),
            _ => None,
        }
    }
}

struct View {
    canvas: sdl2::render::Canvas<Window>
}

impl View {
    fn update(&mut self, model: &Model) -> Vec<Box<dyn Action>> {
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.clear();
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(
            Rect::new(model.block_pos.x, model.block_pos.y, 100, 100)
        );
        self.canvas.present();

        vec![
            Box::new(QuitAction {
                msg: Msg::Quit
            }),
            Box::new(KeyDownAction{
                msg: Msg::Quit,
                action_keys: vec![Keycode::Escape]
            }),
            Box::new(KeyDownAction{
                msg: Msg::MoveBlock(Direction::Up),
                action_keys: vec![Keycode::Up]
            }),
            Box::new(KeyDownAction{
                msg: Msg::MoveBlock(Direction::Down),
                action_keys: vec![Keycode::Down]
            }),
            Box::new(KeyDownAction{
                msg: Msg::MoveBlock(Direction::Left),
                action_keys: vec![Keycode::Left]
            }),
            Box::new(KeyDownAction{
                msg: Msg::MoveBlock(Direction::Right),
                action_keys: vec![Keycode::Right]
            }),
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
    let mut actions: Vec<Box<dyn Action>> = view.update(&model);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            for action in actions.iter() {
                if let Some(msg) = action.check_action(&event) {
                    model = model.update(msg)
                }
                if !model.running {
                    break 'running;
                }
            }
        }
        actions = view.update(model);
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
