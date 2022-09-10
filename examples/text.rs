//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example text

use rusty_engine::prelude::*;

struct GameState {
    timer: Timer,
}

fn main() {
    let mut game = Game::new();
    game.logic.push(load).push(game_logic);
    let game_state = GameState {
        timer: Timer::from_seconds(0.2, true),
    };
    game.run(game_state);
}

fn load(_: &mut Engine, state: &mut State<GameState>) {
    let fps = state.repo.add_one(Text::new("fps", "FPS: "));
    fps.translation = Vec2::new(0.0, 250.0);
    fps.font = "font/FiraMono-Medium.ttf".to_string();
    fps.font_size = 60.0;

    let zoom_msg = state.repo.add_one(Text::new(
        "zoom_msg",
        "Changing font size re-renders the text smoothly at a different size,\nbut using this technique for animation is both jittery (character kerning) and expensive.")
    );
    zoom_msg.font_size = 35.0;
    zoom_msg.translation = Vec2::new(0.0, 150.0);

    let font_msg = state.repo.add_one(Text::new(
        "font_msg",
        "You can choose a font at creation time by providing the filename of a font stored in assets/.\n\"font/FiraSans-Bold.ttf\" is the default. \"font/FiraMono-Medium.ttf\" is also included in the asset pack.")
    );
    font_msg.font_size = 20.0;
    font_msg.font = "font/FiraMono-Medium.ttf".to_string();
    font_msg.translation.y = 0.0;

    let msg = state.repo.add_one(Text::new("msg", "Changing the text's translation, rotation, and scale is fast,\n so feel free to do that a lot."));
    msg.font_size = 24.0;
    msg.translation.y = -100.0;

    let translation = state.repo.add_one(Text::new("translation", "Translation"));
    translation.font_size = 36.0;
    translation.translation = Vec2::new(-400.0, -230.0);

    let rotation = state.repo.add_one(Text::new("rotation", "Rotation"));
    rotation.font_size = 36.0;
    rotation.translation = Vec2::new(0.0, -230.0);

    let scale = state.repo.add_one(Text::new("scale", "Scale"));
    scale.font_size = 36.0;
    scale.translation = Vec2::new(400.0, -230.0);
}

fn game_logic(engine: &mut Engine, state: &mut State<GameState>) {
    if state.main.timer.tick(engine.delta).just_finished() {
        let mut fps = state.repo.get_one_mut::<Text>("fps").unwrap();
        fps.value = format!("FPS: {:.1}", 1.0 / engine.delta_f32);
    }

    let texts = state
        .repo
        .get::<Text, 4>(["translation", "rotation", "scale", "zoom_msg"])
        .unwrap();

    texts[0].translation.x = 50.0 * (engine.time_since_startup_f64).sin() as f32 - 400.0;
    texts[0].translation.y = 50.0 * (engine.time_since_startup_f64).cos() as f32 - 230.0;

    texts[1].rotation -= 1.5 * engine.delta_f32;

    texts[2].scale = 1.5 + ((engine.time_since_startup_f64 * 0.5).cos() as f32) * -1.0;

    texts[3].font_size = 10.0 * (engine.time_since_startup_f64 * 0.5).cos() as f32 + 25.0;
}
