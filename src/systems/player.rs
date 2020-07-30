use crate::{components::Player, resources::Paused};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};
use nalgebra::base::Vector3;

const MOVE_SPEED: f32 = 10.0f32;
const AREA_WIDTH: f32 = 760.0f32;

#[derive(SystemDesc)]
pub struct PlayerSystem;

impl<'s> System<'s> for PlayerSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Paused>,
    );

    fn run(&mut self, (mut transforms, players, input, paused): Self::SystemData) {
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
                transform.prepend_translation(movement.normalize() * MOVE_SPEED);
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
