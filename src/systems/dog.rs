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