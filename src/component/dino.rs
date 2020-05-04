use specs::{Component, NullStorage};

#[storage(NullStorage)]
#[derive(Component, Default)]
pub struct Dino;

pub type DinoSpeed = f32;