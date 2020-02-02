use vek::*;
use specs::prelude::*;
use quicksilver::{
    geom::{Rectangle, Triangle, Vector, Transform},
    input::Key,
    graphics::{Color, Background, Image, Font, FontStyle},
    sound::Sound,
    lifecycle::{Window, Event, Asset},
};
use rand::{thread_rng, prelude::*};
use crate::{
    State,
    Universals,
    world::{self, Pos, Ori, Vel, Body, Seafloor, Attr},
    gameover::GameOver,
};

#[derive(Copy, Clone, Default)]
pub struct Inputs {
    pub left: bool,
    pub right: bool,
    pub boost: bool,
}

pub struct Game {
    world: world::World,
    globals: world::Globals,
    inputs: Inputs,
    time: f32,

    background: Asset<Image>,
    submarine: Asset<Image>,
    seal: Asset<Image>,
    fishes: Vec<Asset<Image>>,
    bubbles: Vec<Asset<Image>>,
    tapes: Vec<Asset<Image>>,
    fuels: Vec<Asset<Image>>,
    dark: Asset<Image>,

    chomp: Asset<Sound>,
    hardsplash: Asset<Sound>,
    thud: Asset<Sound>,
    ding: Asset<Sound>,

    font: Asset<Font>,
}

impl Game {
    pub fn new() -> Self {
        let (globals, world) = world::create();
        Self {
            world,
            globals,
            inputs: Inputs::default(),
            time: 0.0,
            background: Asset::new(Image::load("ocean.png")),
            submarine: Asset::new(Image::load("submarine.png")),
            seal: Asset::new(Image::load("seal.png")),
            fishes: vec![
                Asset::new(Image::load("fish0.png")),
                Asset::new(Image::load("fish1.png")),
                Asset::new(Image::load("fish2.png")),
                Asset::new(Image::load("fish3.png")),
                Asset::new(Image::load("fish4.png")),
            ],
            bubbles: vec![
                Asset::new(Image::load("bubble0.png")),
                Asset::new(Image::load("bubble1.png")),
            ],
            tapes: vec![
                Asset::new(Image::load("tape0.png")),
                Asset::new(Image::load("tape1.png")),
                Asset::new(Image::load("roll.png")),
            ],
            fuels: vec![
                Asset::new(Image::load("fuel.png")),
            ],
            dark: Asset::new(Image::load("dark.png")),

            chomp: Asset::new(Sound::load("chomp.wav")),
            hardsplash: Asset::new(Sound::load("hardsplash.wav")),
            thud: Asset::new(Sound::load("thud.wav")),
            ding: Asset::new(Sound::load("ding.wav")),

            font: Asset::new(Font::load("font.ttf")),
        }
    }

    pub fn tick(&mut self, window: &mut Window, universals: &mut Universals) -> Option<State> {
        let time = self.time;

        // Handle input
        self.inputs.left = window.keyboard()[Key::Left].is_down();
        self.inputs.right = window.keyboard()[Key::Right].is_down();
        self.inputs.boost = window.keyboard()[Key::Up].is_down();

        // Tick world
        let tick_info = world::tick(&self.world, self.inputs, time, &self.globals);

        for event in tick_info.events.iter() {
            match event {
                world::Event::Eat => {
                    self.chomp.execute(|chomp| chomp.play());
                },
                world::Event::Splash(x) if (x - tick_info.view_centre.x).abs() < 400.0 => {
                    self.hardsplash.execute(|hardsplash| {hardsplash.set_volume(5.0); hardsplash.play() });
                },
                world::Event::GetFuel => {
                    self.ding.execute(|ding| ding.play());
                },
                world::Event::GetTape => {
                    self.ding.execute(|ding| ding.play());
                },
                _ => {},
            }
        }

        let world_trans = Transform::IDENTITY
            * Transform::translate((Vec2::new(window.screen_size().x, window.screen_size().y) * 0.5).into_tuple())
            * Transform::scale(Vec2::broadcast(tick_info.view_scale).into_tuple())
            * Transform::translate((-tick_info.view_centre).into_tuple());

        window.clear(Color::from_rgba(0xbe, 0xcd, 0xbb, 1.0));

        // Background
        self.background.execute(|background| {
            for i in 0..20 {
                let w = 512.0;
                let x = (((tick_info.view_centre.x - 1000.0) / w).floor() + i as f32) * w;
                window.draw_ex(
                    &Rectangle::new((x, -390.0), (w, 2048.0)),
                    Background::Img(&background),
                    world_trans,
                    -2.0,
                );
            }

            Ok(())
        });

        // Sea floor
        let seafloor = self.world.read_resource::<Seafloor>();
        for i in 0..100 {
            let incr = 20.0;
            let x = (tick_info.view_centre.x - 1000.0) + i as f32 * incr;
            window.draw_ex(
                &Triangle::new(
                    (x, seafloor.sample(x)),
                    (x + incr, seafloor.sample(x + incr)),
                    (0.0, 1000000.0)
                ),
                Color::from_rgba(250, 200, 150, 255.0),
                world_trans,
                -1.0,
            );
        }

        // Entities
        for (pos, ori, vel, body) in (
            &self.world.read_storage::<Pos>(),
            &self.world.read_storage::<Ori>(),
            &self.world.read_storage::<Vel>(),
            &self.world.read_storage::<Body>(),
        ).join() {
            match body {
                Body::Seal => {
                    self.seal.execute(|seal| {
                        window.draw_ex(
                            &Rectangle::new((-48.0, -32.0), (64.0, 64.0)),
                            Background::Img(&seal),
                                world_trans
                                * Transform::translate(pos.0.into_tuple())
                                * Transform::rotate(ori.0 * 180.0 / 3.1415)
                                * Transform::scale(if vel.0.x > 0.0 { (1.0, 1.0) } else { (1.0, -1.0) }),
                            0.0,
                        );

                        Ok(())
                    });
                },
                Body::Fish(i) => {
                    let img_idx = i % self.fishes.len();
                    self.fishes[img_idx].execute(|fish| {
                        window.draw_ex(
                            &Rectangle::new((-24.0, -16.0), (32.0, 32.0)),
                            Background::Img(&fish),
                                world_trans
                                * Transform::translate(pos.0.into_tuple())
                                * Transform::rotate(ori.0 * 180.0 / 3.1415)
                                * Transform::scale(if vel.0.x > 0.0 { (1.0, 1.0) } else { (1.0, -1.0) }),
                            0.0,
                        );

                        Ok(())
                    });
                },
                Body::Bubble(i) => {
                    let img_idx = i % self.bubbles.len();
                    self.bubbles[img_idx].execute(|bubble| {
                        window.draw_ex(
                            &Rectangle::new((-24.0, -24.0), (48.0, 48.0)),
                            Background::Img(&bubble),
                                world_trans
                                * Transform::translate(pos.0.into_tuple())
                                * Transform::rotate(ori.0 * 180.0 / 3.1415)
                                * Transform::scale(if vel.0.x > 0.0 { (1.0, 1.0) } else { (1.0, -1.0) }),
                            0.5,
                        );

                        Ok(())
                    });
                },
                Body::Fuel(i) => {
                    let img_idx = i % self.fuels.len();
                    self.fuels[img_idx].execute(|bubble| {
                        window.draw_ex(
                            &Rectangle::new((-20.0, -20.0), (40.0, 40.0)),
                            Background::Img(&bubble),
                                world_trans
                                * Transform::translate(pos.0.into_tuple())
                                * Transform::rotate(ori.0 * 180.0 / 3.1415)
                                * Transform::scale(if vel.0.x > 0.0 { (1.0, 1.0) } else { (1.0, -1.0) }),
                            0.5,
                        );

                        Ok(())
                    });
                },
                Body::Tape(i) => {
                    let img_idx = i % self.tapes.len();
                    self.tapes[img_idx].execute(|bubble| {
                        window.draw_ex(
                            &Rectangle::new((-20.0, -20.0), (40.0, 40.0)),
                            Background::Img(&bubble),
                                world_trans
                                * Transform::translate(pos.0.into_tuple())
                                * Transform::rotate(ori.0 * 180.0 / 3.1415)
                                * Transform::scale(if vel.0.x > 0.0 { (1.0, 1.0) } else { (1.0, -1.0) }),
                            0.5,
                        );

                        Ok(())
                    });
                },
                Body::Submarine => {
                    self.submarine.execute(|submarine| {
                        window.draw_ex(
                            &Rectangle::new((-512.0, -512.0), (1024.0, 1024.0)),
                            Background::Img(&submarine),
                            world_trans
                                * Transform::rotate((time * 1.0).sin() * 3.0)
                                * Transform::translate((
                                    pos.0 + Vec2::new(
                                        thread_rng().gen_range(-1.0, 1.0),
                                        thread_rng().gen_range(-1.0, 1.0) + (time * 2.0).sin() * 16.0,
                                    )
                                ).into_tuple()),
                            -0.5,
                        );

                        Ok(())
                    });
                },
            }
        }

        // Sea
        window.draw_ex(
            &Rectangle::new(Vec2::new(-500000.0, 0.0).into_tuple(), Vec2::broadcast(1000000.0).into_tuple()),
            Color::from_rgba(0, 150, 250, 0.3),
            world_trans,
            1.0,
        );

        // Darkness

        self.dark.execute(|dark| {
            window.draw_ex(
                &Rectangle::new((0.0, 0.0), (window.screen_size().x, window.screen_size().y)),
                Background::Blended(&dark, Color::from_rgba(255, 255, 255, (tick_info.view_centre.y / 1500.0).max(0.0).min(1.0).powf(2.0))),
                Transform::IDENTITY,
                5.0,
            );

            Ok(())
        });

        // UI
        let attr = self.world.read_resource::<Attr>();

        let mut font = &mut self.font;
        let mut draw_bar = |msg, val, y: f32| {
            font.execute(|font| {
                let img = font.render(msg, &FontStyle::new(32.0, Color::WHITE)).unwrap();
                window.draw_ex(
                    &img.area(),
                    Background::Img(&img),
                    Transform::translate((160.0, y + 2.0)),
                    10.0,
                );

                window.draw_ex(&Rectangle::new((22.0, y + 6.0), (128.0, 24.0)), Color::from_rgba(100, 100, 100, 1.0), Transform::IDENTITY, 10.0);
                window.draw_ex(&Rectangle::new((22.0, y + 6.0), (128.0 * val, 24.0)), Color::from_rgba(100, 255, 50, 1.0), Transform::IDENTITY, 10.0);

                Ok(())
            });
        };

        draw_bar("Stamina", attr.stamina, 16.0);
        draw_bar("Hull", attr.hull, 48.0);
        draw_bar("Fuel", attr.fuel, 80.0);

        font.execute(|font| {
            let img = font.render(&format!("Score: {}", attr.score.floor() as u32), &FontStyle::new(48.0, Color::WHITE)).unwrap();
            window.draw_ex(
                &img.area(),
                Background::Img(&img),
                Transform::translate((window.screen_size().x - img.area().width() - 22.0, 22.0)),
                10.0,
            );

            Ok(())
        });

        self.time = time + 1.0 / 60.0;

        if attr.lost {
            universals.high_score = universals.high_score.max(attr.score.floor() as u32);
            universals.total_score += attr.score.floor() as u32;
            return Some(State::GameOver(GameOver::new(attr.score.floor() as u32 == universals.high_score)))
        }

        None
    }
}
