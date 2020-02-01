use vek::*;
use specs::prelude::*;
use quicksilver::{
    geom::{Rectangle, Vector, Transform},
    input::Key,
    graphics::Color,
    lifecycle::{Window, Event},
};
use crate::{
    State,
    world::{self, Pos, Ori, Seafloor},
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
}

impl Game {
    pub fn new() -> Self {
        let (globals, world) = world::create();
        Self {
            world,
            globals,
            inputs: Inputs::default(),
        }
    }

    pub fn tick(&mut self, window: &mut Window) {
        // Handle input
        self.inputs.left = window.keyboard()[Key::Left].is_down();
        self.inputs.right = window.keyboard()[Key::Right].is_down();
        self.inputs.boost = window.keyboard()[Key::Up].is_down();

        // Tick world
        let tick_info = world::tick(&self.world, self.inputs);

        let world_trans = Transform::IDENTITY
            * Transform::translate((Vec2::new(window.screen_size().x, window.screen_size().y) * 0.5).into_tuple())
            * Transform::scale(Vec2::broadcast(tick_info.view_scale).into_tuple())
            * Transform::translate((-tick_info.view_centre).into_tuple());

        window.clear(Color::WHITE);

        // Sea floor
        let seafloor = self.world.read_resource::<Seafloor>();

        // Entities
        for (pos, ori) in (
            &self.world.read_storage::<Pos>(),
            &self.world.read_storage::<Ori>(),
        ).join() {
            let rect = Rectangle::new(Vec2::new(-42.0, -12.0).into_tuple(), Vec2::new(64.0, 24.0).into_tuple());
            window.draw_ex(
                &rect,
                Color::RED,
                world_trans
                    * Transform::translate(pos.0.into_tuple())
                    * Transform::rotate(ori.0 * 180.0 / 3.1415),
                0.0,
            );
        }

        // Sea
        window.draw_ex(
            &Rectangle::new(Vec2::new(-500000.0, 0.0).into_tuple(), Vec2::broadcast(1000000.0).into_tuple()),
            Color::from_rgba(0, 150, 250, 0.3),
            world_trans,
            0.0,
        );
    }
}
