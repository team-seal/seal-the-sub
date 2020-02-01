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
    pub submarine: Entity,
}

pub struct Attr {
    pub stamina: f32,
}

impl Attr {
    pub fn new() -> Self {
        Self {
            stamina: 1.0,
        }
    }

    pub fn tick(&mut self) {
        self.stamina -= 0.001;
    }
}

pub enum Event {
    Eat,
}

pub fn create() -> (Globals, specs::World) {
    let mut world = specs::World::new();

    world.register::<Pos>();
    world.register::<Vel>();
    world.register::<Ori>();
    world.register::<Rot>();
    world.register::<Agent>();
    world.register::<Body>();
    world.register::<Item>();
    world.register::<Respawn>();

    world.insert(Seafloor::sine());
    world.insert(Attr::new());

    let player = world
        .create_entity()
        .with(Pos(Vec2::broadcast(0.0)))
        .with(Vel(Vec2::unit_x()))
        .with(Ori(0.0))
        .with(Rot(0.0))
        .with(Agent::Player)
        .with(Body::Seal)
        .build();

    let submarine = world
        .create_entity()
        .with(Pos(Vec2::new(0.0, 512.0)))
        .with(Vel(Vec2::unit_x()))
        .with(Ori(0.0))
        .with(Rot(0.0))
        .with(Body::Submarine)
        .build();

    for i in 0..30 {
        world
            .create_entity()
            .with(Pos(Vec2::new(
                thread_rng().gen_range(-5000.0, 5000.0),
                thread_rng().gen_range(0.0, 1500.0),
            )))
            .with(Vel(Vec2::zero()))
            .with(Ori(0.0))
            .with(Rot(0.0))
            .with(Agent::Fish)
            .with(Body::Fish(i))
            .with(Item::Fish)
            .build();
    }

    for i in 0..150 {
        world
            .create_entity()
            .with(Pos(Vec2::new(
                thread_rng().gen_range(-5000.0, 5000.0),
                thread_rng().gen_range(0.0, 1500.0),
            )))
            .with(Vel(Vec2::zero()))
            .with(Ori(0.0))
            .with(Rot(0.0))
            .with(Agent::Bubble)
            .with(Body::Bubble(i))
            .build();
    }

    (Globals {
        player,
        submarine,
    }, world)
}

#[derive(Default)]
pub struct TickInfo {
    pub view_centre: Vec2<f32>,
    pub view_scale: f32,
    pub events: Vec<Event>,
}

const TURN_RATE_WATER: f32 = 0.0065;
const TURN_RATE_AIR: f32 = 0.001;
const GRAVITY: f32 = 0.1;

pub fn tick(world: &specs::World, inputs: Inputs, time: f32) -> TickInfo {
    let mut tick_info = TickInfo::default();

    let underwater = |pos: &Pos| pos.0.y > 0.0;

    let seafloor = world.read_resource::<Seafloor>();
    let mut attr = world.write_resource::<Attr>();
    attr.tick();

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
            rot.0 *= 0.90;

            let ori_dir = Vec2::new(
                ori.0.cos(),
                ori.0.sin(),
            );
            vel.0 *= f32::lerp(ori_dir.dot(vel.0.try_normalized().unwrap_or(Vec2::zero())).max((-ori_dir).dot(vel.0.try_normalized().unwrap_or(Vec2::zero()))), 1.0, 0.9);
        } else {
            rot.0 *= 0.98;
        }

        if !underwater(pos) {
            vel.0.y += GRAVITY;
        }

        pos.0 += vel.0;
        ori.0 += rot.0;

        // Collision with seafloor
        let sample = seafloor.sample(pos.0.x);
        if pos.0.y > sample {
            pos.0.y = sample;
            vel.0 *= 0.93;
        }

    }

    // Find all positions and velocities
    let mut all_pos_vel = (
        &world.read_storage::<Pos>(),
        &world.read_storage::<Vel>(),
        &world.read_storage::<Item>(),
    )
        .join()
        .map(|(p, v, _)| (p.0, v.0))
        .collect::<Vec<_>>();

    // Control
    for (entity, pos, vel, ori, rot, agent) in (
        &world.entities(),
        &world.read_storage::<Pos>(),
        &mut world.write_storage::<Vel>(),
        &mut world.write_storage::<Ori>(),
        &mut world.write_storage::<Rot>(),
        &mut world.write_storage::<Agent>(),
    ).join() {
        let physics = match agent {
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
                tick_info.view_scale = 1.0;// + 1.0 / (1.0 + vel.0.magnitude());

                true
            },
            Agent::Fish => {
                all_pos_vel.sort_by_key(|(other_pos, _)| pos.0.distance_squared(*other_pos) as i32);

                let (total_vel, n) = all_pos_vel.iter().take(4).fold((Vec2::zero(), 0.0), |(tv, n), (_, v)| (tv + v, n + 1.0));
                let avg_vel = total_vel / n;

                let (total_pos, n) = all_pos_vel.iter().take(4).fold((Vec2::zero(), 0.0), |(tp, n), (p, _)| (tp + p, n + 1.0));
                let avg_pos = total_pos / n;

                let shy_dir = all_pos_vel.iter().take(4).fold(Vec2::zero(), |a, (p, _)| a + (pos.0 - p).try_normalized().unwrap_or(Vec2::zero()) * (100.0 - (pos.0 - p).magnitude()).max(0.0));
                let dir = Vec2::lerp((avg_pos - pos.0).normalized(), avg_vel, 0.5) + shy_dir;
                let dir = (dir.map(|e| e + thread_rng().gen_range(-0.02, 0.02)) - pos.0 * Vec2::new(1.0, 2.0) * 0.0005).normalized();

                let dir = Lerp::lerp(Vec2::new(ori.0.cos(), ori.0.sin()), dir, 0.1).try_normalized().unwrap_or(Vec2::zero());
                ori.0 = dir.y.atan2(dir.x);

                true
            },
            Agent::Bubble => {
                vel.0.x = (time + pos.0.x * 0.01).sin();
                vel.0.y = -1.0;

                if !underwater(pos) {
                    world.write_storage().insert(entity, Respawn);
                }

                false
            },
        };

        // Swimming
        if underwater(pos) && physics {
            vel.0 += Vec2::new(
                ori.0.cos(),
                ori.0.sin(),
            ) * 0.3;

            rot.0 += (time * 10.0).sin() * vel.0.magnitude().sqrt() * 0.001;
        }
    }

    // Collision
    for (pos, agent, body) in (
        &world.read_storage::<Pos>(),
        &world.read_storage::<Agent>(),
        &world.read_storage::<Body>(),
    ).join() {
        if let Agent::Player = agent {
            for (other_entity, other_pos, item, _) in (
                &world.entities(),
                &world.read_storage::<Pos>(),
                &world.read_storage::<Item>(),
                &world.read_storage::<Body>(),
            )
                .join()
                .filter(|(_, op, _, other_body)| op.0.distance_squared(pos.0) < (body.radius() + other_body.radius()).powf(2.0))
            {
                match item {
                    Item::Fish => {
                        world.write_storage().insert(other_entity, Respawn);
                        tick_info.events.push(Event::Eat);
                    },
                }
            }
        }
    }

    // Respawn things
    for (pos, _) in (
        &mut world.write_storage::<Pos>(),
        &world.read_storage::<Respawn>(),
    ).join() {
        pos.0 = Vec2::new(
            thread_rng().gen_range(-5000.0, 5000.0),
            thread_rng().gen_range(-300.0, 1500.0),
        );
    }
    world.write_storage::<Respawn>().clear();

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

pub enum Item {
    Fish,
}

impl Component for Item {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Respawn;

impl Component for Respawn {
    type Storage = NullStorage<Self>;
}

pub enum Body {
    Seal,
    Fish(usize),
    Submarine,
    Bubble(usize),
}

impl Component for Body {
    type Storage = VecStorage<Self>;
}

impl Body {
    pub fn radius(&self) -> f32 {
        match self {
            Body::Seal => 20.0,
            Body::Fish(_) => 12.0,
            Body::Submarine => 800.0,
            Body::Bubble(_) => 12.0,
        }
    }
}

pub enum Agent {
    Player,
    Fish,
    Bubble,
}

impl Component for Agent {
    type Storage = VecStorage<Self>;
}

pub struct Seafloor {
    heights: Vec<f32>,
}

const SEAFLOOR_HEIGHT: f32 = 1500.0;
const SEAFLOOR_STRIDE: f32 = 10.0;
const SEAFLOOR_OFFSET: i32 = 500;

impl Seafloor {
    pub fn sine() -> Self {
        Self {
            heights: (-SEAFLOOR_OFFSET..500)
                .map(|i| i as f32 * SEAFLOOR_STRIDE)
                .map(|x| SEAFLOOR_HEIGHT + (x * 0.01).sin() * 30.0 + (x * 0.002).sin() * 120.0)
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

    pub fn normal_at(&self, x: f32) -> Vec2<f32> {
        Vec2::new(
            1.0,
            1.0 / (self.sample(x - 0.5) - self.sample(x + 0.5))
        ).try_normalized().unwrap_or(-Vec2::unit_y())
    }
}
