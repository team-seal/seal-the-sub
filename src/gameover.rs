use vek::*;
use specs::prelude::*;
use quicksilver::{
    geom::{Rectangle, Triangle, Vector, Transform},
    input::{Key, ButtonState},
    graphics::{Color, Background, Image, Font, FontStyle},
    sound::Sound,
    lifecycle::{Window, Event, Asset},
};
use rand::{thread_rng, prelude::*};
use crate::{
    State,
    Universals,
    world::{self, Pos, Ori, Vel, Body, Seafloor, Attr},
    game::Game,
    menu::Menu,
};

pub struct GameOver {
    time: f32,
    is_high_score: bool,

    background: Asset<Image>,
    submarine: Asset<Image>,
    seal: Asset<Image>,
    fishes: Vec<Asset<Image>>,
    bubbles: Vec<Asset<Image>>,
    dark: Asset<Image>,

    chomp: Asset<Sound>,

    font: Asset<Font>,
}

impl GameOver {
    pub fn new(is_high_score: bool) -> Self {
        let (globals, world) = world::create();
        Self {
            time: 0.0,
            is_high_score,
            background: Asset::new(Image::load("ocean.png")),
            submarine: Asset::new(Image::load("submarine.png")),
            seal: Asset::new(Image::load("seal.png")),
            fishes: vec![
                Asset::new(Image::load("fish0.png")),
                Asset::new(Image::load("fish1.png")),
                Asset::new(Image::load("fish2.png")),
            ],
            bubbles: vec![
                Asset::new(Image::load("bubble0.png")),
                Asset::new(Image::load("bubble1.png")),
            ],
            dark: Asset::new(Image::load("dark.png")),

            chomp: Asset::new(Sound::load("chomp.wav")),

            font: Asset::new(Font::load("font.ttf")),
        }
    }

    pub fn tick(&mut self, window: &mut Window, universals: &mut Universals) -> Option<State> {
        let time = self.time;

        // Handle input
        if window.keyboard()[Key::Space].is_down() && time > 0.5 {
            return Some(State::Menu(Menu::new()))
        }

        window.clear(Color::from_rgba(120, 200, 255, 1.0));

        let is_high_score = self.is_high_score;
        self.font.execute(|font| {
            let img = font.render("Game Over!", &FontStyle::new(64.0, Color::WHITE)).unwrap();
            window.draw_ex(
                &img.area(),
                Background::Img(&img),
                Transform::translate((120.0, 120.0)),
                10.0,
            );

            if is_high_score {
                let img = font.render("You got a high score!", &FontStyle::new(48.0, Color::from_rgba(50, 255, 150, 1.0))).unwrap();
                window.draw_ex(
                    &img.area(),
                    Background::Img(&img),
                    Transform::translate((120.0, 190.0)),
                    10.0,
                );
            }

            let img = font.render("Press SPACE to return to the menu", &FontStyle::new(48.0, Color::WHITE)).unwrap();
            window.draw_ex(
                &img.area(),
                Background::Img(&img),
                Transform::translate((120.0, 240.0)),
                10.0,
            );

            Ok(())
        });

        self.time = time + 1.0 / 60.0;

        None
    }
}

