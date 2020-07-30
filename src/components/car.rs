use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, PrefabData, Serialize)]
#[prefab(Component)]
pub struct Car{
    pub height: f32,
    pub width: f32,
}

impl Car {
    pub fn new() -> Self {
        Car { 
            height: 40f32,
            width: 40f32,
        }
    }

    // deactivates car's hitbox
    pub fn deactivate_hitbox(&mut self) {
        self.height = 0f32;
        self.width = 0f32;
    }
}

impl Component for Car {
    type Storage = DenseVecStorage<Self>;
}
