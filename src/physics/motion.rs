use glam::Vec3;

#[derive(Debug)]
pub struct Motion {
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub on_ground: bool,
    pub jumping: bool,
}

impl Motion {
    pub fn new() -> Self {
        Self {
            velocity: Vec3::ZERO,
            acceleration: Vec3::ZERO,
            on_ground: false,
            jumping: false,
        }
    }
}
