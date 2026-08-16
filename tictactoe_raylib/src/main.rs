use raylib::{ffi::Rectangle, prelude::*};

use tictactoe_ratatui::tictactoe::{Cell, GameState, Outcome, PlaceError, Player, WinCombination};

mod board_renderer;
mod key_bindings;

pub use board_renderer::*;
pub use key_bindings::*;

#[derive(Debug)]
enum Screen {
    Menu,
    LocalPlay,
    ServerMode,
    ClientMode,
    Quit,
}

fn main() {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("TicTacToe in Rust + Raylib")
        .build();
    rl.set_target_fps(30);
    rl.set_exit_key(None);
    let mut screen = Screen::Menu;

    let key_bindings = KeyBinding::from_raylib_handle(&mut rl);

    loop {
        println!("Entering screen {:?}", screen);
        screen = match screen {
            Screen::Menu => run_menu_screen(&mut rl, &thread),
            Screen::LocalPlay => run_local_play_screen(&mut rl, &thread, &key_bindings),
            Screen::ServerMode => todo!("Not implemented yet"),
            Screen::ClientMode => todo!("Not implemented yet"),
            Screen::Quit => break,
        };
        println!("Next screen: {:?}", screen);

        // Draw one frame in black to clear the inputs, otherwise the Escape key pressed on the game screen
        // would immediately be read + handled on the menu as well.
        rl.draw(&thread, |mut d| {
            d.clear_background(Color::WHITE);
        });
    }
}

// Main Menu
fn run_menu_screen(rl: &mut RaylibHandle, thread: &RaylibThread) -> Screen {
    let local_play = Rectangle::new(300.0, 350.0, 200.0, 40.0);
    let server_mode = Rectangle::new(300.0, 400.0, 200.0, 40.0);
    let client_mode = Rectangle::new(300.0, 450.0, 200.0, 40.0);
    let quit_game = Rectangle::new(300.0, 500.0, 200.0, 40.0);

    while !rl.window_should_close() {
        // Keyboard Shortcuts
        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            return Screen::Quit;
        } else if rl.is_key_pressed(KeyboardKey::KEY_ONE) {
            return Screen::LocalPlay;
        } else if rl.is_key_pressed(KeyboardKey::KEY_TWO) {
            return Screen::ServerMode;
        } else if rl.is_key_pressed(KeyboardKey::KEY_THREE) {
            return Screen::ClientMode;
        }

        // Rendering + GuiButton handling
        let mut d = rl.begin_drawing(thread);
        d.clear_background(Color::WHITE);
        d.draw_text("Tic Tac Toe", 300, 200, 40, Color::BLACK);
        d.draw_text("in Rust + Raylib", 300, 300, 30, Color::BLACK);

        if d.gui_button(local_play, "[1] Local Play") {
            return Screen::LocalPlay;
        }
        if d.gui_button(server_mode, "[2] Server Mode") {
            return Screen::ServerMode;
        }
        if d.gui_button(client_mode, "[3] Client Mode") {
            return Screen::ClientMode;
        }
        if d.gui_button(quit_game, "Quit App") {
            return Screen::LocalPlay;
        }
    }

    Screen::Quit
}

// Local Play
fn run_local_play_screen(
    rl: &mut RaylibHandle,
    thread: &RaylibThread,
    key_bindings: &KeyBinding,
) -> Screen {
    let mut game = GameState::default();
    let mut last_placement = Ok(());

    while !rl.window_should_close() {
        match key_bindings.get_action(rl) {
            Some(GameAction::PlaceToken(x, y)) => last_placement = game.place(x, y),
            Some(GameAction::ResetBoard) => {
                game = GameState::default();
                last_placement = Ok(());
            }
            Some(GameAction::CloseBoard) => return Screen::Menu,
            None => {}
        }

        // Render
        rl.draw(&thread, |mut d| {
            d.clear_background(Color::PINK); // pink to check everything is covered
            draw_game_board(&mut d, &game, &key_bindings);
            draw_side_panel(&mut d, &game, last_placement, &key_bindings);
        });
    }
    Screen::Quit
}
