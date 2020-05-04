use specs::{System, ReadStorage, WriteStorage, Join, Entities, Read, LazyUpdate, Entity};
use system::render::Sprite;
use component::physics::{Position, BoundingBox};

/// A system that connects sprites to the physics position.
pub struct SpritePositionSystem;
impl<'a> System<'a> for SpritePositionSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, BoundingBox>,
        WriteStorage<'a, Sprite>
    );

    fn run(&mut self, (pos, bb, mut sprite): Self::SystemData) {
        for (pos, bb, sprite) in (&pos, &bb, &mut sprite).join() {
            sprite.set_pos(pos.x as i32, (600.0 - pos.y - bb.y) as i32);
        }
    }
}