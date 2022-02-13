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
    canvas_size: Vector2,
    running: bool,
}

fn init() -> (Model, Cmd) {
    (
        Model {
            block_pos: Vector2{ x: 0, y: 0 },
            canvas_size: Vector2{ x: 200, y: 200 },
            running: true
        },
        Cmd::GetCanvasSize
    )
}

enum Msg {
    MoveBlock(Direction),
    SetCanvasSize(Vector2),
    Quit
}

enum Cmd {
    None,
    GetCanvasSize
}

impl Model {
    fn update(self, msg: Msg) -> (Model, Cmd) {
        match msg {
            Msg::MoveBlock(dir) => {
                (Model { block_pos: self.block_pos.move_in_dir(dir, 1), ..self }, Cmd::None)
            }
            Msg::SetCanvasSize(canvas_size) => {
                (Model { canvas_size, ..self }, Cmd::None)
            }
            Msg::Quit => {
                (Model { running: false, ..self }, Cmd::None)
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
        // Clear screen.
        self.canvas.set_draw_color(Color::RGB(255, 255, 255));
        self.canvas.clear();

        // Draw border.
        self.canvas.set_draw_color(Color::RGB(0, 0, 0));
        self.canvas.draw_rect(
            sdl2::rect::Rect::new(10, 10, (model.canvas_size.x - 20) as u32, (model.canvas_size.y - 20) as u32)
        ).expect("Could not draw border rect");

        // Draw 'player' rect.
        self.canvas.set_draw_color(Color::RGB(255, 0, 0));
        self.canvas.fill_rect(
            sdl2::rect::Rect::new(model.block_pos.x, model.block_pos.y, 100, 100)
        ).expect("Could not fill rect for model.block_pos");

        // Do rendering.
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

fn parse_command(cmd: Cmd, model: Model, view: &View) -> Model {
    match cmd {
        Cmd::GetCanvasSize => {
            let (x, y) = view.canvas.output_size().expect("Could not get output size");
            let msg = Msg::SetCanvasSize(Vector2 { x: x as i32, y: y as i32 });
            let (new_model, new_cmd) = model.update(msg);
            parse_command(new_cmd, new_model, view)
        }
        _ => {
            model
        }
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let (mut model, mut cmd) = init();
    let mut view = View {
        canvas: window.into_canvas().build().unwrap()
    };
    model = parse_command(cmd, model, &view);
    let mut actions = view.update(&model);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            while let Some(action) = actions.pop() {
                if let Some(msg) = action(&event) {
                    (model, cmd) = model.update(msg);
                    model = parse_command(cmd, model, &view);
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
