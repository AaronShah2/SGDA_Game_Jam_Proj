// use amethyst::{
//     assets::PrefabData,
//     derive::PrefabData,
//     ecs::{Component, DenseVecStorage, Entity, WriteStorage},
//     Error,
// };
// use serde::{Deserialize, Serialize};

mod background;
mod car;
mod enemy;
mod mud;
mod player;

pub use background::Background;
pub use car::Car;
pub use enemy::Enemy;
pub use mud::Mud;
pub use player::Player;

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
