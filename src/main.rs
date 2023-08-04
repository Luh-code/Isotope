pub mod window;

use window::main_window;

pub fn main() -> iced::Result {
  main_window::create_window()
}