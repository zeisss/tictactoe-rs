use tictactoe_ratatui::tictactoe::{Player, Outcome, PlaceError, GameState};

fn main() {
    let mut board = GameState::default();
    board.place(0, 0).unwrap()
    ;
    println!("Hello, world!");
    println!("{:?}", board);
}
