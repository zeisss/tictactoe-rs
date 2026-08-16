use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers, MouseEventKind, poll};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line as TextLine, Span, Text};
use ratatui::widgets::Paragraph;
use ratatui::widgets::canvas::{Canvas, Circle, Context, Line};

// Name: Waldemar, James and Stephan's TicTacToe

mod tictactoe;
use tictactoe::{GameState, PlaceError};

use crate::tictactoe::Outcome;

const QUIT_KEY: KeyEvent = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::SHIFT);

#[derive(Debug)]
struct App {
    mouse_events_ignored: i32,
    running: bool,
    state: GameState,
    last_place: std::result::Result<(), PlaceError>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            state: GameState::default(),
            mouse_events_ignored: 0,
            running: true,
            last_place: Ok(()),
        }
    }
}

impl App {
    fn goodbye(&mut self) {
        println!(
            "Saying goodbye after {} mouse events",
            self.mouse_events_ignored
        );
        self.running = false
    }

    fn update(&mut self, event: event::Event) {
        match event {
            event::Event::Mouse(ev)
                if ev.kind == MouseEventKind::Down(event::MouseButton::Left) =>
            {
                println!("Mouse clicked: {:?}", ev);
                self.mouse_events_ignored += 1;
                // self.state.place(0, 0);
            }
            event::Event::Key(key) if key == QUIT_KEY => {
                self.goodbye();
            }
            event::Event::Key(key) => match key.code {
                KeyCode::Char('q') => self.last_place = self.state.place(0, 2),
                KeyCode::Char('w') => self.last_place = self.state.place(1, 2),
                KeyCode::Char('e') => self.last_place = self.state.place(2, 2),

                KeyCode::Char('a') => self.last_place = self.state.place(0, 1),
                KeyCode::Char('s') => self.last_place = self.state.place(1, 1),
                KeyCode::Char('d') => self.last_place = self.state.place(2, 1),

                KeyCode::Char('y') | KeyCode::Char('z') => self.last_place = self.state.place(0, 0),
                KeyCode::Char('x') => self.last_place = self.state.place(1, 0),
                KeyCode::Char('c') => self.last_place = self.state.place(2, 0),
                _ => {}
            },
            _ => {}
        }
    }
}

fn main() -> Result<()> {
    color_eyre::install()?;

    ratatui::run(|terminal| {
        ratatui::crossterm::execute!(
            terminal.backend_mut(),
            ratatui::crossterm::terminal::EnterAlternateScreen
        )?;
        ratatui::crossterm::execute!(
            terminal.backend_mut(),
            ratatui::crossterm::event::EnableMouseCapture
        )?;
        terminal.show_cursor()?;

        let mut app: App = App::default();
        while app.running {
            terminal.draw(|f| render(f, &app))?;

            while app.running {
                let ev = event::read()?;
                app.update(ev);
                if !poll(Duration::from_millis(48))? {
                    break;
                }
            }
        }
        ratatui::crossterm::execute!(
            terminal.backend_mut(),
            ratatui::crossterm::event::DisableMouseCapture
        )?;
        ratatui::crossterm::execute!(
            terminal.backend_mut(),
            ratatui::crossterm::terminal::LeaveAlternateScreen
        )?;
        Ok(())
    })
}

/// Render the UI with a canvas widget.
fn render(frame: &mut Frame, app: &App) {
    let vertical = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
    let horizontal = Layout::horizontal([Constraint::Length(80), Constraint::Min(40)]).spacing(1);
    let [top, main] = frame.area().layout(&vertical);
    let [area, sidebar] = main.layout(&horizontal);

    render_title(frame, top);
    render_game_board(frame, area, &app.state);
    render_sidebar(frame, sidebar, app);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = TextLine::from_iter([
        Span::from("Stephan, Waldemar and James' TicTacToe").bold(),
        Span::from(" (Press 'Q' to quit)"),
    ]);
    frame.render_widget(title.centered(), area);
}

// Render the current game status and player turn in the sidebar.
fn render_sidebar(frame: &mut Frame, area: Rect, app: &App) {
    // let text = "Centered text\nwith multiple lines.\nCheck out the recipe!";
    let text: Span = {
        match app.state.outcome {
            Some(Outcome::PlayerWins(player, _)) => format!("Game over: {:?} won!", player).into(),
            Some(Outcome::Draw) => "Game Over: Draw!".into(),
            None => format!("Current player: {:?}", app.state.active_player).into(),
        }
    };
    let error_hint = {
        if app
            .last_place
            .is_err_and(|err| err == PlaceError::CellOccupied)
        {
            "\nCell already occupied".red()
        } else {
            "".into()
        }
    };
    // Paragraph::new(Line::from(vec!["Hello, ".into(), "world!".red()]));

    let lines = ratatui::text::Line::from(vec![text, error_hint]);
    let paragraph = Paragraph::new(Text::from(lines))
        .style(Color::White)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

pub fn render_circle(ctx: &mut Context, x: i8, y: i8) {
    let x = (x * 6 + 3) as f64;
    let y = (y * 6 + 3) as f64;
    ctx.draw(&Circle {
        x,
        y,
        radius: 2.0,
        color: Color::Red,
    });
}

pub fn render_cross(ctx: &mut Context, x: i8, y: i8) {
    let x = (x * 6 + 3) as f64;
    let y = (y * 6 + 3) as f64;
    ctx.draw(&Line {
        x1: x - 2.0,
        y1: y - 2.0,
        x2: x + 2.0,
        y2: y + 2.0,
        color: Color::Green,
    });
    ctx.draw(&Line {
        x1: x - 2.0,
        y1: y + 2.0,
        x2: x + 2.0,
        y2: y - 2.0,
        color: Color::Green,
    });
}

/// Renders the canvas widget with various shapes and a map.
pub fn render_game_board(frame: &mut Frame, area: Rect, state: &GameState) {
    let canvas = Canvas::default()
        .x_bounds([0.0, 17.0])
        .y_bounds([0.0, 17.0])
        .marker(Marker::Braille)
        .paint(|ctx| {
            ctx.draw(&Line {
                x1: 6.0,
                y1: 0.0,
                x2: 6.0,
                y2: 17.0,
                color: Color::Blue,
            });
            ctx.draw(&Line {
                x1: 12.0,
                y1: 0.0,
                x2: 12.0,
                y2: 17.0,
                color: Color::Blue,
            });

            ctx.draw(&Line {
                x1: 0.0,
                y1: 6.0,
                x2: 17.0,
                y2: 6.0,
                color: Color::Blue,
            });
            ctx.draw(&Line {
                x1: 0.0,
                y1: 12.0,
                x2: 17.0,
                y2: 12.0,
                color: Color::Blue,
            });

            ctx.layer();

            // Render Game board
            for x in 0..3 {
                for y in 0..3 {
                    match state.get_cell((x, y)) {
                        tictactoe::Cell::Empty => {
                            let label = match (x, y) {
                                (0, 2) => "q",
                                (1, 2) => "w",
                                (2, 2) => "e",

                                (0, 1) => "a",
                                (1, 1) => "s",
                                (2, 1) => "d",

                                (0, 0) => "y|z",
                                (1, 0) => "x",
                                (2, 0) => "c",
                                _ => "_",
                            };

                            ctx.print((x * 6 + 3) as f64, (y * 6 + 3) as f64, label);
                        }
                        tictactoe::Cell::PlayerOccupied(player) => match player {
                            tictactoe::Player::Nought => render_circle(ctx, x as i8, y as i8),
                            tictactoe::Player::Cross => render_cross(ctx, x as i8, y as i8),
                        },
                    }
                }
            }
        });

    frame.render_widget(canvas, area);
}
