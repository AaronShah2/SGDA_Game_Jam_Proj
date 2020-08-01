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

#[derive(SystemDesc)]
pub struct MudSystem;

impl<'s> System<'s> for MudSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Mud>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, Enemy>,
        Read<'s, Paused>,
    );

    //TODO: Fix collision hitbox
    fn run(&mut self, (transforms, mut muds, players, enemies, paused): Self::SystemData) {
        // Pauses game
        if *paused == Paused::Paused {
            return;
        }
        for (mud, mud_transform) in (&mut muds, &transforms).join() {
            for (_, player_transform) in (&players, &transforms).join() {
                // log::info!("player_coor: {}, mud_coor: {}",
                // player_transform.translation(), mud_transform.translation());

                // keeps track of distance between mud and player
                let x = player_transform.translation().x - mud_transform.translation().x;
                let y = player_transform.translation().y - mud_transform.translation().y;

                // checks if within boundaries
                if x >= -(mud.width) && x <= mud.width && y >= -(mud.height) && y <= mud.height {
                    mud.is_player_touching = true;
                } else {
                    mud.is_player_touching = false;
                }
            }
            for (_, enemy_transform) in (&enemies, &transforms).join() {
                // keeps track of distance between mud and player
                let x = enemy_transform.translation().x - mud_transform.translation().x;
                let y = enemy_transform.translation().y - mud_transform.translation().y;

                
                // checks if within boundaries
                if x >= -(mud.width) && x <= mud.width && y >= -(mud.height) && y <= mud.height {
                    mud.is_enemy_touching = true;
                } else {
                    mud.is_enemy_touching = false;
                }
            }
        }
    }
}
