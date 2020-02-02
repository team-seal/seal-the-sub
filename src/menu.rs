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
    music: Asset<Sound>,

    font: Asset<Font>,
    music_playing: bool,
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
            music: Asset::new(Sound::load("music.ogg")),

            font: Asset::new(Font::load("font.ttf")),
            music_playing: false,
        }
    }

    pub fn tick(&mut self, window: &mut Window, universals: &mut Universals) -> Option<State> {
        let time = self.time;

        let mut music_playing = &mut self.music_playing;
        if !*music_playing {
            self.music.execute(|music| {
                *music_playing = true;
                music.set_volume(0.25);
                music.play()
            }).unwrap();
        }

        // Handle input
        if window.keyboard()[Key::Space].is_down() && time > 0.5 {
            return Some(State::Game(Game::new()))
        }

        window.clear(Color::from_rgba(120, 200, 255, 1.0));

        self.background.execute(|background| {
                        window.draw_ex(
                            &Rectangle::new((0.0, 0.0), (1000.0, 500.0)),
                            Background::Img(&background),
                            Transform::IDENTITY,
                            -5.5,
                        );

                        Ok(())
                    });

        self.submarine.execute(|submarine| {
                        window.draw_ex(
                            &Rectangle::new((-180.0, -180.0), (360.0, 360.0)),
                            Background::Img(&submarine),
                            Transform::rotate((time * 1.0).sin() * 3.0)
                            * Transform::translate((700.0, 250.0 + (time * 2.0).sin() * 8.0)),
                            -0.5,
                        );

                        Ok(())
                    });

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

