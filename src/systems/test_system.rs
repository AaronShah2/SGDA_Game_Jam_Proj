// File copied from api

use amethyst::{
    prelude::*,
    input::{InputHandler, ControllerButton, VirtualKeyCode, StringBindings},
    core::SystemDesc,
    derive::SystemDesc,
    ecs::{Read, System, SystemData, World},
};

// Initializes System
#[derive(SystemDesc)]
pub struct TestSystem;
    
impl<'s> System<'s> for TestSystem {
    // The same BindingTypes from the InputBundle needs to be inside the InputHandler
    type SystemData = Read<'s, InputHandler<StringBindings>>;

    fn run(&mut self, input: Self::SystemData) {
        // Gets mouse coordinates
        if let Some((x, y)) = input.mouse_position() {
            //..
            // test if file works
            println!("x:{}, y:{}", x,y);
        }
        

        // Checks if the A button is down on the keyboard
        let buttonA = input.key_is_down(VirtualKeyCode::A);
        //..
    }
}