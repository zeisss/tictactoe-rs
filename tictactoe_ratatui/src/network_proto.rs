use crate::tictactoe::Player;

#[derive(Debug, PartialEq)]
enum Message {
    NewGame(Player),                              // NGX / NGO
    Place { player: Player, x: usize, y: usize }, //[X|O][0-2][0-2]
    Goodbye,                                      // BYE
}

impl From<Message> for [u8; 3] {
    fn from(val: Message) -> Self {
        let s = match val {
            Message::NewGame(player) => match player {
                Player::Nought => "NGO",
                Player::Cross => "NGX",
            },
            Message::Place { player, x, y } => {
                let p = match player {
                    Player::Nought => "O",
                    Player::Cross => "X",
                };
                &*format!("{}{}{}", p, x, y)
            }
            Message::Goodbye => "BYE",
        };
        s.as_bytes().try_into().unwrap()
    }
}

#[derive(Debug)]
enum ParseError {
    Unknown,
    BadPlayerByte,
}

impl TryFrom<[u8; 3]> for Message {
    type Error = ParseError;
    fn try_from(bytes: [u8; 3]) -> Result<Self, Self::Error> {
        let valid_range = b'0'..=b'2';
        match bytes {
            [b'B', b'Y', b'E'] => Ok(Message::Goodbye),
            [b'N', b'G', b'O'] => Ok(Message::NewGame(Player::Nought)),
            [b'N', b'G', b'X'] => Ok(Message::NewGame(Player::Cross)),
            [player_byte, x, y] if valid_range.contains(&x) && valid_range.contains(&y) => {
                Ok(Message::Place {
                    player: match player_byte {
                        b'O' => Player::Nought,
                        b'X' => Player::Cross,
                        _ => return Err(ParseError::BadPlayerByte),
                    },
                    x: (x - b'0') as usize,
                    y: (y - b'0') as usize,
                })
            }
            _ => Err(ParseError::Unknown),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn good_bye_msg_round_trip() {
        let bytes: [u8; 3] = Message::Goodbye.into();
        assert_eq!(bytes, [b'B', b'Y', b'E']);
        let msg: Message = bytes.try_into().unwrap();
        assert_eq!(msg, Message::Goodbye);
    }

    #[test]
    fn new_game_nought_msg_round_trip() {
        let bytes: [u8; 3] = Message::NewGame(Player::Nought).into();
        assert_eq!(bytes, [b'N', b'G', b'O']);
        let msg: Message = bytes.try_into().unwrap();
        assert_eq!(msg, Message::NewGame(Player::Nought));
    }

    #[test]
    fn new_game_cross_msg_round_trip() {
        let bytes: [u8; 3] = Message::NewGame(Player::Cross).into();
        assert_eq!(bytes, [b'N', b'G', b'X']);
        let msg: Message = bytes.try_into().unwrap();
        assert_eq!(msg, Message::NewGame(Player::Cross));
    }

    #[test]
    fn place_msg_round_trip() {
        let original = Message::Place {
            player: Player::Cross,
            x: 1,
            y: 2,
        };
        let bytes: [u8; 3] = original.into();
        assert_eq!(bytes, [b'X', b'1', b'2']);
        let msg: Message = bytes.try_into().unwrap();
        assert_eq!(
            msg,
            Message::Place {
                player: Player::Cross,
                x: 1,
                y: 2
            }
        );
    }

    #[test]
    fn place_msg_round_trip_nought() {
        let original = Message::Place {
            player: Player::Nought,
            x: 0,
            y: 2,
        };
        let bytes: [u8; 3] = original.into();
        assert_eq!(bytes, [b'O', b'0', b'2']);
        let msg: Message = bytes.try_into().unwrap();
        assert_eq!(
            msg,
            Message::Place {
                player: Player::Nought,
                x: 0,
                y: 2
            }
        );
    }

    #[test]
    fn bad_player_byte_is_rejected() {
        let bytes = [b'A', b'1', b'2'];
        let result: Result<Message, ParseError> = bytes.try_into();
        assert!(matches!(result, Err(ParseError::BadPlayerByte)));
    }

    #[test]
    fn out_of_bounds_x_is_rejected() {
        let bytes = [b'X', b'3', b'0'];
        let result: Result<Message, ParseError> = bytes.try_into();
        assert!(matches!(result, Err(ParseError::Unknown)));
    }

    #[test]
    fn out_of_bounds_y_is_rejected() {
        let bytes = [b'X', b'0', b'3'];
        let result: Result<Message, ParseError> = bytes.try_into();
        assert!(matches!(result, Err(ParseError::Unknown)));
    }

    #[test]
    fn unknown_msg_is_rejected() {
        let bytes = [b'F', b'O', b'O'];
        let result: Result<Message, ParseError> = bytes.try_into();
        assert!(matches!(result, Err(ParseError::Unknown)));
    }
}
