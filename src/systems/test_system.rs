// File copied from api

use amethyst::{
    prelude::*,
    core::{Transform, SystemDesc},
    derive::SystemDesc,
    ecs::{Component, DenseVecStorage, Join, Read, ReadStorage, System, SystemData, World, WriteStorage},
    input::{InputHandler, StringBindings},
};

// Copied from sample
pub struct Player {
    id: usize,
}

impl Player {
    // Test function
    pub fn shoot(&self) {
        println!("PEW! {}", self.id);
    }
    
    pub fn new() -> Self {
        Player {
            id: 0,
        }
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

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
// // Initializes System
// #[derive(SystemDesc)]
// pub struct TestSystem;
    
// impl<'s> System<'s> for TestSystem {
//     // The same BindingTypes from the InputBundle needs to be inside the InputHandler
//     type SystemData = Read<'s, InputHandler<StringBindings>>;

//     fn run(&mut self, input: Self::SystemData) {
//         // Gets mouse coordinates
//         if let Some((x, y)) = input.mouse_position() {
//             //..
//             // test if file works
//             println!("x:{}, y:{}", x,y);
//         }
        

//         // Checks if the A button is down on the keyboard
//         let buttonA = input.key_is_down(VirtualKeyCode::A);
//         //..
//     }
// }