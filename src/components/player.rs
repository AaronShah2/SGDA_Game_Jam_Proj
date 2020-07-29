use amethyst::ecs::{Component, DenseVecStorage};

pub struct Player {
    id: usize,
}

impl Player {
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
