use component::ihm::IhmElement;
use specs::{System, Entities, Read, LazyUpdate, ReadStorage, Join};
use system::rules::{Rules, RulesState};
use asset::AssetManager;
use system::render::Sprite;
use component::physics::Position;
use config;

pub struct IhmSystem;
impl<'a> System<'a> for IhmSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, LazyUpdate>,
        Read<'a, Rules>,
        Read<'a, AssetManager>,
        ReadStorage<'a, IhmElement>
    );

    fn run(&mut self, (entities, updater, rules, asset_manager, elements): Self::SystemData) {

        match rules.state {
            RulesState::RUN => {
                for (entity, element) in (&*entities, &elements).join() {
                    entities.delete(entity).unwrap();
                }
            }
            RulesState::PENDING => {
                let entity = entities.create();
                updater.insert(entity, IhmElement);
                updater.insert(entity, Sprite::new(asset_manager["title"].0.clone(), config::ZDINO + 1));
                updater.insert(entity, Position::new(54.0, 192.0));
                updater.insert(entity, asset_manager["title"].1.clone());
            }
            RulesState::END => {
                let entity = entities.create();
                updater.insert(entity, IhmElement);
                updater.insert(entity, Sprite::new(asset_manager["touche"].0.clone(), config::ZDINO + 1));
                updater.insert(entity, Position::new(289.0, 217.0));
                updater.insert(entity, asset_manager["touche"].1.clone());
            }
        }
    }
}