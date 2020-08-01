use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

const RUBBER_BAND_CUTOFF: f32 = 200.0;
const RUBBER_BAND_COEFFICIENT: f32 = 0.01;

#[derive(Copy, Clone, Debug, Deserialize, PrefabData, Serialize)]
#[prefab(Component)]
pub struct Enemy {
    pub speed: f32,
}
impl Enemy {
    pub fn new() -> Self {
        Enemy { speed: 9.0f32 }
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

    //Stops enemy
    pub fn stop(&mut self)
    {
        self.speed = 0.0f32;
    }

    pub fn speed(&self, distance: f32) -> f32 {
        self.speed + (distance - RUBBER_BAND_CUTOFF).max(0.0) * RUBBER_BAND_COEFFICIENT
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
