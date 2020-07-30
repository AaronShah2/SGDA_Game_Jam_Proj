mod controls;
pub mod prefabs;
pub mod sprites;

use amethyst::prelude::*;

pub use controls::Controls;

/// A registry of some type of resource which enables lookup
pub trait ResourceRegistry {
    /// The type of resource being produced
    type ResourceType;

    /// Get the requested resource matching the given name
    fn find(&self, world: &World, name: &str) -> Option<Self::ResourceType>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QuitToMenu(pub bool);
impl Default for QuitToMenu {
    fn default() -> Self {
        QuitToMenu(false)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Paused {
    Paused,
    Unpaused,
}
impl Default for Paused {
    fn default() -> Self {
        Paused::Unpaused
    }
}
