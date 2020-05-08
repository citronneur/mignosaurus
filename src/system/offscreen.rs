use specs::{System, LazyUpdate, Entities, ReadStorage, Read, Join, Component, Write};
use component::physics::{Position, BoundingBox};
use vek::Aabr;
use config;
use system::rules::Rules;
use component::virus::Virus;

/// This system delete all entitues that are
/// outside the screen
pub struct OffscreenSystem;
impl<'a> System<'a> for OffscreenSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, BoundingBox>,
        Read<'a, LazyUpdate>,
        Write<'a, Rules>,
        ReadStorage<'a, Virus>
    );

    fn run(&mut self, (entities, pos, bb, updater, mut rules, virus): Self::SystemData) {
        for (entity, pos, bb, virus_storage) in (&*entities, &pos, &bb, (&virus).maybe()).join() {
            if !bb.as_aabr(*pos).collides_with_aabr(Aabr {
                min: Position::new(0.0, 0.0).0,
                max: Position::new(config::WIDTH, 600.0).0
            }) {
                entities.delete(entity).unwrap();
                if let Some(_virus) = virus_storage {
                    rules.score += 1;
                }
            }
        }
    }
}