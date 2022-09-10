//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example layer

use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    game.logic.push(load);

    // We don't do anything after game setup, so our game logic can be an empty closure
    game.run(());
}

fn load(_: &mut Engine, state: &mut State<()>) {
    let mut layer = 0.0;
    let preset_iterator = SpritePreset::variant_iter().peekable();
    for (x, sprite_preset) in (-300..=600).step_by(30).zip(preset_iterator) {
        let mut sprite = state
            .repo
            .add_one(Sprite::new(format!("{:?}", sprite_preset), sprite_preset));
        sprite.translation = Vec2::new(x as f32, (-x) as f32);
        sprite.layer = layer; // 0.0 is the bottom (back) layer. 999.0 is the top (front) layer.
        layer += 1.0;
    }
}
