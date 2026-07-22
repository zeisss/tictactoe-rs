use std::fmt;
use std::time::Duration;

use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers, MouseEventKind, poll};
use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Stylize};
use ratatui::symbols::Marker;
use ratatui::text::{Line as TextLine, Span};
use ratatui::widgets::Paragraph;
use ratatui::widgets::canvas::{Canvas, Circle, Context, Line};

// Name: Waldemar, James and Stephan's TicTacToe

mod tictactoe {
    use std::result::{Result, Result::Ok, Result::Err};

    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Player {
        Naught,
        Cross,
    }
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Outcome {
        NaughtWins,
        CrossWins,
        Draw,
    }
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum Cell {
        Empty,
        PlayerOccupied(Player),
    }
    #[derive(Debug, Copy, Clone, PartialEq)]
    pub enum PlaceError {
        InvalidCoordinates,
        CellOccupied,
        GameOver,
    }

    #[derive(Debug)]
    pub struct GameState {
        pub active_player: Player,
        pub outcome: Option<Outcome>,
        pub board: [[Cell; 3]; 3],
    }

    impl GameState {
        pub fn default() -> Self {
            Self {
                active_player: Player::Naught,
                outcome: None,
                board: [[Cell::Empty; 3]; 3],
            }
        }

        pub fn place(&mut self, x: usize, y: usize) -> Result<(), PlaceError> {
            if x > 2 || y > 2 {
                return Err(PlaceError::InvalidCoordinates);
            }
            if self.outcome.is_some() {
                return Err(PlaceError::GameOver)
            }

            let cell = self.board[x][y];
            match cell {
                Cell::PlayerOccupied(_) => {
                    return Err(PlaceError::CellOccupied);
                }
                Cell::Empty => {
                    self.board[x][y] = Cell::PlayerOccupied(self.active_player);
                }
            }

            // Swap active player
            self.active_player = match self.active_player {
                Player::Naught => Player::Cross,
                Player::Cross => Player::Naught,
            };

            // check wincondition
            if let Some(outcome) = self.check_wincondition() {
                self.outcome = Some(outcome);
            }
            Ok(())
        }

        fn check_wincondition(&self) -> Option<Outcome> {
            // Check rows
            for row in 0..3 {
                if let Cell::PlayerOccupied(player) = self.board[row][0] {
                    if self.board[row][1] == Cell::PlayerOccupied(player)
                        && self.board[row][2] == Cell::PlayerOccupied(player)
                    {
                        return Some(match player {
                            Player::Naught => Outcome::NaughtWins,
                            Player::Cross => Outcome::CrossWins,
                        });
                    }
                }
            }

            // Check columns
            for col in 0..3 {
                if let Cell::PlayerOccupied(player) = self.board[0][col] {
                    if self.board[1][col] == Cell::PlayerOccupied(player)
                        && self.board[2][col] == Cell::PlayerOccupied(player)
                    {
                        return Some(match player {
                            Player::Naught => Outcome::NaughtWins,
                            Player::Cross => Outcome::CrossWins,
                        });
                    }
                }
            }

            // Check diagonals
            if let Cell::PlayerOccupied(player) = self.board[0][0] {
                if self.board[1][1] == Cell::PlayerOccupied(player)
                    && self.board[2][2] == Cell::PlayerOccupied(player)
                {
                    return Some(match player {
                        Player::Naught => Outcome::NaughtWins,
                        Player::Cross => Outcome::CrossWins,
                    });
                }
            }

            if let Cell::PlayerOccupied(player) = self.board[0][2] {
                if self.board[1][1] == Cell::PlayerOccupied(player)
                    && self.board[2][0] == Cell::PlayerOccupied(player)
                {
                    return Some(match player {
                        Player::Naught => Outcome::NaughtWins,
                        Player::Cross => Outcome::CrossWins,
                    });
                }
            }

            // Check for draw
            if self
                .board
                .iter()
                .all(|row| row.iter().all(|&cell| cell != Cell::Empty))
            {
                return Some(Outcome::Draw);
            }

            None
        }
    }

    #[test]
    fn test_game_state() {
        let mut game = GameState::default();
        assert_eq!(game.outcome, None);
        game.place(0, 0); // Naught
        game.place(1, 0); // Cross
        game.place(0, 1); // Naught
        game.place(1, 1); // Cross
        game.place(0, 2); // Naught wins
        assert_eq!(game.outcome, Some(Outcome::NaughtWins));
    }

    #[test]
    fn test_game_state_cross_wins() {
        let mut game = GameState::default();
        assert_eq!(game.outcome, None);
        game.place(0, 0); // Naught
        game.place(1, 0); // Cross
        game.place(0, 1); // Naught
        game.place(1, 1); // Cross
        game.place(2, 2); // Naught
        game.place(1, 2); // Cross wins
        assert_eq!(game.outcome, Some(Outcome::CrossWins));
    }

    #[test]
    fn test_game_state_draw() {
        let mut game = GameState::default();
        assert_eq!(game.outcome, None);

        game.place(0, 0); // Naught
        game.place(1, 1); // Cross
        game.place(2, 2); // Naught
        game.place(0, 2); // Cross
        game.place(2, 0); // Naught
        game.place(2, 1); // Cross
        game.place(1, 2); // Naught
        game.place(1, 0); // Cross
        game.place(0, 1); // Naught -> Draw

        assert_eq!(game.outcome, Some(Outcome::Draw));
    }
}

use tictactoe::GameState;

const QUIT_KEY: KeyEvent = KeyEvent::new(KeyCode::Char('q'), KeyModifiers::SHIFT);



#[derive(Debug)]
struct App {
    mouse_events_ignored: i32,
    running: bool,
    state: GameState,
}

impl App {
    fn default() -> Self {
        Self {
            mouse_events_ignored: 0,
            running: true,
            state: GameState::default(),
        }
    }

    fn goodbye(&mut self) {
        println!("Saying goodbye after {} mouse events", self.mouse_events_ignored);
        self.running = false
    }

    fn update(&mut self, event: event::Event) {
        match event {
            event::Event::Mouse(ev) if ev.kind == MouseEventKind::Down(event::MouseButton::Left) => {
                println!("Mouse clicked: {:?}", ev);
                self.mouse_events_ignored += 1;
                // self.state.place(0, 0);
            }
            event::Event::Key(key) if key == QUIT_KEY => {
                self.goodbye();
            }
            event::Event::Key(key) => {
                match key.code {
                KeyCode::Char('q')=> _ = self.state.place(0,2),
                KeyCode::Char('w')=> _ = self.state.place(1,2),
                KeyCode::Char('e')=> _ = self.state.place(2,2),

                KeyCode::Char('a')=> _ = self.state.place(0,1),
                KeyCode::Char('s')=> _ = self.state.place(1,1),
                KeyCode::Char('d')=> _ = self.state.place(2,1),

                KeyCode::Char('y') | KeyCode::Char('z') => _ = self.state.place(0,0),
                KeyCode::Char('x')=> _ = self.state.place(1,0),
                KeyCode::Char('c')=> _ = self.state.place(2,0),
                _ => {},
                }
            }
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
            terminal.draw(|f| render(f, &app) )?;
            

            while app.running {
                let ev = event::read()?;
                app.update(ev);
                if !poll(Duration::from_millis(48))? {
                    break;
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
    render_game_board(frame, area, &app.state);
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
    let text : String = {
        if app.state.outcome.is_some() {
            format!("Game over! Outcome: {:?}", app.state.outcome.unwrap())
        } else {
            format!("Current player: {:?}", app.state.active_player)
        }
    };
    let paragraph = Paragraph::new(text)
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
            // .xxx.|.xxx.|.xxx.

            ctx.layer();

            // Render Game board
            for x in 0..3 {
                for y in 0..3 {
                    match state.board[x][y] {
                        tictactoe::Cell::Empty => {
                            ctx.print((x * 6 + 3) as f64, (y * 6 + 3) as f64, format!("{},{}", x, y));
                        }
                        tictactoe::Cell::PlayerOccupied(player) => match player {
                            tictactoe::Player::Naught => render_circle(ctx, x as i8, y as i8),
                            tictactoe::Player::Cross => render_cross(ctx, x as i8, y as i8),
                        },
                    }
                }
            }
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
