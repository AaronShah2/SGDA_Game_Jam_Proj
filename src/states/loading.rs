use crate::{
    resources::{
        audio::initialize_audio, prefabs::initialize_prefabs, sprites::initialize_sprite_sheets,
    },
    states::MenuState,
};

use amethyst::{assets::ProgressCounter, prelude::*};

#[derive(Default)]
pub struct LoadingState {
    counters: Vec<ProgressCounter>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        // Loads Sprites & Prefabs
        // Counters determine time it will take for everything to be loaded
        self.counters.push(initialize_prefabs(&mut data.world));
        self.counters
            .push(initialize_sprite_sheets(&mut data.world));
        self.counters.push(initialize_audio(&mut data.world));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);

        // number of assets being loaded
        let loading: usize = self.counters.iter().map(ProgressCounter::num_loading).sum();

        // number of assets that have failed to load
        let failed: usize = self.counters.iter().map(ProgressCounter::num_failed).sum();

        // number of assets that have finished loading
        let finished: usize = self
            .counters
            .iter()
            .map(ProgressCounter::num_finished)
            .sum();
        println!(
            "Loading: {}, Failed: {}, Finished: {}",
            loading, failed, finished,
        );

        // checks if there are no more assets that need to be loaded
        if loading == 0 {
            // if certain assets fail to load, then return an error
            if failed != 0 {
                panic!(
                    "Failed loading assets :(\n\nErrors:\n{:#?}",
                    self.counters
                        .iter()
                        .map(|pc| pc.errors().into_iter())
                        .flatten()
                        .collect::<Vec<_>>()
                );
            }
            self.counters.clear();

            // Switches to Menu
            Trans::Switch(Box::new(MenuState::default()))
        // Trans::Switch(Box::new(super::Test::default()))
        } else {
            Trans::None
        }
    }
}
