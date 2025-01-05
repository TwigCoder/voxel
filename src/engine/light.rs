use glam::{Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    position: [f32; 4],
    color: [f32; 4],
    direction: [f32; 4],
    ambient: [f32; 4],
    attenuation: [f32; 4],
    params: [f32; 4],
}

pub struct Light {
    pub position: Vec3,
    pub color: Vec3,
    pub direction: Vec3,
    pub ambient: Vec3,
    pub uniform: LightUniform,
}

impl Light {
    pub fn new(position: Vec3, color: Vec3, direction: Vec3) -> Self {
        let uniform = LightUniform {
            position: [position.x, position.y, position.z, 1.0],
            color: [color.x, color.y, color.z, 1.0],
            direction: [direction.x, direction.y, direction.z, 1.0],
            ambient: [0.1, 0.1, 0.1, 1.0],
            attenuation: [1.0, 0.09, 0.032, 0.0],
            params: [1.0, 32.0, 0.0, 0.0],
        };
        
        Self {
            position,
            color,
            direction,
            ambient: Vec3::new(0.1, 0.1, 0.1),
            uniform,
        }
    }
    
    pub fn update(&mut self) {
        self.uniform.position = [self.position.x, self.position.y, self.position.z, 1.0];
        self.uniform.direction = [self.direction.x, self.direction.y, self.direction.z, 1.0];
    }
}
