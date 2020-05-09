use system::render::{SpriteRef, load};
use component::physics::BoundingBox;
use std::collections::HashMap;
use blit::BlitBuffer;

pub mod dino_jump;
pub mod dino_run;
pub mod dino_run_1;
pub mod dino_run_2;
pub mod virus_1;
pub mod virus_2;
pub mod ground;
pub mod rocket;
pub mod cloud;
pub mod num1;
pub mod num2;
pub mod num3;
pub mod num4;
pub mod num5;
pub mod num6;
pub mod num7;
pub mod num8;
pub mod num9;
pub mod num0;
pub mod title;
pub mod touche;

pub type AssetManager<> = HashMap<String, (SpriteRef, BoundingBox, &'static[u32])>;


pub fn update(manager: &mut AssetManager, name: String, bitmap: &'static[u32], bbox: BoundingBox) {
    let sprite_ref = {
        let sprite = BlitBuffer::from_buffer(bitmap, bbox.x as i32, 0x0);
        // Load the sprite and get a reference
        load(sprite).unwrap()
    };

    manager.insert(name, (sprite_ref, bbox, bitmap));
}
