extern crate miniquad;
extern crate specs;
extern crate blit;
extern crate vek;
extern crate anyhow;
extern crate lazy_static;
extern crate rayon;
extern crate rotsprite;

mod system;
mod component;
mod render;
mod game;
mod asset;
mod config;
use miniquad::*;
use game::Game;

#[cfg(target_os = "linux")]
extern "C" {
    // Seed random when on Linux
    fn srand(input: u32);
}
#[cfg(not(target_os = "linux"))]
fn srand(_: u32) {}

fn main() {
    unsafe {
        srand(miniquad::date::now() as u32);
    }
    miniquad::start(conf::Conf::default(), |mut ctx| {
        UserData::owning(Game::new(&mut ctx), ctx)
    });
}

