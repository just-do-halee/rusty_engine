//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example music_sampler

use rusty_engine::prelude::*;

struct GameState {
    music_index: usize,
}

fn main() {
    let mut game = Game::new();

    game.logic.push(load).push(logic);

    game.run(GameState { music_index: 0 });
}

fn load(_: &mut Engine, state: &mut State<GameState>) {
    let msg = state.repo.add_one(Sprite::new(
        "msg",
        "Press any key to advance to the next music selection.\n\nIf you are not running with \"--release\", it may take several seconds for each song to load!",
    ));
    msg.translation.y = -200.0;
}

fn logic(engine: &mut Engine, state: &mut State<GameState>) {
    let mut should_play_new_song = false;
    // Play a new song because a key was pressed
    for ev in engine.keyboard_events.drain(..) {
        if ev.state != ButtonState::Pressed {
            continue;
        }
        state.main.music_index = (state.main.music_index + 1) % MusicPreset::variant_iter().count();
        should_play_new_song = true;
        break;
    }

    if should_play_new_song || !engine.audio_manager.music_playing() {
        // Actually play the new song
        let music_preset = MusicPreset::variant_iter()
            .nth(state.main.music_index)
            .unwrap();
        engine.audio_manager.play_music(music_preset, 1.0);
        // And make text saying what song we're playing
        let note1 = state
            .repo
            .add_one(Text::new("note1", format!("Looping: {:?}", music_preset)));
        note1.font_size = 75.0;
    }
}
