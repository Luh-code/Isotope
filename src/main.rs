pub mod window;

use crate::window::main_window;

pub mod tests;

pub fn main() -> iced::Result {
  main_window::create_window()
}