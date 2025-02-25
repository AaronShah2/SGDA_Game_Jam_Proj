use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, PrefabData, Serialize)]
#[prefab(Component)]
pub struct Car {
    pub is_player_touching: bool,
    pub is_enemy_touching: bool,
    pub height: f32,
    pub width: f32,
}

impl Car {
    pub fn new() -> Self {
        Car {
            is_player_touching: false,
            is_enemy_touching: false,
            height: 140f32,
            width: 210f32,
        }
    }

    // deactivates car's hitbox
    pub fn deactivate_hitbox(&mut self) {
        self.height = 0f32;
        self.width = 0f32;
    }
}

impl Default for Car {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Car {
    type Storage = DenseVecStorage<Self>;
}
