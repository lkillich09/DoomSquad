use minifb::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GameAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Sprint,
    Crouch,
    Jump,
    RocketJump,
    Interact,
    ToggleAimMode,
    FireBullet,
    FireArrow,
    TurnLeft,
    TurnRight,
    AimUp,
    AimDown,
    ToggleGodMode,
    LockEnemies,
    MenuBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyBindings {
    // Hier speichern wir spaeter die Zuordnung von Aktionen zu Tasten.
    pub bindings: HashMap<GameAction, String>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        let bindings = HashMap::from([
            (GameAction::MoveForward, "W".to_string()),
            (GameAction::MoveBackward, "S".to_string()),
            (GameAction::MoveLeft, "A".to_string()),
            (GameAction::MoveRight, "D".to_string()),
            (GameAction::Sprint, "LeftShift".to_string()),
            (GameAction::Crouch, "C".to_string()),
            (GameAction::Jump, "Space".to_string()),
            (GameAction::RocketJump, "R".to_string()),
            (GameAction::Interact, "F".to_string()),
            (GameAction::ToggleAimMode, "E".to_string()),
            (GameAction::FireBullet, "RightCtrl".to_string()),
            (GameAction::FireArrow, "RightShift".to_string()),
            (GameAction::TurnLeft, "Left".to_string()),
            (GameAction::TurnRight, "Right".to_string()),
            (GameAction::AimUp, "Up".to_string()),
            (GameAction::AimDown, "Down".to_string()),
            (GameAction::ToggleGodMode, "G".to_string()),
            (GameAction::LockEnemies, "L".to_string()),
            (GameAction::MenuBack, "Escape".to_string()),
        ]);

        Self { bindings }
    }
}

impl KeyBindings {
    pub fn get_key(&self, action: GameAction) -> Option<&String> {
        self.bindings.get(&action)
    }

    pub fn get_minifb_key(&self, action: GameAction) -> Option<Key> {
        let key_name = self.get_key(action)?;
        
        match key_name.as_str() {
            //numbers
            "Key0" => Some(Key::Key0),
            "Key1" => Some(Key::Key1),
            "Key2" => Some(Key::Key2),
            "Key3" => Some(Key::Key3),
            "Key4" => Some(Key::Key4),
            "Key5" => Some(Key::Key5),
            "Key6" => Some(Key::Key6),
            "Key7" => Some(Key::Key7),
            "Key8" => Some(Key::Key8),
            "Key9" => Some(Key::Key9),
            
            //letters
            "A" => Some(Key::A),
            "B" => Some(Key::B),
            "C" => Some(Key::C),
            "D" => Some(Key::D),
            "E" => Some(Key::E),
            "F" => Some(Key::F),
            "G" => Some(Key::G),
            "H" => Some(Key::H),
            "I" => Some(Key::I),
            "J" => Some(Key::J),
            "K" => Some(Key::K),
            "L" => Some(Key::L),
            "M" => Some(Key::M),
            "N" => Some(Key::N),
            "O" => Some(Key::O),
            "P" => Some(Key::P),
            "Q" => Some(Key::Q),
            "R" => Some(Key::R),
            "S" => Some(Key::S),
            "T" => Some(Key::T),
            "U" => Some(Key::U),
            "V" => Some(Key::V),
            "W" => Some(Key::W),
            "X" => Some(Key::X),
            "Y" => Some(Key::Y),
            "Z" => Some(Key::Z),

            //Function Keys
            "F1" => Some(Key::F1),
            "F2" => Some(Key::F2),
            "F3" => Some(Key::F3),
            "F4" => Some(Key::F4),
            "F5" => Some(Key::F5),
            "F6" => Some(Key::F6),
            "F7" => Some(Key::F7),
            "F8" => Some(Key::F8),
            "F9" => Some(Key::F9),
            "F10" => Some(Key::F10),
            "F11" => Some(Key::F11),
            "F12" => Some(Key::F12),
            "F13" => Some(Key::F13),
            "F14" => Some(Key::F14),
            "F15" => Some(Key::F15),

            //Arrow keys
            "Down" => Some(Key::Down),
            "Left" => Some(Key::Left),
            "Right" => Some(Key::Right),
            "Up" => Some(Key::Up),

            //Special characters
            "Apostrophe" => Some(Key::Apostrophe),
            "Backquote" => Some(Key::Backquote),
            "Backslash" => Some(Key::Backslash),
            "Comma" => Some(Key::Comma),
            "Equal" => Some(Key::Equal),
            "LeftBracket" => Some(Key::LeftBracket),
            "Minus" => Some(Key::Minus),
            "Period" => Some(Key::Period),
            "RightBracket" => Some(Key::RightBracket),
            "Semicolon" => Some(Key::Semicolon),
            "Slash" => Some(Key::Slash),

            //Additional Keys
            "Backspace" => Some(Key::Backspace),
            "Delete" => Some(Key::Delete),
            "End" => Some(Key::End),
            "Enter" => Some(Key::Enter),
            "Escape" => Some(Key::Escape),
            "Home" => Some(Key::Home),
            "Insert" => Some(Key::Insert),
            "Menu" => Some(Key::Menu),
            "PageDown" => Some(Key::PageDown),
            "PageUp" => Some(Key::PageUp),
            "Pause" => Some(Key::Pause),
            "Space" => Some(Key::Space),
            "Tab" => Some(Key::Tab),
            
            //Lock Keys
            "NumLock" => Some(Key::NumLock),
            "CapsLock" => Some(Key::CapsLock),
            "ScrollLock" => Some(Key::ScrollLock),

            //Modifier
            "LeftShift" => Some(Key::LeftShift),
            "RightShift" => Some(Key::RightShift),
            "LeftCtrl" => Some(Key::LeftCtrl),
            "RightCtrl" => Some(Key::RightCtrl),
            "LeftAlt" => Some(Key::LeftAlt),
            "RightAlt" => Some(Key::RightAlt),
            "LeftSuper" => Some(Key::LeftSuper),
            "RightSuper" => Some(Key::RightSuper),

            //Numeric Keypad
            "NumPad0" => Some(Key::NumPad0),
            "NumPad1" => Some(Key::NumPad1),
            "NumPad2" => Some(Key::NumPad2),
            "NumPad3" => Some(Key::NumPad3),
            "NumPad4" => Some(Key::NumPad4),
            "NumPad5" => Some(Key::NumPad5),
            "NumPad6" => Some(Key::NumPad6),
            "NumPad7" => Some(Key::NumPad7),
            "NumPad8" => Some(Key::NumPad8),
            "NumPad9" => Some(Key::NumPad9),
            "NumPadDot" => Some(Key::NumPadDot),
            "NumPadSlash" => Some(Key::NumPadSlash),
            "NumPadAsterisk" => Some(Key::NumPadAsterisk),
            "NumPadMinus" => Some(Key::NumPadMinus),
            "NumPadPlus" => Some(Key::NumPadPlus),
            "NumPadEnter" => Some(Key::NumPadEnter),

            //No usable keys
            _ => None,
        }
    }

    pub fn set_key(&mut self, action: GameAction, key: Key) {
        let key_name = Self::key_to_string(key);
        self.bindings.insert(action, key_name);

        if let Err(error) = self.save() {
            eprintln!("Fehler beim Speichern der Tastenbelegung {}", error);
        }
    }
    

    pub fn key_to_string(key: Key) -> String {
        format!("{:?}", key)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config = ron::to_string(self)?;
        std::fs::write("key_bindings.ron", config)?;
        Ok(())
    }

    pub fn load() -> Self {
        let config = std::fs::read_to_string("key_bindings.ron");

        match config {
            Ok(content) => ron::from_str(&content).unwrap_or_default(),
            Err(_) => Self::default(),
        }
    }
}


