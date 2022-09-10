//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example sfx_sampler

use rusty_engine::prelude::*;

#[derive(Default)]
struct GameState {
    sfx_timers: Vec<Timer>,
    end_timer: Timer,
}

fn main() {
    let mut game = Game::new();
    game.logic.push(load).push(logic);
    game.run(GameState::default());
}

fn load(_: &mut Engine, state: &mut State<GameState>) {
    // One timer to launch each sound effect
    for (i, _sfx) in SfxPreset::variant_iter().enumerate() {
        state
            .main
            .sfx_timers
            .push(Timer::from_seconds((i as f32) * 2.0, false));
    }
    // One timer to end them all
    state.main.end_timer =
        Timer::from_seconds((SfxPreset::variant_iter().len() as f32) * 2.0 + 1.0, false);

    let mut msg = state
        .repo
        .add_one(Text::new("msg", "Playing sound effects!"));
    msg.translation = Vec2::new(0.0, 100.0);
    msg.font_size = 60.0;

    let mut sfx_label = state.repo.add_one(Text::new("sfx_label", ""));
    sfx_label.translation = Vec2::new(0.0, -100.0);
    sfx_label.font_size = 90.0;
}

fn logic(engine: &mut Engine, state: &mut State<GameState>) {
    for (i, timer) in state.main.sfx_timers.iter_mut().enumerate() {
        // None of the timers repeat, and they're all set to different times, so when the timer in
        // index X goes off, play sound effect in index X
        if timer.tick(engine.delta).just_finished() {
            // Play a new sound effect
            let sfx = SfxPreset::variant_iter().nth(i).unwrap();
            engine.audio_manager.play_sfx(sfx, 1.0);
            // Update the text to show which sound effect we are playing
            let sfx_label = state.repo.get_one_mut::<Text>("sfx_label").unwrap();
            sfx_label.value = format!("{:?}", sfx);
        }
    }

    // Are we all done?
    if state.main.end_timer.tick(engine.delta).just_finished() {
        let sfx_label = state.repo.get_one_mut::<Text>("sfx_label").unwrap();
        sfx_label.value = "That's all! Press Esc to quit.".into();
    }
}
