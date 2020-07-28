//! This file manages the state of the game

mod loading;
mod menu;
mod options;
mod test;
mod gameplay;

pub use loading::LoadingState;
pub use menu::MenuState;
pub use options::OptionsState;
pub use test::Test;
pub use gameplay::Gameplay;
