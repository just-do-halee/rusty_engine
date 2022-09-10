//! To run this code, clone the rusty_engine repository and run the command:
//!
//!     cargo run --release --example state

use std::f32::consts::TAU;

use rusty_engine::prelude::*;

// You can name your game state struct whatever you want, and it can contain many different types as
// its fields
struct MyCustomGameStateStuff {
    // Use a timer to tell when to start/stop turning
    change_timer: Timer,
    // Usa a boolean track of whether we are currently turning or not
    turning: bool,
}

fn main() {
    let mut game = Game::new();

    game.logic //
        .push(load)
        .push(logic);

    game.run(MyCustomGameStateStuff {
        change_timer: Timer::from_seconds(1.0, true),
        turning: false,
    });
}

fn load(_: &mut Engine, state: &mut State<MyCustomGameStateStuff>) {
    let _ = state
        .repo
        .add_one(Sprite::new("Race Car", SpritePreset::RacingCarGreen));
}

fn logic(engine: &mut Engine, state: &mut State<MyCustomGameStateStuff>) {
    // Get mutable references to the variables in the game state that we care about
    let race_car = state.repo.get_one_mut::<Sprite>("Race Car").unwrap();

    // If we aren't turning, then tick the timer until it's time to start turning again
    if !state.main.turning && state.main.change_timer.tick(engine.delta).just_finished() {
        state.main.turning = true;
    }

    // Rotate the player
    if state.main.turning {
        race_car.rotation += engine.delta_f32 * 3.0;
        // If the player rotated all the way around, reset direction, stop turning
        // TAU == (2 * PI), which is exactly one rotation in radians
        if race_car.rotation > TAU {
            race_car.rotation = 0.0;
            state.main.turning = false;
        }
    }
}
