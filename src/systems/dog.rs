// dog obj that stops player and enemy
use crate::{
    components::{Dog, Player},
    resources::Paused,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};
use nalgebra::base::Vector3;

const ROTATION_ANGLE: f32 = 3.14f32;
// lets dog move left and right
const AREA_WIDTH: f32 = 760.0f32;
#[derive(SystemDesc)]
pub struct DogSystem;

impl<'s> System<'s> for DogSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Dog>,
        Read<'s, Paused>,
    );

    fn run(&mut self, (mut transforms, dogs, paused): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        for (dog, transform) in (&dogs, &mut transforms).join() {
            let movement = Vector3::new(-1.0, 0.0, 0.0f32);

            // Turns dog around if it hits wall
            if transform.translation().x >= AREA_WIDTH
            {
                transform.set_rotation_y_axis(0.0f32);
            }
            if transform.translation().x <= -(AREA_WIDTH)
            {
                transform.set_rotation_y_axis(ROTATION_ANGLE);
            }
            
            // moves dog diff direction depending on angle
            if transform.rotation().angle() == 0.0
            {
                transform.prepend_translation(movement.normalize() * (dog.speed));
            }
            else
            {
                transform.prepend_translation(-(movement.normalize()) * (dog.speed));
            }
            // sets area boundaries
            transform.translation_mut().x =
            transform.translation().x.max(-AREA_WIDTH).min(AREA_WIDTH);
        }
    }
}

// checks collision system for dog class
#[derive(SystemDesc)]
pub struct DogCollisionSystem;

impl<'s> System<'s> for DogCollisionSystem {
#[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Dog>,
        ReadStorage<'s, Player>,
        Read<'s, Paused>,
    );
    fn run(&mut self, (transforms, mut dogs, player, paused): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        for (dog, dog_transform) in (&mut dogs, &transforms).join() {
            for (player, player_transform) in (&player, &transforms).join() {
                // log::info!("player_coor: {}, dog_coor: {}",
                // player_transform.translation(), dog_transform.translation());

                // keeps track of distance between dog and player
                let x = player_transform.translation().x - dog_transform.translation().x;
                let y = player_transform.translation().y - dog_transform.translation().y;
                //log::info!("x: {}, y: {}", x, y);
                // checks if within boundaries
                if x >= -(dog.width) && x <= dog.width && y >= -(dog.height) && y <= dog.height {
                    log::info!("You are in the dog-zone.");
                    dog.is_player_touching = true;
                } else {
                    dog.is_player_touching = false;
                }
            }
        }
    }
}

// attack system for dog
#[derive(SystemDesc)]
pub struct DogAttackSystem;

impl<'s> System<'s> for DogAttackSystem {
#[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Dog>,
        WriteStorage<'s, Player>,
        Read<'s, Paused>,
    );
    fn run(&mut self, (dogs, mut players, paused): Self::SystemData) {
        if *paused == Paused::Paused {
            return;
        }
        for (dog,) in (&dogs,).join() {
            for (player,) in (&mut players,).join() {
                if dog.is_player_touching {
                    player.stop();
                }
                else 
                {
                    if player.speed == 0.0 {
                        player.normal_speed();
                    }
                }
            }
        }
    }
}
