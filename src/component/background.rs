use specs::{Component, NullStorage};

#[storage(NullStorage)]
#[derive(Component, Default)]
pub struct Background;

#[storage(NullStorage)]
#[derive(Component, Default)]
pub struct Relative;