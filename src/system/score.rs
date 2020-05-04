use specs::{System, LazyUpdate, Entities, ReadStorage, Read, Join, Component, WriteStorage, Write};
use component::physics::{Position, BoundingBox, Velocity, Hitmap};
use vek::Aabr;
use config;
use component::virus::Virus;
use component::dino::{Dino, DinoSpeed};
use component::background::Relative;
use system::rules::{Rules, RulesState};
use system::render::Sprite;
use asset::AssetManager;


pub struct ScoreSystem;
impl<'a> System<'a> for ScoreSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, BoundingBox>,
        WriteStorage<'a, Velocity>,
        ReadStorage<'a, Virus>,
        ReadStorage<'a, Dino>,
        ReadStorage<'a, Relative>,
        ReadStorage<'a, Hitmap>,
        Write<'a, DinoSpeed>,
        Write<'a, Rules>,
        Read<'a, AssetManager>
    );

    fn run(&mut self, (pos, bb, mut vel, virus, dino, relative, hitmaps, mut speed, mut rules, asset): Self::SystemData) {
        for (pos_dino, bb_dino, dino, dino_hitmap) in (&pos, &bb, &dino, &hitmaps).join() {
            for (pos_virus, bb_virus, virus, virus_hitmap) in (&pos, &bb, &virus, &hitmaps).join() {
                if bb_dino.as_aabr(*pos_dino).collides_with_aabr(bb_virus.as_aabr(*pos_virus)) {
                    let intersection = bb_dino.as_aabr(*pos_dino).intersection(bb_virus.as_aabr(*pos_virus));
                    for x in (intersection.min.x as usize)..(intersection.max.x as usize) {
                        for y in (intersection.min.y as usize)..(intersection.max.y as usize) {
                            let x_dino_coordinate = x - pos_dino.0.x as usize;
                            let y_dino_coordinate = y - (600.0 - pos_dino.0.y - bb_dino.0.y) as usize;
                            let x_virus_coordinate = x - pos_virus.0.x as usize;
                            let y_virus_coordinate = y - (600.0 - pos_virus.0.y - bb_virus.0.y) as usize;

                            if asset[&dino_hitmap.asset].2[y_dino_coordinate*(bb_dino.0.x as usize) + x_dino_coordinate] & asset[&virus_hitmap.asset].2[y_virus_coordinate*(bb_virus.0.x as usize) + x_virus_coordinate] != 0 {
                                rules.state = RulesState::END;
                                return;
                            }
                        }
                    }
                }

                if pos_virus.x < 0.0 {
                    *speed += 0.2;
                    for (vel, back) in (&mut vel, &relative).join() {
                        vel.x -= 0.2;
                    }
                }
            }
        }
    }
}