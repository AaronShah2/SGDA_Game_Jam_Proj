use crate::{
    resources::prefabs::initialize_prefabs,
    states::MenuState,
};

use amethyst::{
    assets::ProgressCounter,
    prelude::*,
};

#[derive(Default)]
pub struct LoadingState {
    prefab_progress: Option<ProgressCounter>,
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, mut data: StateData<GameData>) {
        self.prefab_progress = Some(initialize_prefabs(&mut data.world));
    }

    fn update(&mut self, data: &mut StateData<GameData>) -> SimpleTrans {
        data.data.update(&data.world);
        if let Some(ref counter) = self.prefab_progress.as_ref() {
            println!(
                "Loading: {}, Failed: {}, Finished: {}",
                counter.num_loading(),
                counter.num_failed(),
                counter.num_finished(),
            );
            use amethyst::assets::Completion::*;
            match counter.complete() {
                Complete => {
                    self.prefab_progress = None;
                    return Trans::Switch(Box::new(MenuState::default()));
                },
                Failed => {
                    panic!("Some prefabs have failed :(\n\nErrors:\n{:#?}", counter.errors());
                },
                Loading => {},
            }
        }
        return Trans::None;
    }
}
