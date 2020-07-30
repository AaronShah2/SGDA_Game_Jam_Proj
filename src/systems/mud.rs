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

const COLLISION_RADIUS: f32 = 510.0;
#[derive(SystemDesc)]
pub struct MudSystem;

impl<'s> System<'s> for MudSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
        ReadStorage<'s, Mud>,
        Read<'s, Paused>,
    );

    //TODO: Fix collision hitbox, slow down enemy
    fn run(&mut self, (transforms, mut players, enemies, mud, paused): Self::SystemData) {
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
        for ((_, enemy_transform), (_, mud_transform)) in
            (&enemies, &transforms).join().flat_map(|e| {
                (&mud, &transforms)
                    .join()
                    .map(|m| (e, m))
                    .collect::<Vec<_>>()
            })
        {
            if (enemy_transform.translation() - mud_transform.translation()).norm()
                <= COLLISION_RADIUS
            {
                log::info!("Collision between enemy and mud");
            }
        }
    }
}
