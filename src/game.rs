use render::Render;
use miniquad::*;
use blit::{BlitBuffer, Color};
use component::physics::{Position, Velocity, Force, BoundingBox, DeltaTime, Hitmap};
use system::physics::{GravitySystem, NewtonSystem, VelocitySystem};
use system::sprite::{SpritePositionSystem};
use system::dino::{Jump, DinoSystem, DinoAnimationSystem};
use std::time::Duration;
use system::offscreen::OffscreenSystem;
use component::virus::{Virus};
use component::dino::{Dino, DinoSpeed};
use system::virus::{VirusSystem, VirusMotionSystem};
use config;
use asset;
use asset::AssetManager;
use component::background::{Background, Relative};
use system::background::{BackgroundSystem, RocketSystem};
use system::render::{RenderSystem, PixelBuffer, Sprite, load};
use specs::prelude::*;
use system::score::ScoreSystem;
use system::rules::{Rules, RulesSystem};


pub struct Game<'a, 'b> {
    render: Render,
    world: World,
    dispatcher: Dispatcher<'a, 'b>,
    last_update: f64
}

impl<'a, 'b> Game<'a, 'b> {
    pub fn new(ctx: &mut Context) -> Self {
        let mut world = World::new();
        world.insert(PixelBuffer::new(config::WIDTH as usize, 600));
        world.insert(Jump::from(false));
        world.insert(DeltaTime::from(0.0 as f64));
        world.insert(DinoSpeed::from(0.0));
        world.insert(Rules::new());

        // Load the sprite rendering component
        world.register::<Sprite>();
        world.register::<Velocity>();
        world.register::<Force>();
        world.register::<Position>();
        world.register::<BoundingBox>();
        world.register::<Virus>();
        world.register::<Dino>();
        world.register::<Background>();
        world.register::<Relative>();
        world.register::<Hitmap>();

        let mut asset_manager = AssetManager::new();

        asset::update(&mut asset_manager, "dino_run".to_string(), asset::dino_run::ASSET, BoundingBox::new(120.0, 162.0));
        asset::update(&mut asset_manager, "dino_run_1".to_string(), asset::dino_run_1::ASSET, BoundingBox::new(120.0, 162.0));
        asset::update(&mut asset_manager, "dino_run_2".to_string(), asset::dino_run_2::ASSET, BoundingBox::new(120.0, 162.0));
        asset::update(&mut asset_manager, "dino_jump".to_string(), asset::dino_jump::ASSET, BoundingBox::new(117.0, 133.0));
        asset::update(&mut asset_manager, "virus1".to_string(), asset::virus_1::ASSET, BoundingBox::new(57.0, 56.0));
        asset::update(&mut asset_manager, "virus2".to_string(), asset::virus_2::ASSET, BoundingBox::new(78.0, 67.0));
        asset::update(&mut asset_manager, "ground".to_string(), asset::ground::ASSET, BoundingBox::new(1024.0, 32.0));
        asset::update(&mut asset_manager, "rocket".to_string(), asset::rocket::ASSET, BoundingBox::new(85.0, 66.0));
        asset::update(&mut asset_manager, "cloud".to_string(), asset::cloud::ASSET, BoundingBox::new(174.0, 111.0));

        world.create_entity()
            .with(Sprite::new(asset_manager["ground"].0.clone(), 0))
            .with(Position::new(-50.0, 180.0))
            .with(BoundingBox::new(1024.0, 32.0))
            .with(Velocity::new(-300.0, 0.0))
            .with(Background)
            .with(Relative)
            .build();

        world.create_entity()
            .with(Sprite::new(asset_manager["dino_run"].0.clone(), config::ZDINO))
            .with(Position::new(50.0, 60.0))
            .with(BoundingBox::new(120.0, 162.0))
            .with(Velocity::new(0.0, 0.0))
            .with(Force::new(0.0, -4500.0))
            .with(Dino)
            .with(Hitmap::new("dino_run".to_string()))
            .build();

        world.insert(asset_manager);

        let dispatcher = DispatcherBuilder::new()
            .with(DinoSystem::new(), "dino", &[])
            .with(VirusMotionSystem, "virusmotion", &[])
            .with(NewtonSystem, "newton", &["virusmotion"])
            .with(VelocitySystem, "velocity", &["newton"])
            .with(GravitySystem, "gravity", &["velocity"])
            .with(OffscreenSystem, "offscreen", &["velocity"])
            .with(VirusSystem::new(), "virus", &[])
            .with(DinoAnimationSystem::new(), "dinoanimationsystem", &["gravity"])
            .with(SpritePositionSystem, "spriteposition", &["velocity"])
            .with(ScoreSystem, "scoresystem", &["spriteposition"])
            .with(RulesSystem, "rules", &["spriteposition"])
            .with(BackgroundSystem, "background", &["spriteposition"])
            .with(RocketSystem::new(24.0, "rocket".to_string(), 50.0), "rocket", &[])
            .with(RocketSystem::new(8.0, "cloud".to_string(), 100.0), "cloud", &[])
            .with_thread_local(RenderSystem)
            .build();

        Game {
            render: Render::new(ctx, config::WIDTH as usize, 600),
            world,
            dispatcher,
            last_update: date::now()
        }
    }
}

impl<'a, 'b> EventHandler for Game<'a, 'b> {

    fn update(&mut self, ctx: &mut Context) {
        let now = date::now();
        *self.world.write_resource::<DeltaTime>() = now - self.last_update;
        self.last_update = now;
        self.dispatcher.dispatch(&self.world);
        self.world.maintain();
    }

    fn draw(&mut self, ctx: &mut Context) {
        let mut buffer = self.world.write_resource::<PixelBuffer>();
        self.render.render(ctx, &buffer);
        buffer.clear(0xFFFFFFFF);

    }

     fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        _keycode: KeyCode,
        _keymods: KeyMods,
        _repeat: bool,
    ) {
        *self.world.write_resource::<Jump>() = true;
    }

    fn key_up_event(&mut self, _ctx: &mut Context, _keycode: KeyCode, _keymods: KeyMods) {
        *self.world.write_resource::<Jump>() = false;
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        *self.world.write_resource::<Jump>() = true;
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        _button: MouseButton,
        _x: f32,
        _y: f32,
    ) {
        *self.world.write_resource::<Jump>() = false;
    }
}
