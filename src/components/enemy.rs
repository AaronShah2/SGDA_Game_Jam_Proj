use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Deserialize, PrefabData, Serialize)]
#[prefab(Component)]
pub struct Enemy {
    pub speed: f32,
}
impl Enemy {
    pub fn new() -> Self {
        Enemy { speed: 10.0f32 }
    }
    //slows enemy down
    pub fn slow_down(&mut self) {
        self.speed = 4.5f32;
    }

    //returns enemy back to normal
    pub fn normal_speed(&mut self) {
        self.speed = 9.0f32;
    }

    //doubles speed
    pub fn speed_up(&mut self) {
        self.speed = 18.0f32;
    }
}
impl Default for Enemy {
    fn default() -> Self {
        Self::new()
    }
}

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
