use crate::{components::{Player, Car}, resources::Paused};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};
use nalgebra::base::Vector3;

const AREA_WIDTH: f32 = 760.0f32;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Paused>,
        ReadStorage<'s, Car>
    );

    fn run(&mut self, (mut transforms, players, input, paused, cars): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        for (player, transform) in (&players, &mut transforms).join() {
            // unwraps elements from inputs.ron
            let horizontal = input.axis_value("horizontal").unwrap_or(0.0);
            let vertical = input.axis_value("vertical").unwrap_or(0.0);

            // lets player move
            let movement = Vector3::new(horizontal, vertical, 0.0f32);
            if movement.norm_squared() != 0.0 {
                transform.prepend_translation(movement.normalize() * (player.speed));
                transform.translation_mut().x =
                    transform.translation().x.max(-AREA_WIDTH).min(AREA_WIDTH);
                transform.translation_mut().y =
                    transform.translation().y;
                // handles car collision
                if player.isInCar {
                    for (car,) in (&cars,).join() {
                    transform.translation_mut().x =
                    transform.translation()
                        .x.max(-(car.width)).min(car.width);
                    transform.translation_mut().y =
                    transform.translation()
                        .y.max(-(car.height)).min(car.height);
                    }
                }
                
            }

            // test function, need to remove
            let shoot = input.action_is_down("shoot").unwrap_or(false);
            if shoot {
                player.shoot();
            }
        }
    }
}

const COLLISION_RADIUS: f32 = 120.0;
#[derive(SystemDesc)]
pub struct PlayerCollisionSystem;

impl<'s> System<'s> for PlayerCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        ReadStorage<'s, Car>,
        Read<'s, Paused>,
    );

    fn run(&mut self, (transforms, mut players, cars, paused): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        for (player, player_transform) in (&mut players, &transforms).join() {
            for (car, car_transform) in (&cars, &transforms).join() {
                // log::info!("player_coor: {}, Car_coor: {}",
                // player_transform.translation(), car_transform.translation());

                // keeps track of distance between car and player
                let x = player_transform.translation().x - car_transform.translation().x;
                let y = player_transform.translation().y - car_transform.translation().y;
                //log::info!("x: {}, y: {}", x, y);
                // checks if within boundaries
                if x>= -(car.width) && x<= car.width
                    && y >= -(car.height) && y<= car.height
                {
                    // log::info!("You are in the car-zone.");
                    player.isInCar = true;
                }
                else {
                    player.isInCar = false;
                }
                
            }
        }

        // for ((_, player_transform), (car, car_transform)) in
        //     (&players, &transforms).join().flat_map(|p| {
        //         (&cars, &transforms)
        //             .join()
        //             .map(|e| (p, e))
        //             .collect::<Vec<_>>()
        //     })
        // {
        //     log::info!("player_coor: {}, Car_coor: {}",
        //     player_transform.translation(), car_transform.translation());
        //     if (player_transform.translation() - car_transform.translation()).norm()
        //         <= COLLISION_RADIUS
        //     {
                
        //     }
        // }
    }
}
