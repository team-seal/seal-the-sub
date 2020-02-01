mod world;
mod game;

use vek::*;
use world::{Pos, Ori};
use specs::prelude::*;
use quicksilver::{
    geom::Vector,
    graphics::{Color, Graphics},
    lifecycle::{run, EventStream, Settings, Window},
};

fn main() {
    run(
        Settings {
            size: Vector::new(800.0, 600.0).into(),
            title: "Seal the Sub",
            ..Settings::default()
        },
        app,
    );
}

pub enum State {
    Game,
}

async fn app(window: Window, mut gfx: Graphics, mut events: EventStream) -> quicksilver::Result<()> {
    let mut state = State::Game;

    while let Some(new_state) = match state {
        State::Game => game::play(&window, &mut gfx, &mut events).await?,
    } {
        state = new_state;
    }

    Ok(())
}
