mod world;
mod game;
mod menu;
mod gameover;

use vek::*;
use world::{Pos, Ori};
use specs::prelude::*;
use quicksilver::{
    geom::Vector,
    saving::{save, load},
    lifecycle::{run, Settings, Window},
};
use serde::{Serialize, Deserialize};
use crate::{
    game::Game,
    menu::Menu,
    gameover::GameOver,
};

pub enum State {
    Game(Game),
    Menu(Menu),
    GameOver(GameOver),
}

struct Engine {
    state: State,
    universals: Universals,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Universals {
    high_score: u32,
    total_score: u32,
}

impl quicksilver::lifecycle::State for Engine {
    fn new() -> quicksilver::Result<Self> {
        Ok(Self {
            state: State::Menu(Menu::new()),
            universals: load("seal-the-sub", "foo").unwrap_or(Universals {
                high_score: 0,
                total_score: 0,
            })
        })
    }

    fn draw(&mut self, window: &mut Window) -> quicksilver::Result<()> {
        if let Some(new_state) = match &mut self.state {
            State::Game(game) => game.tick(window, &mut self.universals),
            State::Menu(menu) => menu.tick(window, &mut self.universals),
            State::GameOver(gameover) => gameover.tick(window, &mut self.universals),
        } {
            self.state = new_state;
            save("seal-the-sub", "foo", &self.universals);
        }

        Ok(())
    }
}

fn main() {
    run::<Engine>(
        "Seal the Sub",
        Vector::new(1000.0, 500.0),
        Settings::default(),
    );
}
