use raylib::prelude::*;

use tictactoe_ratatui::tictactoe::{Cell, GameState, Outcome, PlaceError, Player};

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("TicTacToe in Rust + Raylib")
        .build();

    let mut game = GameState::default();
    let mut last_placement = Ok(());

    while !rl.window_should_close() {
        // Event Handling
        if rl.is_key_pressed(KeyboardKey::KEY_Q) {
            last_placement = game.place(0, 0)
        } else if rl.is_key_pressed(KeyboardKey::KEY_W) {
            last_placement = game.place(1, 0)
        } else if rl.is_key_pressed(KeyboardKey::KEY_E) {
            last_placement = game.place(2, 0)
        } else if rl.is_key_pressed(KeyboardKey::KEY_A) {
            last_placement = game.place(0, 1)
        } else if rl.is_key_pressed(KeyboardKey::KEY_S) {
            last_placement = game.place(1, 1)
        } else if rl.is_key_pressed(KeyboardKey::KEY_D) {
            last_placement = game.place(2, 1)
        } else if rl.is_key_pressed(KeyboardKey::KEY_Z) {
            last_placement = game.place(0, 2)
        } else if rl.is_key_pressed(KeyboardKey::KEY_X) {
            last_placement = game.place(1, 2)
        } else if rl.is_key_pressed(KeyboardKey::KEY_C) {
            last_placement = game.place(2, 2)
        } else if rl.is_key_pressed(KeyboardKey::KEY_R) {
            game = GameState::default();
            last_placement = Ok(())
        }

        // Render
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::PINK); // pink to check everything is covered
        draw_game_board(&mut d, &game);
        draw_side_panel(&mut d, &game, last_placement);
    }
}

fn position_to_key(x: usize, y: usize) -> &'static str {
    match (x, y) {
        (0, 0) => "Q",
        (1, 0) => "W",
        (2, 0) => "E",

        (0, 1) => "A",
        (1, 1) => "S",
        (2, 1) => "D",

        (0, 2) => "Z",
        (1, 2) => "X",
        (2, 2) => "C",

        _ => "_",
    }
}

fn player_color(p: Player) -> Color {
    match p {
        Player::Cross => Color::RED,
        Player::Naught => Color::GREEN,
    }
}
const TEXT_COLOR: Color = Color::BLACK;
const ERROR_COLOR: Color = Color::DARKRED;
const BACKGROUND_LIGHT: Color = Color::LIGHTGRAY;
const BACKGROUND_DARK: Color = Color::GRAY;
const BACKGROUND_SIDE: Color = Color::WHITESMOKE;
const LINE_COLOR: Color = Color::BLACK;

fn draw_side_panel(
    d: &mut RaylibDrawHandle,
    game: &GameState,
    last_placement: Result<(), PlaceError>,
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
            Outcome::CrossWins => "Cross won!",
            Outcome::NaughtWins => "Naught won!",
        };

        d.draw_text(message, 610, y, 20, TEXT_COLOR);
    } else {
        // No winner yet, show active player
        let message = format!("{}", game.active_player);
        d.draw_text("Active Player", 610, y, 20, TEXT_COLOR);
        d.draw_text(&message, 610, y + 30, 20, player_color(game.active_player));
    }

    // Draw footer with keybindings and help text
    let mut y = 500;
    d.draw_text(
        "Press key for field\nto place your token",
        610,
        y,
        16,
        TEXT_COLOR,
    );
    y += 45;
    d.draw_text("R: Restart game", 610, y, 20, TEXT_COLOR);
    y += 25;
    d.draw_text("Esc: Quit app", 610, y, 20, TEXT_COLOR);
}

fn draw_game_board(d: &mut RaylibDrawHandle, game: &GameState) {
    const MARGIN: i32 = 20;
    const SIZE: i32 = 200;

    // Draw checker board
    d.draw_rectangle(0, 0, SIZE, SIZE, BACKGROUND_LIGHT);
    d.draw_rectangle(SIZE, 0, SIZE, SIZE, BACKGROUND_DARK);
    d.draw_rectangle(SIZE * 2, 0, SIZE, SIZE, BACKGROUND_LIGHT);

    d.draw_rectangle(0, SIZE, SIZE, SIZE, BACKGROUND_DARK);
    d.draw_rectangle(SIZE, SIZE, SIZE, SIZE, BACKGROUND_LIGHT);
    d.draw_rectangle(SIZE * 2, SIZE, SIZE, SIZE, BACKGROUND_DARK);

    d.draw_rectangle(0, SIZE * 2, SIZE, SIZE, BACKGROUND_LIGHT);
    d.draw_rectangle(SIZE, SIZE * 2, SIZE, SIZE, BACKGROUND_DARK);
    d.draw_rectangle(SIZE * 2, SIZE * 2, SIZE, SIZE, BACKGROUND_LIGHT);

    // Draw lines to separate tiles / cells
    d.draw_line(0, SIZE, SIZE * 3, SIZE, LINE_COLOR);
    d.draw_line(0, SIZE * 2, SIZE * 3, SIZE * 2, LINE_COLOR);

    d.draw_line(SIZE, 0, SIZE, SIZE * 3, LINE_COLOR);
    d.draw_line(SIZE * 2, 0, SIZE * 2, SIZE * 3, LINE_COLOR);

    // Draw a 3x3 grid where each cell is 200x200 (Matching the window height of 600)
    for x in 0..=2 {
        for y in 0..=2 {
            let cell = game.get_cell((x, y));

            match cell {
                Cell::Empty => {
                    d.draw_text(
                        position_to_key(x, y),
                        x as i32 * SIZE + 100,
                        y as i32 * SIZE + 100,
                        20,
                        Color::BLACK,
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
                _ => {}
            };
        }
    }
}
