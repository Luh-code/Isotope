pub mod window;

use crate::window::main_window::{
  create_window,
  EditorFlags,
};
use crate::window::keybinds::input_manager::{
  KeyBinding,
  KeyBindingManager,
};
use iced::keyboard::{
  KeyCode,
  Modifiers,
};

pub mod tests;

pub fn main() -> iced::Result {
  let mut mger = KeyBindingManager::new();
  mger.add_key_binding(
    KeyBinding {
      key_code: KeyCode::C,
      modifiers: Modifiers::CTRL,
    },
    String::from("test"),
    false
  );

  let flags = EditorFlags{
    key_binding_manager: mger,
  };
  create_window(flags)
}