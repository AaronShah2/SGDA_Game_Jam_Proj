use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, PrefabData, Serialize)]
#[prefab(Component)]
pub struct Player {
    pub speed: f32,
}

impl Player {
    pub fn shoot(&self) {
        println!("PEW!");
    }
    //slows player down
    pub fn slow_down(&mut self) {
        self.speed = 5.0f32;
    }

    //returns speed back to normal
    pub fn normal_speed(&mut self) {
        self.speed = 10.0f32;
    }

    //doubles speed
    pub fn speed_up(&mut self) {
        self.speed = 20.0f32;
    }

    pub fn new() -> Self {
        Player { speed: 10.0f32 }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
