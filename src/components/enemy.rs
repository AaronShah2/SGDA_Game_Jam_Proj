use amethyst::{
    assets::PrefabData,
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Debug, Default, Deserialize, PrefabData, Serialize)]
#[prefab(Component)]
pub struct Enemy;

impl Component for Enemy {
    type Storage = DenseVecStorage<Self>;
}
