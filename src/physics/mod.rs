pub mod collision;
pub mod physics_system;
pub mod aabb;
pub mod motion;

use glam::Vec3;

pub const GRAVITY: f32 = -9.81;
pub const TERMINAL_VELOCITY: f32 = -54.0;
