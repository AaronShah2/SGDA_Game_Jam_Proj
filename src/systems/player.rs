use crate::{
    components::{Car, Player, Dog, Mud},
    resources::Paused,
};
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
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Paused>,
        ReadStorage<'s, Car>,
    );

    fn run(&mut self, (mut transforms, players, input, paused, cars): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }

        // finds car area & location of car impact
        let mut car_x_min = 0.0f32;
        let mut car_x_max = 0.0f32;
        let mut car_y_min = 0.0f32;
        let mut car_y_max = 0.0f32;
        for (car, transform) in (&cars, &mut transforms).join() {
            if car.is_player_touching {
                car_x_min = transform.translation().x - (car.width * 1.1);
                car_x_max = transform.translation().x + (car.width * 1.1);
                car_y_min = transform.translation().y - (car.height * 1.1);
                car_y_max = transform.translation().y + (car.height * 1.1);
            }
        }
        for (player, transform) in (&players, &mut transforms).join() {
            // unwraps elements from inputs.ron
            let horizontal = input.axis_value("horizontal").unwrap_or(0.0);
            let vertical = input.axis_value("vertical").unwrap_or(0.0);

            // lets player move
            let movement = Vector3::new(horizontal, vertical, 0.0f32);
            if movement.norm_squared() != 0.0 {
                if !player.is_in_car {
                    // player movement
                    transform.prepend_translation(movement.normalize() * (player.speed));
                    
                } else {
                    
                    // moves player back if they are in a car
                    while transform.translation().x > car_x_min
                        && transform.translation().x < car_x_max
                        && transform.translation().y > car_y_min
                        && transform.translation().y < car_y_max
                    {
                        transform.prepend_translation(-(movement.normalize() * (player.speed)));
                    }
                }

                // sets area boundaries
                transform.translation_mut().x =
                    transform.translation().x.max(-AREA_WIDTH).min(AREA_WIDTH);
            }

            // test function, need to remove
            let shoot = input.action_is_down("shoot").unwrap_or(false);
            if shoot {
                player.shoot();
            }
        }
    }
}

#[derive(SystemDesc)]
pub struct PlayerCollisionSystem;

impl<'s> System<'s> for PlayerCollisionSystem {
    type SystemData = (
        WriteStorage<'s, Player>,
        ReadStorage<'s, Car>,
        ReadStorage<'s, Dog>,
        ReadStorage<'s, Mud>,
        Read<'s, Paused>,
    );

    fn run(&mut self, (mut players, cars, dogs, muds, paused): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        for (player,) in (&mut players,).join() {
            // checks if player is hit by car
            let mut hit_by_car: bool = false;
            for (car,) in (&cars,).join() {
                if car.is_player_touching {
                    hit_by_car = true;
                }
            }
            // checks if player is hit by dog
            let mut hit_by_dog: bool = false;
            for (dog,) in (&dogs,).join() {
                if dog.is_player_touching {
                    hit_by_dog = true;
                }
            }

            // checks if player is hit by mud
            let mut hit_by_mud: bool = false;
            for (mud,) in (&muds,).join() {
                if mud.is_player_touching {
                    hit_by_mud = true;
                }
            }

            // adjust player's speed bacsed on their collisions
            if hit_by_dog {
                player.stop();
            } else if hit_by_mud {
                player.slow_down();
            }
            else {
                player.normal_speed();
            }

            // checks if player hit by car
            if hit_by_car {
                player.is_in_car = true;
            }
            else {
                player.is_in_car = false;
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
