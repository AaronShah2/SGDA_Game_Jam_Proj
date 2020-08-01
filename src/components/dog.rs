use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, PrefabData, Serialize)]
#[prefab(Component)]
pub struct Dog {
    pub is_player_touching: bool,
    pub height: f32,
    pub width: f32,
    pub speed: f32,
}

impl Dog {
    pub fn new() -> Self {
        Dog {
            is_player_touching: false,
            height: 80.0f32,
            width: 80.0f32,
            speed: 5.0f32,
        }
    }
}

impl Default for Dog {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Dog {
    type Storage = DenseVecStorage<Self>;
}