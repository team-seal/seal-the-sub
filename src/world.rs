use vek::*;
use specs::{
    prelude::*,
    Component,
};
use crate::Inputs;

pub struct Globals {
    pub player: Entity,
}

pub fn create() -> (Globals, specs::World) {
    let mut world = specs::World::new();
    world.register::<Pos>();
    world.register::<Vel>();
    world.register::<Ori>();
    world.register::<Rot>();
    world.register::<Player>();

    let player = world
        .create_entity()
        .with(Player)
        .with(Pos(Vec2::broadcast(64.0)))
        .with(Vel(Vec2::unit_x()))
        .with(Ori(0.0))
        .with(Rot(0.0))
        .build();

    world
        .create_entity()
        .with(Pos(Vec2::new(0.0, 0.0)))
        .build();
    world
        .create_entity()
        .with(Pos(Vec2::new(132.0, 132.0)))
        .build();

    (Globals {
        player,
    }, world)
}

#[derive(Default)]
pub struct TickInfo {
    pub view_centre: Vec2<f32>,
    pub view_scale: f32,
}

const TURN_RATE_WATER: f32 = 0.005;
const TURN_RATE_AIR: f32 = 0.001;
const GRAVITY: f32 = 0.1;

pub fn tick(world: &specs::World, inputs: Inputs) -> TickInfo {
    let mut tick_info = TickInfo::default();

    for (pos, vel, ori, rot, player) in (
        &mut world.write_storage::<Pos>(),
        &mut world.write_storage::<Vel>(),
        &mut world.write_storage::<Ori>(),
        &mut world.write_storage::<Rot>(),
        world.read_storage::<Player>().maybe(),
    ).join() {
        let underwater = pos.0.y > 0.0;

        // User input
        if player.is_some() {
            if underwater {
                if inputs.left { rot.0 -= TURN_RATE_WATER; }
                if inputs.right { rot.0 += TURN_RATE_WATER; }

                // Swimming
                vel.0 += Vec2::new(
                    ori.0.cos(),
                    ori.0.sin(),
                ) * 0.25;

                if inputs.boost { vel.0 *= 1.025; }
            } else {
                if inputs.left { rot.0 -= TURN_RATE_AIR; }
                if inputs.right { rot.0 += TURN_RATE_AIR; }
            }
        }
        // Drag
        if underwater {
            // Drag
            vel.0 *= 0.95;
            rot.0 *= 0.95;
        } else {
            vel.0 *= 0.99;
            rot.0 *= 0.99;
        }

        if !underwater {
            vel.0.y += GRAVITY;
        }

        pos.0 += vel.0;
        ori.0 += rot.0;

        // Tick info
        if player.is_some() {
            tick_info.view_centre = pos.0;// + vel.0 * 24.0;
            tick_info.view_scale = 1.0;// - vel.0.magnitude() * 0.12;
        }
    }

    tick_info
}

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

pub struct Pos(pub Vec2<f32>);

impl Component for Pos {
    type Storage = VecStorage<Self>;
}

pub struct Vel(pub Vec2<f32>);

impl Component for Vel {
    type Storage = VecStorage<Self>;
}

pub struct Ori(pub f32);

impl Component for Ori {
    type Storage = VecStorage<Self>;
}

pub struct Rot(pub f32);

impl Component for Rot {
    type Storage = VecStorage<Self>;
}
