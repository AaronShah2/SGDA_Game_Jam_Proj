// mud obj that slows player and enemy down
use crate::{
    components::{Enemy, Mud, Player},
    resources::Paused,
};
use amethyst::{
    core::Transform,
    derive::SystemDesc,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
};

const COLLISION_RADIUS: f32 = 600.0;
#[derive(SystemDesc)]
pub struct MudSystem;

impl<'s> System<'s> for MudSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        WriteStorage<'s, Enemy>,
        ReadStorage<'s, Mud>,
        Read<'s, Paused>,
    );

    //TODO: Fix collision hitbox, slow down enemy
    fn run(&mut self, (transforms, mut players, mut enemies, mud, paused): Self::SystemData) {
        // Pauses game
        if *paused == Paused::Paused {
            return;
        }
        // checks if player collides with mud
        for (player, player_transform) in (&mut players, &transforms).join() {
            for (_, mud_transform) in (&mud, &transforms).join() {
                if (player_transform.translation() - mud_transform.translation()).norm()
                    <= COLLISION_RADIUS
                {
                    player.slow_down();
                } else {
                    player.normal_speed();
                }
            }
        }
        // checks if enemy collides with mud
        for (enemy, enemy_transform) in (&mut enemies, &transforms).join() {
            for (_, mud_transform) in (&mud, &transforms).join() {
                if (enemy_transform.translation() - mud_transform.translation()).norm()
                    <= COLLISION_RADIUS
                {
                    enemy.slow_down();
                } else {
                    enemy.normal_speed();
                }
            }
        }
    }
}
