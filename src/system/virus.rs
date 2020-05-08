use system::render::{SpriteRef, load, Sprite};
use specs::{World, System, ReadStorage, WriteStorage, LazyUpdate, Read, Entities, Component, VecStorage, Join, NullStorage};
use blit::BlitBuffer;
use component::physics::{Position, BoundingBox, Velocity, DeltaTime, Force, Hitmap};
use system::dino::Jump;
use std::f64::consts;
use component::virus::Virus;
use miniquad::*;
use config;
use asset::AssetManager;
use component::background::Relative;
use system::rules::Rules;


pub struct VirusSystem {
    scenario: Vec<(f64, String, Virus)>,
    last_virus: usize
}

impl VirusSystem {
    pub fn new() -> Self {
        VirusSystem {
            last_virus: 0,
            scenario : vec![
                (800.0, "virus1".to_string(), Virus::straight_easy_bottom()),
                (600.0, "virus2".to_string(), Virus::straight_easy_bottom()),
                (600.0, "virus1".to_string(), Virus::straight_easy_bottom()),
                (600.0, "virus2".to_string(), Virus::sin_easy_bottom()),
                (800.0, "virus1".to_string(), Virus::sin_easy_bottom()),
                (600.0, "virus2".to_string(), Virus::sin_easy_bottom()),
                (600.0, "virus1".to_string(), Virus::sin_hard_bottom()),
                (600.0, "virus2".to_string(), Virus::sin_hard_top()),
                (600.0, "virus1".to_string(), Virus::sin_hard_top())
            ]
        }
    }

    pub fn update(&mut self, asset_manager: &AssetManager, delta_time: f64, last_x: f64, entities: &Entities, updater: &Read<LazyUpdate>, speed: f64) {
        let config = self.scenario[self.last_virus % self.scenario.len()].clone();
        if config::WIDTH - last_x > config.0 {
            self.add_virus(asset_manager[&config.1].0.clone(), asset_manager[&config.1].1.clone(), entities, updater, config.2, speed, config.1);
            self.last_virus += 1 + (delta_time * 10000.0) as usize % 5;
        }
    }

    pub fn add_virus(&mut self, sprite: SpriteRef, bbox: BoundingBox, entities: &Entities, updater: &Read<LazyUpdate>, mut virus: Virus, speed: f64, name: String) {
        let entity = entities.create();
        updater.insert(entity, virus);
        updater.insert(entity, Sprite::new(sprite, config::ZDINO + 1));
        updater.insert(entity, Position::new(config::WIDTH - 10.0, virus.offset));
        updater.insert(entity, bbox);
        updater.insert(entity, Velocity::new(-450.0 + speed, 0.0));
        updater.insert(entity, Force::new(0.0, 0.0));
        updater.insert(entity, Relative);
        updater.insert(entity, Hitmap::new(name));
    }
}

impl<'a> System<'a> for VirusSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, DeltaTime>,
        Read<'a, AssetManager>,
        ReadStorage<'a, Virus>,
        ReadStorage<'a, Position>,
        Read<'a, Rules>
    );

    fn run(&mut self, (entities, updater, delta_time, asset_manager, virus, pos, rules): Self::SystemData) {
        let mut min_x = 0.0;

        for (vir, pos) in (&virus, &pos).join() {
            if pos.x > min_x {
                min_x = pos.x;
            }
        }

        self.update(&(*asset_manager), *delta_time, min_x, &entities, &updater, -1.0 * rules.rel_speed);
    }
}

pub struct VirusMotionSystem;
impl<'a> System<'a> for VirusMotionSystem {
    type SystemData = (
        Read<'a, DeltaTime>,
        WriteStorage<'a, Virus>,
        WriteStorage<'a, Velocity>
    );

    fn run(&mut self, (delta_time, mut virus, mut vel): Self::SystemData) {
        for (vir, vel) in (&mut virus, &mut vel).join() {
            vel.y = 6.0 * (vir.lifetime * 6.0).cos() * vir.amplitude;
            vir.lifetime += (*delta_time);
        }
    }
}