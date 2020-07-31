//! This file manages the state of the game

mod cutscene;
mod gameplay;
mod loading;
mod menu;
mod options;
mod pause;
mod test;

pub use cutscene::CutsceneState;
pub use gameplay::GameplayState;
pub use loading::LoadingState;
pub use menu::MenuState;
pub use options::OptionsState;
pub use pause::PauseState;
pub use test::TestState;
