use specs::{System, LazyUpdate, Entities, ReadStorage, Read, Join, Component, WriteStorage, Write};
use component::physics::{Position, BoundingBox, Velocity, Hitmap};
use vek::Aabr;
use config;
use component::virus::Virus;
use component::dino::Dino;
use component::background::Relative;
use system::rules::{Rules, RulesState};
use system::render::Sprite;
use asset::AssetManager;
use component::score::ScoreSprite;


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
        Write<'a, Rules>,
        Read<'a, AssetManager>
    );

    fn run(&mut self, (pos, bb, mut vel, virus, dino, relative, hitmaps, mut rules, asset): Self::SystemData) {
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

                let old_speed = rules.rel_speed;
                rules.rel_speed = rules.score as f64 * 5.0;
                if rules.rel_speed - old_speed > 0.0 {
                    for (vel, back) in (&mut vel, &relative).join() {
                        vel.x -= rules.rel_speed - old_speed;
                    }
                }
            }
        }
    }
}

pub struct ScoreSpriteSystem {
    pub last_score: u32
}

impl ScoreSpriteSystem {
    pub fn new() -> Self {
        ScoreSpriteSystem {
            last_score: 0
        }
    }
}
impl<'a> System<'a> for ScoreSpriteSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, Rules>,
        Read<'a, AssetManager>,
        ReadStorage<'a, ScoreSprite>
    );

    fn run(&mut self, (entities, updater, rules, asset_manager, score_sprites): Self::SystemData) {
        if self.last_score == rules.score {
            return
        }

        self.last_score = rules.score;
        for (entity, score_sprite) in (&*entities, &score_sprites).join() {
            entities.delete(entity).unwrap();
        }
        let mut y_offset = config::WIDTH - 80.0;
        let mut score = rules.score;

        while score > 0 {
            let num = score % 10;
            let index = match num {
                0 => "0",
                1 => "1",
                2 => "2",
                3 => "3",
                4 => "4",
                5 => "5",
                6 => "6",
                7 => "7",
                8 => "8",
                9 => "9",
                _ => "1"
            };

            let entity = entities.create();
            updater.insert(entity, ScoreSprite);
            updater.insert(entity, Sprite::new(asset_manager[index].0.clone(), config::ZDINO + 1));
            updater.insert(entity, Position::new(y_offset, config::HEIGHT - 110.0));
            updater.insert(entity, asset_manager[index].1.clone());

            score = score / 10;
            y_offset -= 80.0;
        }
    }
}