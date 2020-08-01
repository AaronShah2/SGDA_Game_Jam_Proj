use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, PrefabData, Serialize)]
#[prefab(Component)]
pub struct Mud{
    pub is_player_touching: bool,
    pub is_enemy_touching: bool,
    pub height: f32,
    pub width: f32,
}

impl Mud {
    pub fn new() -> Self {
        Mud {
            is_player_touching: false,
            is_enemy_touching: false,
            height: 80.0f32,
            width: 120.0f32,
        }
    }
}

impl Default for Mud {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Mud {
    type Storage = DenseVecStorage<Self>;
}
