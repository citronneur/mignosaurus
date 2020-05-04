use specs::{System, LazyUpdate, Entities, ReadStorage, Read, Join, Component};
use component::physics::{Position, BoundingBox};
use vek::Aabr;
use config;

/// This system delete all entitues that are
/// outside the screen
pub struct OffscreenSystem;
impl<'a> System<'a> for OffscreenSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BoundingBox>,
        Read<'a, LazyUpdate>
    );

    fn run(&mut self, (entities, pos, bb, updater): Self::SystemData) {
        for (entity, pos, bb) in (&*entities, &pos, &bb).join() {
            if !bb.as_aabr(*pos).collides_with_aabr(Aabr {
                min: Position::new(0.0, 0.0).0,
                max: Position::new(config::WIDTH, 600.0).0
            }) {
                entities.delete(entity).unwrap()
            }
        }
    }
}