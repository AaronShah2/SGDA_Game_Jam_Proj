// use amethyst::{
//     assets::PrefabData,
//     derive::PrefabData,
//     ecs::{Component, DenseVecStorage, Entity, WriteStorage},
//     Error,
// };
// use serde::{Deserialize, Serialize};

mod player;
mod mud;
mod enemy;
mod background;

pub use mud::Mud;
pub use player::Player;
pub use enemy::Enemy;
pub use background::Background;

// #[derive(Copy, Clone, Debug, Default, Deserialize, PrefabData, Serialize)]
// #[prefab(Component)]
// pub struct Background;

// impl Component for Background {
//     type Storage = DenseVecStorage<Self>;
// }

// #[derive(Copy, Clone, Debug, Default, Deserialize, PrefabData, Serialize)]
// #[prefab(Component)]
// pub struct Enemy;

// impl Component for Enemy {
//     type Storage = DenseVecStorage<Self>;
// }
