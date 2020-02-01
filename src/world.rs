use vek::*;
use rand::{prelude::*, thread_rng};
use specs::{
    prelude::*,
    Component,
};
use crate::game::Inputs;

pub use specs::World;

pub struct Globals {
    pub player: Entity,
}

pub fn create() -> (Globals, specs::World) {
    let mut world = specs::World::new();

    world.register::<Pos>();
    world.register::<Vel>();
    world.register::<Ori>();
    world.register::<Rot>();
    world.register::<Agent>();

    world.insert(Seafloor::sine());

    let player = world
        .create_entity()
        .with(Pos(Vec2::broadcast(64.0)))
        .with(Vel(Vec2::unit_x()))
        .with(Ori(0.0))
        .with(Rot(0.0))
        .with(Agent::Player)
        .build();

    for _ in 0..100 {
        world
            .create_entity()
            .with(Pos(Vec2::new(
                thread_rng().gen_range(-1000.0, 1000.0),
                thread_rng().gen_range(0.0, 1000.0),
            )))
            .with(Vel(Vec2::zero()))
            .with(Ori(0.0))
            .with(Rot(0.0))
            .with(Agent::Fish)
            .build();
    }

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

    let underwater = |pos: &Pos| pos.0.y > 0.0;

    let seafloor = world.read_resource::<Seafloor>();

    // Physics
    for (pos, vel, ori, rot) in (
        &mut world.write_storage::<Pos>(),
        &mut world.write_storage::<Vel>(),
        &mut world.write_storage::<Ori>(),
        &mut world.write_storage::<Rot>(),
    ).join() {
        // Drag
        if underwater(pos) {
            // Drag
            vel.0 *= 0.95;
            rot.0 *= 0.95;
        } else {
            vel.0 *= 0.99;
            rot.0 *= 0.99;
        }

        if !underwater(pos) {
            vel.0.y += GRAVITY;
        }

        pos.0 += vel.0;
        ori.0 += rot.0;

        // Collision with seafloor
        pos.0.y = pos.0.y.min(seafloor.sample(pos.0.x));
    }

    // Control
    for (pos, vel, ori, rot, agent) in (
        &mut world.write_storage::<Pos>(),
        &mut world.write_storage::<Vel>(),
        &mut world.write_storage::<Ori>(),
        &mut world.write_storage::<Rot>(),
        &mut world.write_storage::<Agent>(),
    ).join() {
        match agent {
            Agent::Player => {
                // User input
                if underwater(pos) {
                    if inputs.left { rot.0 -= TURN_RATE_WATER; }
                    if inputs.right { rot.0 += TURN_RATE_WATER; }

                    if inputs.boost { vel.0 *= 1.025; }
                } else {
                    if inputs.left { rot.0 -= TURN_RATE_AIR; }
                    if inputs.right { rot.0 += TURN_RATE_AIR; }
                }

                // Tick info
                tick_info.view_centre = pos.0;
                tick_info.view_scale = 1.0;
            },
            Agent::Fish => rot.0 += thread_rng().gen_range(-1.0, 1.0) * TURN_RATE_WATER,
        }

        // Swimming
        if underwater(pos) {
            vel.0 += Vec2::new(
                ori.0.cos(),
                ori.0.sin(),
            ) * 0.25;
        }
    }

    tick_info
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

pub enum Agent {
    Player,
    Fish,
}

impl Component for Agent {
    type Storage = VecStorage<Self>;
}

pub struct Seafloor {
    heights: Vec<f32>,
}

const SEAFLOOR_HEIGHT: f32 = 1000.0;
const SEAFLOOR_STRIDE: f32 = 10.0;
const SEAFLOOR_OFFSET: i32 = 500;

impl Seafloor {
    pub fn sine() -> Self {
        Self {
            heights: (-SEAFLOOR_OFFSET..500)
                .map(|i| i as f32 * SEAFLOOR_STRIDE)
                .map(|x| SEAFLOOR_HEIGHT + (x * 0.01).sin() * 30.0)
                .collect(),
        }
    }

    pub fn sample(&self, x: f32) -> f32 {
        let xx = x + SEAFLOOR_OFFSET as f32 * SEAFLOOR_STRIDE;
        let fract = (xx / SEAFLOOR_STRIDE).fract();
        let idx = (xx / SEAFLOOR_STRIDE) as usize;

        let a = self.heights.get(idx).copied().unwrap_or(0.0);
        let b = self.heights.get(idx.saturating_add(1)).copied().unwrap_or(0.0);

        a + (b - a) * fract
    }
}
