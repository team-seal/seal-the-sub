mod world;
mod game;

use vek::*;
use world::{Pos, Ori};
use specs::prelude::*;
use quicksilver::{
    geom::Vector,
    lifecycle::{run, Settings, Window},
};
use crate::game::Game;

pub enum State {
    Game(Game),
}

struct Engine {
    state: State,
}

impl quicksilver::lifecycle::State for Engine {
    fn new() -> quicksilver::Result<Self> {
        Ok(Self {
            state: State::Game(Game::new()),
        })
    }

    fn draw(&mut self, window: &mut Window) -> quicksilver::Result<()> {
        match &mut self.state {
            State::Game(game) => game.tick(window),
        }

        Ok(())
    }
}

fn main() {
    run::<Engine>(
        "Seal the Sub",
        Vector::new(800.0, 600.0),
        Settings::default(),
    );
}
