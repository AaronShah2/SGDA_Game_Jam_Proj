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
/// A struct which tracks the current score and the high score
pub struct HighScore {
    distance: f32,
    max_distance: f32,
}
impl HighScore {
    /// Given the distance traveled, update the current game's score
    /// and the high score as necessary
    pub fn max(&mut self, distance: f32) {
        self.distance = self.distance.max(distance);
        self.max_distance = self.max_distance.max(distance);
    }

    /// Get the current game's score (the farthest traveled this game),
    /// adjusted to meters.
    pub fn get_score(&self) -> f32 {
        self.distance * METERS_PER_DISTANCE_UNIT
    }

    /// Get the high score, adjusted to meters.
    pub fn get_high_score(&self) -> f32 {
        self.max_distance * METERS_PER_DISTANCE_UNIT
    }

    /// Resets the score for the current game, while leaving the high
    /// score unchanged
    pub fn reset(&mut self) {
        self.distance = 0.0;
    }
}

#[derive(Debug, Default)]
pub struct GameplayScoreDisplay {
    pub displays: Vec<Entity>,
}

#[derive(Debug, Default)]
pub struct CollisionEvent;
