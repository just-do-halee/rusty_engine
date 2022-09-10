//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example car_shoot

use rand::prelude::*;
use rusty_engine::prelude::*;

struct GameState {
    marble_labels: Vec<String>,
    cars_left: i32,
    spawn_timer: Timer,
}

fn main() {
    let mut game = Game::new();

    // Initial game state
    let game_state = GameState {
        marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()],
        cars_left: 25,
        spawn_timer: Timer::from_seconds(0.0, false),
    };

    // Set the title of the window
    game.window_settings(WindowDescriptor {
        title: "Car Shoot".into(),
        ..Default::default()
    });

    game.logic.push(load).push(game_logic);
    game.run(game_state);
}

fn load(engine: &mut Engine, state: &mut State<GameState>) {
    // Start the music
    engine
        .audio_manager
        .play_music(MusicPreset::Classy8Bit, 0.1);

    //
    let player = state
        .repo
        .add_one(Sprite::new("player", SpritePreset::RacingBarrierRed));
    player.rotation = UP;
    player.scale = 0.5;
    player.translation.y = -325.0;
    player.layer = 10.0;

    let cars_left = state.repo.add_one(Sprite::new(
        "cars left",
        format!("Cars left: {}", state.main.cars_left),
    ));
    cars_left.translation = Vec2::new(540.0, -320.0);
}
fn game_logic(engine: &mut Engine, state: &mut State<GameState>) {
    // Handle marble gun movement
    let player = state.repo.get_one_mut::<Sprite>("player").unwrap();
    if let Some(location) = engine.mouse_state.location() {
        player.translation.x = location.x;
    }
    let player_x = player.translation.x;

    // Shoot marbles!
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(label) = state.main.marble_labels.pop() {
            let marble = state
                .repo
                .add_one(Sprite::new(label, SpritePreset::RollingBallBlue));
            marble.translation.x = player_x;
            marble.translation.y = -275.0;
            marble.layer = 5.0;
            marble.collision = true;
            engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);
        }
    }

    // Move marbles
    const MARBLE_SPEED: f32 = 600.0;

    let delta_f32 = engine.delta_f32;

    state
        .repo
        .filter_mut::<Sprite, _>(|sprite| sprite.label.starts_with("marble"))
        .for_each(|marble| marble.translation.y += MARBLE_SPEED * delta_f32);

    // Move cars across the screen
    const CAR_SPEED: f32 = 250.0;
    state
        .repo
        .filter_mut::<Sprite, _>(|sprite| sprite.label.starts_with("car"))
        .for_each(|car| car.translation.x += CAR_SPEED * delta_f32);

    // Clean up sprites that have gone off the screen
    state.repo.delete::<Sprite>(|sprite| {
        if sprite.translation.y > 400.0 || sprite.translation.x > 750.0 {
            if sprite.label.starts_with("marble") {
                state.main.marble_labels.push(sprite.label.to_string());
            }
            return true;
        }
        false
    });

    // Spawn cars
    if state.main.spawn_timer.tick(engine.delta).just_finished() {
        // Reset the timer to a new value
        state.main.spawn_timer = Timer::from_seconds(thread_rng().gen_range(0.1..1.25), false);
        // Get the new car
        if state.main.cars_left > 0 {
            state.main.cars_left -= 1;
            let text = state.repo.get_one_mut::<Text>("cars left").unwrap();
            text.value = format!("Cars left: {}", state.main.cars_left);
            let label = format!("car{}", state.main.cars_left);
            use SpritePreset::*;
            let car_choices = vec![
                RacingCarBlack,
                RacingCarBlue,
                RacingCarGreen,
                RacingCarRed,
                RacingCarYellow,
            ];
            let sprite_preset = *car_choices.iter().choose(&mut thread_rng()).unwrap();
            let car = state.repo.add_one(Sprite::new(label, sprite_preset));
            car.translation.x = -740.0;
            car.translation.y = thread_rng().gen_range(-100.0..325.0);
            car.collision = true;
        }
    }

    // Handle collisions
    for event in engine.collision_events.drain(..) {
        if event.state.is_end() {
            continue;
        }
        if !event.pair.one_starts_with("marble") {
            continue;
        }

        for label in event.pair {
            state.repo.remove::<Sprite>(&label);
            if label.starts_with("marble") {
                state.main.marble_labels.push(label);
            }
            engine.audio_manager.play_sfx(SfxPreset::Confirmation1, 0.2);
        }
    }
}
