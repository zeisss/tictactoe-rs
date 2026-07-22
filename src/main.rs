use std::fmt;
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers, MouseEvent, poll};
use crossterm::terminal::enable_raw_mode;
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line as TextLine, Span};
use ratatui::widgets::Paragraph;
use ratatui::widgets::canvas::{Canvas, Circle, Context, Line};

// Name: Waldemar, James and Stephan's TicTacToe

#[derive(Debug)]
struct App {
    mouse_events: i32,
}

impl App {
    fn default() -> Self {
        Self { mouse_events: 0 }
    }
    fn good_byte(&self) {
        println!("Saying goodbye after {} mouse events", self.mouse_events);
    }
}

const QUIT_KEY: KeyEvent = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::NONE);
fn update(app: &mut App, event: event::Event) -> bool {
    match event {
        event::Event::Mouse(_) => {
            app.mouse_events += 1;
        }
        event::Event::Key(key) if key == QUIT_KEY => {
            app.good_byte();
            return true
        }
        _ => {}
    }
    false
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

        let mut app: App = App::default();
        'outer:
        loop {
            terminal.draw(|f| render(f, &app))?;
            loop {
                let ev = event::read()?;
                if update(&mut app, ev) {
                    break 'outer;
                }
                if !poll(Duration::from_millis(48))? {
                    break
                }
            }

            // if event::read()?.is_key_press() {
            //     break Ok(());
            // }
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
    render_canvas(frame, area);
    render_sidebar(frame, sidebar, &app);
}

fn render_title(frame: &mut Frame, area: Rect) {
    let title = TextLine::from_iter([
        Span::from("Stephan, Waldemar and James' TicTacToe").bold(),
        Span::from(" (Press 'q' to quit)"),
    ]);
    frame.render_widget(title.centered(), area);
}

// Render the current game status and player turn in the sidebar.
fn render_sidebar(frame: &mut Frame, area: Rect, app: &App) {
    // let text = "Centered text\nwith multiple lines.\nCheck out the recipe!";
    let counter = fmt::format(format_args!("{}", app.mouse_events));

    let paragraph = Paragraph::new(counter)
        .style(Color::White)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

pub fn render_circle(ctx: &mut Context, x: i8, y: i8) {
    let x = (x * 6 + 3) as f64;
    let y = (y * 6 + 3) as f64;
    ctx.draw(&Circle {
        x: x,
        y: y,
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
pub fn render_canvas(frame: &mut Frame, area: Rect) {
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
            // .xxx.|.xxx.|.xxx.

            ctx.layer();

            render_circle(ctx, 0, 0);
            render_circle(ctx, 0, 2);
            render_circle(ctx, 1, 1);
            render_circle(ctx, 2, 0);
            render_circle(ctx, 2, 2);
            render_cross(ctx, 0, 1);
            render_cross(ctx, 1, 0);
            render_cross(ctx, 1, 2);
            render_cross(ctx, 2, 1);

            // // Draw the cross in the top right cell
            // ctx.draw(&Line{x1: 9.0, y1: 3.0, x2: 11.0, y2: 5.0, color: Color::Green});
            // ctx.draw(&Line{x1: 11.0, y1: 3.0, x2: 9.0, y2: 5.0, color: Color::Green});

            // ctx.draw(&Line::new(0.0, 10.0, 10.0, 10.0, Color::Blue));
            // ctx.draw(&Rectangle {
            //     x: 10.0,
            //     y: 20.0,
            //     width: 10.0,
            //     height: 10.0,
            //     color: Color::Green,
            // });
            // ctx.draw(&Points {
            //     coords: &[
            //         (2.3522, 48.8566),    // Paris
            //         (-122.3321, 47.6062), // Seattle
            //         (-79.3837, 43.6511),  // Toronto
            //         (32.8597, 39.9334),   // Ankara
            //     ],
            //     color: Color::Red,
            // });
        });

    frame.render_widget(canvas, area);
}
