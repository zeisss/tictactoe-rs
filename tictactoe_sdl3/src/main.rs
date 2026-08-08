use tictactoe_ratatui::tictactoe::{Cell, GameState, Outcome, PlaceError, Player};

use sdl3::event::Event;
use sdl3::keyboard::Keycode;
use sdl3::pixels::Color;
use sdl3::render::{Canvas, FPoint};
use sdl3::video::Window;
use std::time::Duration;

use sdl3::rect::Rect;

const LIGHT_GREY: Color = Color::RGB(150, 150, 150);
const DARK_GREY: Color = Color::RGB(50, 50, 50);
const BLACK: Color = Color::RGB(255, 255, 255);
const GREEN: Color = Color::RGB(0, 255, 0);
const RED: Color = Color::RGB(255, 0, 0);

const CELL_SIZE: i32 = 200;

struct App {
    quit: bool,
    board: GameState,

    canvas: Canvas<Window>,

    last_placement: std::result::Result<(), PlaceError>,
}

impl App {
    fn render(&mut self) {
        // Background color
        self.canvas.set_draw_color(LIGHT_GREY);
        self.canvas.clear();

        // Draw cells
        // 3x3 cells for our game board with a little border
        self.canvas.set_draw_color(DARK_GREY);
        // 200x200 per cell - 5 on each side
        for x in 0..3 {
            for y in 0..3 {
                self.canvas
                    .fill_rect(Rect::new(
                        x * CELL_SIZE + 5,
                        y * CELL_SIZE + 5,
                        (CELL_SIZE - 10) as u32,
                        (CELL_SIZE - 10) as u32,
                    ))
                    .unwrap();
            }
        }

        // Draw cells
        for x in 0..3 {
            for y in 0..3 {
                self.canvas.set_draw_color(BLACK);
                // Draw the cell:
                // Either the cross/naught or the keyboard shortcut to place the next token
                let middle = FPoint::new(
                    x as f32 * CELL_SIZE as f32 + 100.0,
                    y as f32 * CELL_SIZE as f32 + 100.0,
                );

                match self.board.get_cell((x as usize, y as usize)) {
                    Cell::PlayerOccupied(Player::Cross) => draw_cross_cell(&mut self.canvas, x, y),
                    Cell::PlayerOccupied(Player::Naught) => {
                        draw_naught_cell(&mut self.canvas, x, y)
                    } // TODO
                    Cell::Empty => {
                        let label = match (x, y) {
                            (0, 0) => "Q",
                            (1, 0) => "W",
                            (2, 0) => "E",

                            (0, 1) => "A",
                            (1, 1) => "S",
                            (2, 1) => "D",

                            (0, 2) => "Y|Z",
                            (1, 2) => "X",
                            (2, 2) => "C",
                            _ => "_",
                        };
                        self.canvas.draw_debug_text(label, middle)
                    }
                }
                .unwrap();
            }
        }

        // Draw status box
        const AREA_X: f32 = 600.0;
        match self.board.outcome {
            Some(outcome) => {
                let message = match outcome {
                    Outcome::NaughtWins => "Naught wins!",
                    Outcome::CrossWins => "Cross wins!",
                    Outcome::Draw => "Draw - no winner!",
                };
                self.canvas.draw_debug_text(message, (AREA_X, 10.0)).unwrap();
            },
            None => {
                self.canvas.set_draw_color(BLACK);

                self.canvas
                    .draw_debug_text("Active Player:".into(), (AREA_X, 10.0))
                    .unwrap();

                self.canvas.draw_debug_text(
                    &self.board.active_player.to_string(),
                    (AREA_X, 20.0),
                ).unwrap();

                if let Err(error) = self.last_placement {
                    self.canvas.set_draw_color(RED);
                    self.canvas.draw_debug_text(format!("Error: {:?}", error).as_str(), (AREA_X, 30.0));
                };
            }
            
        };

        self.canvas.present();
    }

    fn process_inputs(&mut self, event_pump: &mut sdl3::EventPump) {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.quit = true,

                // Cell inputs
                Event::KeyDown {keycode: Some(Keycode::Q), .. } => self.last_placement = self.board.place(0,0),
                Event::KeyDown {keycode: Some(Keycode::W), .. } => self.last_placement = self.board.place(1,0),
                Event::KeyDown {keycode: Some(Keycode::E), .. } => self.last_placement = self.board.place(2,0),

                Event::KeyDown {keycode: Some(Keycode::A), .. } => self.last_placement = self.board.place(0,1),
                Event::KeyDown {keycode: Some(Keycode::S), .. } => self.last_placement = self.board.place(1,1),
                Event::KeyDown {keycode: Some(Keycode::D), .. } => self.last_placement = self.board.place(2,1),

                Event::KeyDown {keycode: Some(Keycode::Y), .. } | Event::KeyDown {keycode: Some(Keycode::Z), .. } => self.last_placement = self.board.place(0,2),
                Event::KeyDown {keycode: Some(Keycode::X), .. } => self.last_placement = self.board.place(1,2),
                Event::KeyDown {keycode: Some(Keycode::C), .. } => self.last_placement = self.board.place(2,2),

                _ => {}
            }
        }
    }
}

fn draw_cross_cell(canvas: &mut Canvas<Window>, x: i32, y: i32) -> Result<(), sdl3::Error> {
    canvas.set_draw_color(RED);
    canvas.draw_line(
        (x * CELL_SIZE + 5, y * CELL_SIZE + 5),
        (x * CELL_SIZE + 190, y * CELL_SIZE + 190),
    )?;
    canvas.draw_line(
        (x * CELL_SIZE + 5, y * CELL_SIZE + 190),
        (x * CELL_SIZE + 190, y * CELL_SIZE + 5),
    )
}

fn draw_naught_cell(canvas: &mut Canvas<Window>, x: i32, y: i32) -> Result<(), sdl3::Error> {
    canvas.set_draw_color(GREEN);
    canvas.draw_line(
        (x * CELL_SIZE + 25, y * CELL_SIZE + 25),
        (x * CELL_SIZE + 25, y * CELL_SIZE + 175),
    )?;
    canvas.draw_line(
        (x * CELL_SIZE + 25, y * CELL_SIZE + 25),
        (x * CELL_SIZE + 175, y * CELL_SIZE + 25),
    )?;
    canvas.draw_line(
        (x * CELL_SIZE + 25, y * CELL_SIZE + 175),
        (x * CELL_SIZE + 175, y * CELL_SIZE + 175),
    )?;
    canvas.draw_line(
        (x * CELL_SIZE + 175, y * CELL_SIZE + 25),
        (x * CELL_SIZE + 175, y * CELL_SIZE + 175),
    )
}

pub fn main() {
    let sdl_context = sdl3::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("rust-sdl3 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut app = App {
        quit: false,
        board: GameState::default(),
        canvas: window.into_canvas(),
        last_placement: Ok(()),
    };

    app.board.place(0, 0);
    app.board.place(2, 1);

    println!("Hello, world!");
    println!("{:?}", app.board);

    app.render();

    let mut event_pump = sdl_context.event_pump().unwrap();
    while !app.quit {
        app.process_inputs(&mut event_pump);

        app.render();

        // The rest of the game loop goes here...

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
