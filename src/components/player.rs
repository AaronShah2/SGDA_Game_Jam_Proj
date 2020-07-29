use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, PrefabData, Serialize)]
#[prefab(Component)]
pub struct Player;

impl Player {
    pub fn shoot(&self) {
        println!("PEW!");
    }

    pub fn new() -> Self {
        Player {}
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}
