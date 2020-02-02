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
    menu,
};

pub struct Menu {
    time: f32,

    background: Asset<Image>,
    submarine: Asset<Image>,
    seal: Asset<Image>,
    fishes: Vec<Asset<Image>>,
    bubbles: Vec<Asset<Image>>,
    dark: Asset<Image>,

    chomp: Asset<Sound>,

    font: Asset<Font>,
}

impl Menu {
    pub fn new() -> Self {
        let (globals, world) = world::create();
        Self {
            time: 0.0,
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
            return Some(State::Game(Game::new()))
        }

        window.clear(Color::from_rgba(120, 200, 255, 1.0));

        self.font.execute(|font| {
            let img = font.render("Seal the Sub", &FontStyle::new(64.0, Color::WHITE)).unwrap();
            window.draw_ex(
                &img.area(),
                Background::Img(&img),
                Transform::translate((120.0, 120.0)),
                10.0,
            );

            let img = font.render(&format!("High Score: {}", universals.high_score), &FontStyle::new(48.0, Color::WHITE)).unwrap();
            window.draw_ex(
                &img.area(),
                Background::Img(&img),
                Transform::translate((120.0, 240.0)),
                10.0,
            );

            let img = font.render(&format!("Total Score: {}", universals.total_score), &FontStyle::new(48.0, Color::WHITE)).unwrap();
            window.draw_ex(
                &img.area(),
                Background::Img(&img),
                Transform::translate((120.0, 300.0)),
                10.0,
            );

            let img = font.render("Press SPACE to play", &FontStyle::new(48.0, Color::WHITE)).unwrap();
            window.draw_ex(
                &img.area(),
                Background::Img(&img),
                Transform::translate((120.0, 380.0)),
                10.0,
            );

            Ok(())
        });

        self.time = time + 1.0 / 60.0;

        None
    }
}

