use std::fmt;
use std::result::{Result, Result::Err, Result::Ok};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Player {
    Naught,
    Cross,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if *self == Player::Naught {
            write!(f, "Naught")
        } else {
            write!(f, "Cross")
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Outcome {
    PlayerWins(Player, WinCombination),
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
    pub board: [Cell; 9],
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            active_player: Player::Naught,
            outcome: None,
            board: [Cell::Empty; 9],
        }
    }
}

type WinCombination = ((usize, usize), (usize, usize), (usize, usize));

impl GameState {
    pub fn get_cell(&self, pos: (usize, usize)) -> Cell {
        if pos.0 > 2 || pos.1 > 2 {
            panic!("Illegal Coordinate");
        }
        self.board[pos.0 + 3 * pos.1]
    }

    // private use only
    fn get_cell_mut(&mut self, pos: (usize, usize)) -> &mut Cell {
        if pos.0 > 2 || pos.1 > 2 {
            panic!("Illegal Coordinate");
        }
        &mut self.board[pos.0 + 3 * pos.1]
    }

    pub fn place(&mut self, x: usize, y: usize) -> Result<(), PlaceError> {
        if x > 2 || y > 2 {
            return Err(PlaceError::InvalidCoordinates);
        }
        if self.outcome.is_some() {
            return Err(PlaceError::GameOver);
        }

        match self.get_cell((x, y)) {
            Cell::PlayerOccupied(_) => {
                return Err(PlaceError::CellOccupied);
            }
            Cell::Empty => {
                *self.get_cell_mut((x, y)) = Cell::PlayerOccupied(self.active_player);
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
        const VALID_WINS: [WinCombination; 8] = [
            // column
            ((0, 0), (0, 1), (0, 2)),
            ((1, 0), (1, 1), (1, 2)),
            ((2, 0), (2, 1), (2, 2)),
            // rows
            ((0, 0), (1, 0), (2, 0)),
            ((0, 1), (1, 1), (2, 1)),
            ((0, 2), (1, 2), (2, 2)),
            // diagonal
            ((0, 0), (1, 1), (2, 2)),
            ((0, 2), (1, 1), (2, 0)),
        ];

        for condition in VALID_WINS.iter() {
            let first = self.get_cell(condition.0);
            if let Cell::PlayerOccupied(player) = first {
                let second = self.get_cell(condition.1);
                let third = self.get_cell(condition.2);

                if first == second && second == third {
                    return Some(Outcome::PlayerWins(
                        player,
                        *condition,
                    ));
                }
            }
        }

        // Check for draw
        if self
            .board
            .iter()
            .all(|cell| *cell != Cell::Empty)
        {
            return Some(Outcome::Draw);
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_gives_empty_board() {
        let game = GameState::default();
        assert!(game.outcome.is_none(), "Expected outcome to be empty");
        assert!(game.board.iter().all(|c| *c == Cell::Empty), "Expected only empty cells");

    }

    #[test]
    fn test_place_occupies_cell() {
        let mut game = GameState::default();
        game.place(0,0).unwrap();
        assert_eq!(Cell::PlayerOccupied(Player::Naught), game.get_cell((0,0)));
    }

    #[test]
    fn test_game_state() {
        let mut game = GameState::default();
        assert_eq!(game.outcome, None);
        game.place(0, 0).unwrap(); // Naught
        game.place(1, 0).unwrap(); // Cross
        game.place(0, 1).unwrap(); // Naught
        game.place(1, 1).unwrap(); // Cross
        game.place(0, 2).unwrap(); // Naught wins
        assert_eq!(game.outcome, Some(Outcome::PlayerWins(
            Player::Naught, 
            ((0,0), (0, 1), (0, 2)),
        )));
    }

    #[test]
    fn test_game_state_cross_wins() {
        let mut game = GameState::default();
        assert_eq!(game.outcome, None);
        game.place(0, 0).unwrap(); // Naught
        game.place(1, 0).unwrap(); // Cross
        game.place(0, 1).unwrap(); // Naught
        game.place(1, 1).unwrap(); // Cross
        game.place(2, 2).unwrap(); // Naught
        game.place(1, 2).unwrap(); // Cross wins
        assert_eq!(game.outcome, Some(Outcome::PlayerWins(
            Player::Cross, 
            ((1, 0), (1, 1), (1, 2)),
        )));
    }

    #[test]
    fn test_game_state_draw() {
        let mut game = GameState::default();
        assert_eq!(game.outcome, None);

        game.place(0, 0).unwrap(); // Naught
        game.place(1, 1).unwrap(); // Cross
        game.place(2, 2).unwrap(); // Naught
        game.place(0, 2).unwrap(); // Cross
        game.place(2, 0).unwrap(); // Naught
        game.place(2, 1).unwrap(); // Cross
        game.place(1, 2).unwrap(); // Naught
        game.place(1, 0).unwrap(); // Cross
        game.place(0, 1).unwrap(); // Naught -> Draw

        assert_eq!(game.outcome, Some(Outcome::Draw));
    }

    #[test]
    fn test_place_fails_on_occupied() {
        // Other player cannot occupy the same space
        let mut game = GameState::default();
        game.place(1, 1).unwrap(); // Naught
        assert_eq!(game.place(1, 1), Err(PlaceError::CellOccupied));

        // Neither can you overwrite your own space
        let mut game = GameState::default();
        game.place(1, 1).unwrap(); // Naught
        game.place(0, 0).unwrap(); // Cross
        assert_eq!(game.place(1, 1), Err(PlaceError::CellOccupied));
    }

    #[test]
    fn test_place_fails_illegal_coordinates() {
        let mut game = GameState::default();
        assert_eq!(game.place(0, 3), Err(PlaceError::InvalidCoordinates));
        assert_eq!(game.place(3, 3), Err(PlaceError::InvalidCoordinates));
        assert_eq!(game.place(255, 255), Err(PlaceError::InvalidCoordinates));
    }

    #[test]
    fn test_place_fails_on_gameover() {
        let mut game = GameState::default();
        game.place(0,0).unwrap();
        game.place(0,1).unwrap();
        game.place(1,0).unwrap();
        game.place(1,1).unwrap();
        game.place(2,0).unwrap();

        assert_eq!(game.place(2,1), Err(PlaceError::GameOver));
        assert!(game.outcome.is_some(), "Expected game to be over");
    }
}
