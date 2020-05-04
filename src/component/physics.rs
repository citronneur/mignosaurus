use specs::{System, ReadStorage, WriteStorage, Component, VecStorage, Join, Read};
use std::ops::{Deref, DerefMut};
use config;

pub type DeltaTime = f64;

type Vec2 = vek::Vec2<f64>;
type Aabr = vek::Aabr<f64>;

#[storage(VecStorage)]
#[derive(Component, Default, Copy, Clone)]
pub struct Position(pub Vec2);

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl Deref for Position {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[storage(VecStorage)]
#[derive(Component, Default)]
pub struct Velocity(pub Vec2);

impl Velocity {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl Deref for Velocity {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Velocity {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[storage(VecStorage)]
#[derive(Component, Default)]
pub struct Force(pub Vec2);

impl Force {
    pub fn new(x: f64, y: f64) -> Self {
        Self(Vec2::new(x, y))
    }
}

impl Deref for Force {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Force {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[storage(VecStorage)]
#[derive(Component, Default, Clone, Copy)]
pub struct BoundingBox(pub Vec2);

impl BoundingBox {
    pub fn new(width: f64, height: f64) -> Self {
        Self(Vec2::new(width, height))
    }

    pub fn as_aabr(&self, mut pos: Position) -> Aabr {
        pos.y = 600.0 - pos.y - self.y;
        Aabr {
            min: pos.0,
            max: pos.0 + self.0
        }
    }
}

impl Deref for BoundingBox {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[storage(VecStorage)]
#[derive(Component, Default)]
pub struct Hitmap {
    pub asset: String
}

impl Hitmap {
    pub fn new(name: String) -> Self {
        Hitmap {
            asset: name
        }
    }
}

