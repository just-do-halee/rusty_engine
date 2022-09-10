//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example sfx

//! This is an example of playing a sound effect preset. For playing your own sound effect file,
//! please see the `sound` example.

use rusty_engine::prelude::*;

fn main() {
    let mut game = Game::new();

    game.logic.push(load);

    game.run(());
}

fn load(engine: &mut Engine, state: &mut State<()>) {
    let msg = state.repo.add_one(Text::new(
        "msg",
        "You can play sound effect presets that are included in the asset pack. For example:",
    ));
    msg.translation.y = 50.0;

    let msg2 = state.repo.add_one(Text::new(
        "msg2",
        "engine.audio_manager.play_sfx(SfxPreset::Jingle1, 1.0);",
    ));
    msg2.translation.y = -50.0;
    msg2.font = "font/FiraMono-Medium.ttf".to_string();

    engine.audio_manager.play_sfx(SfxPreset::Jingle1, 1.0);
}
