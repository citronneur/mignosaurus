use specs::{System, ReadStorage, WriteStorage, Read, Join};
use component::physics::{Position, Velocity, Force, DeltaTime};
use config;
use system::rules::{Rules, RulesState};

/// Velocity system is use to move any
/// entity that have a velocity and a position
pub struct VelocitySystem;
impl<'a> System<'a> for VelocitySystem {
    type SystemData = (
        ReadStorage<'a, Velocity>,
        WriteStorage<'a, Position>,
        Read<'a, DeltaTime>,
        Read<'a, Rules>,
    );

    fn run(&mut self, (vel, mut pos, delta_time, rules): Self::SystemData) {
        if rules.state != RulesState::RUN {
            return;
        }

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.0 += vel.0 * (*delta_time);
        }
    }
}

/// Newton system just apply the
/// Second law of newton by computing dv = F * dt
/// This system have no mass
pub struct NewtonSystem;
impl<'a> System<'a> for NewtonSystem {
    type SystemData = (
        ReadStorage<'a, Force>,
        WriteStorage<'a, Velocity>,
        Read<'a, DeltaTime>
    );

    fn run(&mut self, (force, mut vol, delta_time): Self::SystemData) {
        for (force, vol) in (&force, &mut vol).join() {
            vol.x += force.x * (*delta_time);
            vol.y += force.y * (*delta_time);
        }
    }
}

/// Gravity check if an entity is not under the ground
pub struct GravitySystem;
impl<'a> System<'a> for GravitySystem {
    type SystemData = (
        WriteStorage<'a, Position>,
        WriteStorage<'a, Velocity>
    );

    fn run(&mut self, (mut pos, mut vol): Self::SystemData) {
        for (pos, vol) in (&mut pos, &mut vol).join() {
            if pos.y < config::GROUND {
                pos.y = config::GROUND;
                vol.y = 0.0
            }
        }
    }
}