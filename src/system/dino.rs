use specs::{System, Read, WriteStorage, ReadStorage, Join, Entities, LazyUpdate};
use component::physics::{Velocity, Position, DeltaTime, BoundingBox, Hitmap};
use component::dino::{Dino};
use config;
use system::render::Sprite;
use asset::AssetManager;
use system::rules::{RulesState, Rules};

pub type Jump = bool;

pub struct DinoSystem {
    time: f64
}

impl DinoSystem {
    pub fn new() -> Self {
        DinoSystem {
            time: 0.0
        }
    }
}

impl<'a> System<'a> for DinoSystem {
    type SystemData = (
        Read<'a, Jump>,
        Read<'a, DeltaTime>,
        ReadStorage<'a, Dino>,
        ReadStorage<'a, Position>,
        WriteStorage<'a, Velocity>
    );

    fn run(&mut self, (jump, delta_time, dino, pos, mut vol): Self::SystemData) {
        for (dino, pos, vol) in (&dino, &pos, &mut vol).join() {
            if *jump {
                if vol.y == 0.0 {
                    self.time = 0.0;
                    vol.y += 1250.0;
                }
                if vol.y > 0.0 {
                    vol.y += 2550.0 * (-1.0 * self.time).exp() * (*delta_time);
                    self.time += *delta_time;
                }
            }
        }
    }
}

pub struct DinoAnimationSystem{
    rate: f64
}

impl DinoAnimationSystem {
    pub fn new() -> Self {
        DinoAnimationSystem {
            rate: 0.0
        }
    }
}

impl<'a> System<'a> for DinoAnimationSystem {
    type SystemData = (
        Read<'a, LazyUpdate>,
        Read<'a, AssetManager>,
        Read<'a, DeltaTime>,
        Entities<'a>,
        ReadStorage<'a, Dino>,
        ReadStorage<'a, Velocity>,
        Read<'a, Rules>,
    );

    fn run(&mut self, (updater, manager, delta_time, entities, dino, vol, rules): Self::SystemData) {
        for (entity, dino, vol) in (&entities, &dino, &vol).join() {
            if rules.state == RulesState::PENDING {
                return;
            }

            if vol.y.abs() > 0.0 {
                updater.remove::<Sprite>(entity);
                updater.remove::<BoundingBox>(entity);
                updater.insert(entity, Sprite::new(manager["dino_jump"].0.clone(), config::ZDINO));
                updater.insert(entity, manager["dino_jump"].1.clone());
                updater.insert(entity, Hitmap::new("dino_jump".to_string()));
            }
            else {
                self.rate += *delta_time;
                if self.rate > 0.4 { self.rate = 0.0}
                if self.rate > 0.2 {
                    updater.remove::<Sprite>(entity);
                    updater.remove::<BoundingBox>(entity);
                    updater.insert(entity, Sprite::new(manager["dino_run_1"].0.clone(), config::ZDINO));
                    updater.insert(entity, manager["dino_run_1"].1.clone());
                    updater.insert(entity, Hitmap::new("dino_run_1".to_string()));
                }
                else  {
                    updater.remove::<Sprite>(entity);
                    updater.remove::<BoundingBox>(entity);
                    updater.insert(entity, Sprite::new(manager["dino_run_2"].0.clone(), config::ZDINO));
                    updater.insert(entity, manager["dino_run_2"].1.clone());
                    updater.insert(entity, Hitmap::new("dino_run_2".to_string()));
                }
            }
        }
    }
}