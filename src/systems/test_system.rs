// File copied from api

use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::components::Player;

#[derive(SystemDesc)]
pub struct TestSystem;

impl<'s> System<'s> for TestSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Player>,
        Read<'s, InputHandler<StringBindings>>,
    );
    
    fn run(&mut self, (mut transforms, players, input): Self::SystemData) {
        for (player, transform) in (&players, &mut transforms).join() {

            // unwraps elements from inputs.ron
            let horizontal = input.axis_value("horizontal").unwrap_or(0.0);
            let vertical = input.axis_value("vertical").unwrap_or(0.0);
            
            // lets player move up and down
            transform.move_up(vertical);
            transform.move_right(horizontal);

            // test function, need to remove
            let shoot = input.action_is_down("shoot").unwrap_or(false);
            if shoot {
                player.shoot();
            }
        }
    }
}
