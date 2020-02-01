use vek::*;
use specs::prelude::*;
use quicksilver::{
    geom::{Rectangle, Triangle, Vector, Transform},
    input::Key,
    graphics::{Color, Background, Image},
    lifecycle::{Window, Event, Asset},
};
use rand::{thread_rng, prelude::*};
use crate::{
    State,
    world::{self, Pos, Ori, Vel, Body, Seafloor},
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
        }
    }

    pub fn tick(&mut self, window: &mut Window) {
        let time = self.time;

        // Handle input
        self.inputs.left = window.keyboard()[Key::Left].is_down();
        self.inputs.right = window.keyboard()[Key::Right].is_down();
        self.inputs.boost = window.keyboard()[Key::Up].is_down();

        // Tick world
        let tick_info = world::tick(&self.world, self.inputs, time);

        let world_trans = Transform::IDENTITY
            * Transform::translate((Vec2::new(window.screen_size().x, window.screen_size().y) * 0.5).into_tuple())
            * Transform::scale(Vec2::broadcast(tick_info.view_scale).into_tuple())
            * Transform::translate((-tick_info.view_centre).into_tuple());

        window.clear(Color::WHITE);

        // Background
        self.background.execute(|background| {
            for i in 0..20 {
                let w = 512.0;
                let x = (((tick_info.view_centre.x - 1000.0) / w).floor() + i as f32) * w;
                window.draw_ex(
                    &Rectangle::new((x, -320.0), (w, 2048.0)),
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
                Body::Seal | Body::Fish => {
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

        self.time = time + 1.0 / 60.0;
    }
}
