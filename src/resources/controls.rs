use amethyst::{
    input::{Axis, Bindings, Button, InputHandler, StringBindings, VirtualKeyCode},
    prelude::*,
};

/// An enum containing control schemes for the game
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Controls {
    Arrows,
    Wasd,
}
impl Default for Controls {
    fn default() -> Self {
        Controls::Arrows
    }
}
impl Controls {
    /// Get the label to show on the controls button in the options
    pub fn get_button_label(&self) -> &str {
        match self {
            Controls::Arrows => "Controls: Arrow keys",
            Controls::Wasd => "Controls: WASD",
        }
    }

    /// Get the next control scheme (to be used when clicking the
    /// controls button)
    pub fn successor(&self) -> Self {
        match self {
            Controls::Arrows => Controls::Wasd,
            Controls::Wasd => Controls::Arrows,
        }
    }

    /// Update the world's controls to use the controls specified by
    /// this instance
    pub fn set_control_scheme(&self, world: &mut World) {
        let bindings = &mut world
            .write_resource::<InputHandler<StringBindings>>()
            .bindings;
        match self {
            Controls::Arrows => set_arrows_bindings(bindings),
            Controls::Wasd => set_wasd_bindings(bindings),
        }
    }
}

/// Gets the bindings for arrow keys controls
fn set_arrows_bindings(bindings: &mut Bindings<StringBindings>) {
    bindings
        .insert_axis(
            "vertical",
            Axis::Emulated {
                pos: Button::Key(VirtualKeyCode::Up),
                neg: Button::Key(VirtualKeyCode::Down),
            },
        )
        .expect("error binding controls");
    bindings
        .insert_axis(
            "horizontal",
            Axis::Emulated {
                pos: Button::Key(VirtualKeyCode::Right),
                neg: Button::Key(VirtualKeyCode::Left),
            },
        )
        .expect("error binding controls");
}

/// Gets the bindings for wasd controls
fn set_wasd_bindings(bindings: &mut Bindings<StringBindings>) {
    bindings
        .insert_axis(
            "vertical",
            Axis::Emulated {
                pos: Button::Key(VirtualKeyCode::W),
                neg: Button::Key(VirtualKeyCode::S),
            },
        )
        .expect("error binding controls");
    bindings
        .insert_axis(
            "horizontal",
            Axis::Emulated {
                pos: Button::Key(VirtualKeyCode::D),
                neg: Button::Key(VirtualKeyCode::A),
            },
        )
        .expect("error binding controls");
}
