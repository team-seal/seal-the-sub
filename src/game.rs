use vek::*;
use specs::prelude::*;
use quicksilver::{
    geom::{Rectangle, Vector, Transform},
    graphics::{Color, Graphics},
    lifecycle::{EventStream, Window, Event, Key},
};
use crate::{
    State,
    world::{self, Pos, Ori},
};

#[derive(Copy, Clone, Default)]
pub struct Inputs {
    pub left: bool,
    pub right: bool,
    pub boost: bool,
}

pub async fn play(window: &Window, gfx: &mut Graphics, events: &mut EventStream) -> quicksilver::Result<Option<State>> {
    let (mut globals, world) = world::create();
    let mut inputs = Inputs::default();

    loop {
        // Handle events
        while let Some(event) = events.next_event().await {
            match event {
                Event::KeyboardInput(event) => match event.key() {
                    Key::Left => inputs.left = event.is_down(),
                    Key::Right => inputs.right = event.is_down(),
                    Key::Up => inputs.boost = event.is_down(),
                    _ => {},
                },
                _ => {},
            }
        }

        // Tick world
        let tick_info = world::tick(&world, inputs);

        let world_trans = Transform::IDENTITY
            .then(Transform::translate((- tick_info.view_centre).into_tuple()))
            .then(Transform::scale(Vec2::broadcast(tick_info.view_scale).into_tuple()))
            .then(Transform::translate((Vec2::new(window.size().x, window.size().y) * 0.5).into_tuple()));

        gfx.clear(Color::WHITE);

        for (pos, ori) in (
            &world.read_storage::<Pos>(),
            &world.read_storage::<Ori>(),
        ).join() {
            let rect = Rectangle::new(Vec2::new(-42.0, -12.0).into_tuple(), Vec2::new(64.0, 24.0).into_tuple());
            gfx.set_transform(
                Transform::rotate(ori.0 * 180.0 / 3.1415)
                    .then(Transform::translate(pos.0.into_tuple()))
                    .then(world_trans)
            );
            gfx.fill_rect(&rect, Color::RED);
        }

        gfx.set_transform(world_trans);
        gfx.fill_rect(
            &Rectangle::new(Vec2::new(-500000.0, 0.0).into_tuple(), Vec2::broadcast(1000000.0).into_tuple()),
            Color::from_rgba(0, 150, 250, 0.3),
        );

        gfx.present(&window)?;
    }
}
