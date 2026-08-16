use raylib::prelude::*;
use tictactoe_ratatui::tictactoe::{Cell, GameState, Outcome, PlaceError, Player, WinCombination};

use crate::key_bindings::*;

pub fn player_color(p: Player) -> Color {
    match p {
        Player::Cross => Color::RED,
        Player::Nought => Color::GREEN,
    }
}

pub const TEXT_COLOR: Color = Color::BLACK;
pub const KEY_COLOR: Color = Color::DARKGRAY;
pub const ERROR_COLOR: Color = Color::DARKRED;
pub const BACKGROUND_LIGHT: Color = Color::LIGHTGRAY;
pub const BACKGROUND_DARK: Color = Color::GRAY;
pub const BACKGROUND_SIDE: Color = Color::WHITESMOKE;
pub const LINE_COLOR: Color = Color::BLACK;

#[derive(Debug, Copy, Clone)]
pub enum Error {
    LocalError(PlaceError),
    RemoteError,
}

pub struct Renderer<'a, 'b> {
    pub game: &'a GameState,
    pub key_bindings: &'a KeyBinding,
    pub error: Option<Error>,
    pub draw: &'a mut RaylibDrawHandle<'b>,
}

impl<'a, 'b> Renderer<'a, 'b> {
    pub fn render(&mut self) {
        self.draw.clear_background(Color::PINK); // pink to check everything is covered
        draw_game_board(&mut self.draw, &self.game, &self.key_bindings);
        draw_side_panel(&mut self.draw, &self.game, self.error, &self.key_bindings);
    }
}

pub fn draw_side_panel(
    d: &mut RaylibDrawHandle,
    game: &GameState,
    error: Option<Error>,
    key_bindings: &KeyBinding,
) {
    // Background + Separator Line to board
    d.draw_rectangle(600, 0, 200, 600, BACKGROUND_SIDE);
    d.draw_line(600, 0, 600, 600, LINE_COLOR);
    d.draw_line(601, 0, 601, 600, LINE_COLOR);

    let mut y = 10;
    // If there is an error, show it at the top
    if let Some(err) = error {
        let message = format!("ERROR:\n{:?}", err);
        d.draw_text(&message, 610, y, 20, ERROR_COLOR);

        y += 50;
    }

    if let Some(outcome) = game.outcome {
        // Game is over, just render the winner / outcome
        let message = match outcome {
            Outcome::Draw => "Draw - no winner",
            Outcome::PlayerWins(Player::Cross, _) => "Cross won!",
            Outcome::PlayerWins(Player::Nought, _) => "Naught won!",
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
        key_bindings.close_name.to_uppercase(),
        10,
        KEY_COLOR,
    );
    d.draw_text("Quit app", x + 40, y + 5, 20, TEXT_COLOR);
}

pub fn draw_game_board(d: &mut RaylibDrawHandle, game: &GameState, key_bindings: &KeyBinding) {
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
                Cell::PlayerOccupied(Player::Nought) => {
                    d.draw_circle_lines(
                        x as i32 * SIZE + SIZE / 2,
                        y as i32 * SIZE + SIZE / 2,
                        (SIZE as f32) / 2.0 - MARGIN as f32,
                        player_color(Player::Nought),
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
