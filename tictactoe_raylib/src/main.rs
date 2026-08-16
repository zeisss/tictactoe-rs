use raylib::prelude::*;

use tictactoe_ratatui::tictactoe::{Cell, GameState, Outcome, PlaceError, Player, WinCombination};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("TicTacToe in Rust + Raylib")
        .build();
    rl.set_target_fps(30);

    let key_bindings = KeyBinding::from_raylib_handle(&mut rl);
    let mut game = GameState::default();
    let mut last_placement = Ok(());

    while !rl.window_should_close() {
        // NOTE: Quitting the application via Escape is handled by raylib itself, no need for a keybinding for now
        match key_bindings.get_action(&mut rl) {
            Some(Action::PlaceToken(x, y)) => last_placement = game.place(x, y),
            Some(Action::ResetBoard) => {
                game = GameState::default();
                last_placement = Ok(());
            }
            None => {}
        }

        // Render
        rl.draw(&thread, |mut d| {
            d.clear_background(Color::PINK); // pink to check everything is covered
            draw_game_board(&mut d, &game, &key_bindings);
            draw_side_panel(&mut d, &game, last_placement, &key_bindings);
        });
    }
}

// UI Actions
enum Action {
    PlaceToken(usize, usize),
    ResetBoard,
}

struct KeyBinding {
    positions: [KeyboardKey; 9],
    labels: [String; 9],

    reset_key: KeyboardKey,
    reset_name: String,
}

impl KeyBinding {
    fn from_raylib_handle(rl: &mut RaylibHandle) -> KeyBinding {
        let positions: [KeyboardKey; 9] = [
            KeyboardKey::KEY_Q,
            KeyboardKey::KEY_W,
            KeyboardKey::KEY_E,
            KeyboardKey::KEY_A,
            KeyboardKey::KEY_S,
            KeyboardKey::KEY_D,
            KeyboardKey::KEY_Z, // american keyboard layout, we map this for DE layouts
            KeyboardKey::KEY_X,
            KeyboardKey::KEY_C,
        ];

        // Lookup the actual text on the phsyical keyboard button
        let mut labels = vec![];
        for key in positions {
            labels.push(rl.get_key_name(key).unwrap_or("?".into()));
        }

        let fixed_labels = labels.try_into().unwrap_or_else(|v: Vec<String>| {
            panic!("Expected a Vec of length {} but it was {}", 9, v.len())
        });
        KeyBinding {
            positions,
            labels: fixed_labels,
            reset_key: KeyboardKey::KEY_R,
            reset_name: rl.get_key_name(KeyboardKey::KEY_R).unwrap_or("?".into()),
        }
    }

    fn get_name(&self, key: KeyboardKey) -> String {
        if let Some(pos) = self.positions.iter().position(|k| key == *k) {
            self.labels[pos].to_uppercase().clone()
        } else {
            "?".into()
        }
    }

    fn get_name_for_position(&self, x: usize, y: usize) -> String {
        assert!(x <= 2);
        assert!(y <= 2);
        return self.get_name(self.positions[x + y * 3]);
    }

    fn key_to_position(&self, key: KeyboardKey) -> Option<(usize, usize)> {
        if let Some(pos) = self.positions.iter().position(|k| key == *k) {
            let r = pos % 3;
            Some((r, (pos - r) / 3))
        } else {
            None
        }
    }

    fn get_action(&self, rl: &mut RaylibHandle) -> Option<Action> {
        if rl.is_key_pressed(self.reset_key) {
            Some(Action::ResetBoard)
        } else if let Some(key) = rl.get_key_pressed() {
            let pos = self.key_to_position(key);
            if let Some(p) = pos {
                Some(Action::PlaceToken(p.0, p.1))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn player_color(p: Player) -> Color {
    match p {
        Player::Cross => Color::RED,
        Player::Naught => Color::GREEN,
    }
}
const TEXT_COLOR: Color = Color::BLACK;
const KEY_COLOR: Color = Color::DARKGRAY;
const ERROR_COLOR: Color = Color::DARKRED;
const BACKGROUND_LIGHT: Color = Color::LIGHTGRAY;
const BACKGROUND_DARK: Color = Color::GRAY;
const BACKGROUND_SIDE: Color = Color::WHITESMOKE;
const LINE_COLOR: Color = Color::BLACK;

fn draw_side_panel(
    d: &mut RaylibDrawHandle,
    game: &GameState,
    last_placement: Result<(), PlaceError>,
    key_bindings: &KeyBinding,
) {
    // Background + Separator Line to board
    d.draw_rectangle(600, 0, 200, 600, BACKGROUND_SIDE);
    d.draw_line(600, 0, 600, 600, LINE_COLOR);
    d.draw_line(601, 0, 601, 600, LINE_COLOR);

    let mut y = 10;
    // IF last_placement has an error, we need to show it
    if let Err(err) = last_placement {
        let message = format!("ERROR:\n{:?}", err);
        d.draw_text(&message, 610, y, 20, ERROR_COLOR);

        y += 50;
    }

    if let Some(outcome) = game.outcome {
        // Game is over, just render the winner / outcome
        let message = match outcome {
            Outcome::Draw => "Draw - no winner",
            Outcome::PlayerWins(Player::Cross, _) => "Cross won!",
            Outcome::PlayerWins(Player::Naught, _) => "Naught won!",
        };

        d.draw_text(message, 610, y, 20, TEXT_COLOR);
    } else {
        // No winner yet, show active player
        let message = format!("{}", game.active_player);
        d.draw_text("Active Player", 610, y, 20, TEXT_COLOR);
        d.draw_text(&message, 610, y + 30, 20, player_color(game.active_player));
    }

    // Draw footer with keybindings and help text
    let x = 610;
    let mut y = 470;
    d.draw_text(
        "Press key for field\nto place your token",
        x,
        y,
        16,
        TEXT_COLOR,
    );

    y = 520;
    draw_key_box(
        d,
        Rectangle::new(x as f32, y as f32, 30.0, 30.0),
        key_bindings.reset_name.to_uppercase(),
        10,
        KEY_COLOR,
    );
    d.draw_text("Restart game", x + 40, y + 5, 20, TEXT_COLOR);

    y = 560;
    draw_key_box(
        d,
        Rectangle::new(x as f32, y as f32, 30.0, 30.0),
        "Esc".into(),
        10,
        KEY_COLOR,
    );
    d.draw_text("Quit app", x + 40, y + 5, 20, TEXT_COLOR);
}

fn draw_game_board(d: &mut RaylibDrawHandle, game: &GameState, key_bindings: &KeyBinding) {
    // Check if the game is over and copy the win combination
    let win_combination: Option<WinCombination> =
        if let Some(Outcome::PlayerWins(_, combination)) = game.outcome {
            Some(combination)
        } else {
            None
        };

    const MARGIN: i32 = 20;
    const SIZE: i32 = 200;

    // Draw checker board
    {
        for x in 0..=2 {
            for y in 0..=2 {
                let color = if let Some(pos) = win_combination
                    && ((x, y) == pos.0 || (x, y) == pos.1 || (x, y) == pos.2)
                {
                    Color::SKYBLUE
                } else if (x + y * 3) % 2 == 0 {
                    BACKGROUND_DARK
                } else {
                    BACKGROUND_LIGHT
                };
                d.draw_rectangle(x as i32 * SIZE, y as i32 * SIZE, SIZE, SIZE, color);
            }
        }
    }

    // Draw lines to separate tiles / cells
    {
        d.draw_line(0, SIZE, SIZE * 3, SIZE, LINE_COLOR);
        d.draw_line(0, SIZE * 2, SIZE * 3, SIZE * 2, LINE_COLOR);

        d.draw_line(SIZE, 0, SIZE, SIZE * 3, LINE_COLOR);
        d.draw_line(SIZE * 2, 0, SIZE * 2, SIZE * 3, LINE_COLOR);
    }

    // Draw a 3x3 grid where each cell is 200x200 (Matching the window height of 600)
    for x in 0..=2 {
        for y in 0..=2 {
            let cell = game.get_cell((x, y));

            match cell {
                Cell::Empty => {
                    draw_key_box(
                        d,
                        Rectangle::new(
                            (x as i32 * SIZE + 100 - 30) as f32,
                            (y as i32 * SIZE + 100 - 30) as f32,
                            60.0,
                            60.0,
                        ),
                        key_bindings.get_name_for_position(x, y),
                        30,
                        KEY_COLOR,
                    );
                }
                Cell::PlayerOccupied(Player::Cross) => {
                    d.draw_line(
                        x as i32 * SIZE + MARGIN,
                        y as i32 * SIZE + MARGIN,
                        x as i32 * SIZE + SIZE - MARGIN,
                        y as i32 * SIZE + SIZE - MARGIN,
                        player_color(Player::Cross),
                    );
                    d.draw_line(
                        x as i32 * SIZE + SIZE - MARGIN,
                        y as i32 * SIZE + MARGIN,
                        x as i32 * SIZE + MARGIN,
                        y as i32 * SIZE + SIZE - MARGIN,
                        player_color(Player::Cross),
                    );
                }
                Cell::PlayerOccupied(Player::Naught) => {
                    d.draw_circle_lines(
                        x as i32 * SIZE + SIZE / 2,
                        y as i32 * SIZE + SIZE / 2,
                        (SIZE as f32) / 2.0 - MARGIN as f32,
                        player_color(Player::Naught),
                    );
                }
            };
        }
    }
}

fn draw_key_box(
    d: &mut RaylibDrawHandle,
    r: Rectangle,
    text: String,
    font_size: i32,
    color: Color,
) {
    d.draw_rectangle_lines_ex(r, 2.0, color);
    d.draw_text(
        &text,
        r.x as i32 + font_size / 2,
        r.y as i32 + font_size / 2,
        font_size,
        color,
    );
}
