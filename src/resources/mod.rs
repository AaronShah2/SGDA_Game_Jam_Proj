mod controls;
pub mod prefabs;
pub mod sprites;

use amethyst::{ecs::Entity, prelude::*};

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

const METERS_PER_DISTANCE_UNIT: f32 = 0.01;
#[derive(Debug, Default)]
pub struct HighScore {
    distance: f32,
}
impl HighScore {
    pub fn max(&mut self, distance: f32) {
        self.distance = self.distance.max(distance);
    }

    pub fn get_score(&self) -> f32 {
        self.distance * METERS_PER_DISTANCE_UNIT
    }

    pub fn reset(&mut self) {
        self.distance = 0.0;
    }
}

#[derive(Debug, Default)]
pub struct GameplayScoreDisplay {
    pub displays: Vec<Entity>,
}
