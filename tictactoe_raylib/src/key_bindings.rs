use raylib::prelude::*;

// UI Actions
pub enum GameAction {
    PlaceToken(usize, usize),
    ResetBoard,
    CloseBoard,
}

pub struct KeyBinding {
    pub positions: [KeyboardKey; 9],
    pub labels: [String; 9],

    pub reset_key: KeyboardKey,
    pub reset_name: String,

    pub close_key: KeyboardKey,
    pub close_name: String,
}

impl KeyBinding {
    pub fn from_raylib_handle(rl: &mut RaylibHandle) -> KeyBinding {
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

            close_key: KeyboardKey::KEY_ESCAPE,
            close_name: rl
                .get_key_name(KeyboardKey::KEY_ESCAPE)
                .unwrap_or("Esc".into()),
        }
    }

    pub fn get_name(&self, key: KeyboardKey) -> String {
        if let Some(pos) = self.positions.iter().position(|k| key == *k) {
            self.labels[pos].to_uppercase().clone()
        } else {
            "?".into()
        }
    }

    pub fn get_name_for_position(&self, x: usize, y: usize) -> String {
        assert!(x <= 2);
        assert!(y <= 2);
        return self.get_name(self.positions[x + y * 3]);
    }

    pub fn key_to_position(&self, key: KeyboardKey) -> Option<(usize, usize)> {
        if let Some(pos) = self.positions.iter().position(|k| key == *k) {
            let r = pos % 3;
            Some((r, (pos - r) / 3))
        } else {
            None
        }
    }

    /// get_action checks rl if any bound keys are pressed and returns an Option for the represented action.
    pub fn get_action(&self, rl: &mut RaylibHandle) -> Option<GameAction> {
        if rl.is_key_pressed(self.reset_key) {
            Some(GameAction::ResetBoard)
        } else if rl.is_key_pressed(self.close_key) {
            return Some(GameAction::CloseBoard);
        } else if let Some(key) = rl.get_key_pressed() {
            let pos = self.key_to_position(key);
            if let Some(p) = pos {
                Some(GameAction::PlaceToken(p.0, p.1))
            } else {
                None
            }
        } else {
            None
        }
    }
}
