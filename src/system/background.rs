use specs::{System, Entities, ReadStorage, Join, Read, LazyUpdate};
use component::physics::{Position, BoundingBox, Velocity, DeltaTime};
use asset::AssetManager;
use system::render::Sprite;
use component::background::{Background, Relative};
use system::rules::Rules;

/// Background system
pub struct BackgroundSystem;
impl<'a> System<'a> for BackgroundSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, AssetManager>,
        Read<'a, Rules>,
        ReadStorage<'a, Background>,
        ReadStorage<'a, Position>
    );

    fn run(&mut self, (entities, updater, asset_manager, rules, background, pos): Self::SystemData) {
        let mut min_x = -1020.0;

        for (background, pos) in (&background, &pos).join() {
            if pos.x > min_x {
                min_x = pos.x;
            }
        }

        if min_x <= 0.0 {
            let entity = entities.create();
            updater.insert(entity, Sprite::new(asset_manager["ground"].0.clone(), 0));
            updater.insert(entity, Position::new(1022.0 + min_x, 180.0));
            updater.insert(entity, asset_manager["ground"].1.clone());
            updater.insert(entity, Velocity::new(-300.0 - rules.rel_speed, 0.0));
            updater.insert(entity, Background);
            updater.insert(entity, Relative);
        }
    }
}


pub struct RocketSystem {
    delta: f64,
    tempo: f64,
    asset: String,
    speed: f64
}

impl RocketSystem {
    pub fn new(tempo: f64, asset: String, speed: f64) -> Self {
        RocketSystem {
            delta: 0.0,
            tempo,
            asset,
            speed
        }
    }
}

impl<'a> System<'a> for RocketSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, AssetManager>,
        Read<'a, DeltaTime>,
        Read<'a, Rules>,
    );

    fn run(&mut self, (entities, updater, asset_manager, delta_time, rules): Self::SystemData) {
        self.delta += (*delta_time);
        if self.delta > self.tempo {
            self.delta = 0.0;
        }
        if self.delta == 0.0 {
            let entity = entities.create();
            updater.insert(entity, Sprite::new(asset_manager[&self.asset].0.clone(), 0));
            updater.insert(entity, Position::new(1023.0, 380.0));
            updater.insert(entity, asset_manager[&self.asset].1.clone());
            updater.insert(entity, Velocity::new(-self.speed - rules.rel_speed, 0.0));
            updater.insert(entity, Relative);
        }
    }
}