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
        WriteStorage<'s, Car>
    );

    fn run(&mut self, (mut transforms, players, input, paused, mut cars): Self::SystemData) {
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
                    transform.translation().x.max(-AREA_WIDTH).min(AREA_WIDTH)
                
            }

            // test function, need to remove
            let shoot = input.action_is_down("shoot").unwrap_or(false);
            if shoot {
                player.shoot();
            }
        }
    }
}

const COLLISION_RADIUS: f32 = 9.0;
#[derive(SystemDesc)]
pub struct PlayerCollisionSystem;

impl<'s> System<'s> for PlayerCollisionSystem {
    type SystemData = (
        ReadStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Car>,
        Read<'s, Paused>,
    );

    fn run(&mut self, (transforms, players, cars, paused): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        for ((_, player_transform), (car, car_transform)) in
            (&players, &transforms).join().flat_map(|p| {
                (&cars, &transforms)
                    .join()
                    .map(|e| (p, e))
                    .collect::<Vec<_>>()
            })
        {
            if (player_transform.translation() - car_transform.translation()).norm()
                <= COLLISION_RADIUS
            {
                log::info!("Collision between player and Car");
            }
        }
    }
}
