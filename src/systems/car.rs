// car obj that stops player and enemy
use crate::{
    components::{Car, Enemy, Player},
    resources::Paused,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

#[derive(SystemDesc)]
pub struct CarSystem;

impl<'s> System<'s> for CarSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Car>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
        Read<'s, Paused>,
    );
    fn run(&mut self, (transforms, mut cars, players, enemies, paused): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        for (car, car_transform) in (&mut cars, &transforms).join() {
            for (_, player_transform) in (&players, &transforms).join() {
                // keeps track of distance between car and player
                let x = player_transform.translation().x - car_transform.translation().x;
                let y = player_transform.translation().y - car_transform.translation().y;

                // checks if within boundaries
                if x >= -(car.width) && x <= car.width && y >= -(car.height) && y <= car.height {
                    car.is_player_touching = true;
                } else {
                    car.is_player_touching = false;
                }
            }
            for (_, enemy_transform) in (&enemies, &transforms).join() {
                // keeps track of distance between car and player
                let x = enemy_transform.translation().x - car_transform.translation().x;
                let y = enemy_transform.translation().y - car_transform.translation().y;

                // checks if within boundaries
                if x >= -(car.width) && x <= car.width && y >= -(car.height) && y <= car.height {
                    car.is_enemy_touching = true;
                } else {
                    car.is_enemy_touching = false;
                }
            }
        }
    }
}