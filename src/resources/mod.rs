pub mod prefabs;
pub mod sprites;

use amethyst::prelude::*;

/// A registry of some type of resource which enables lookup
pub trait ResourceRegistry {
    /// The type of resource being produced
    type ResourceType;

    /// Get the requested resource matching the given name
    fn find(&self, world: &World, name: &str) -> Option<Self::ResourceType>;
}