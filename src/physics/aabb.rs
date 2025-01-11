use glam::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3,
}

impl AABB {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self {min, max}
    }
    
    pub fn from_pos_size(position: Vec3, size: Vec3) -> Self {
        Self {
            min: position,
            max: position + size,
        }
    }
    
    pub fn intersects(&self, other: &AABB) -> bool {
        self.min.x <= other.max.x && self.max.x >= other.min.x &&
        self.min.y <= other.max.y && self.max.y >= other.min.y &&
        self.min.z <= other.max.z && self.max.z >= other.min.z
    }
    
    pub fn translate(&mut self, offset: Vec3) {
        self.min += offset;
        self.max += offset;
    }
    
    pub fn get_size(&self) -> Vec3 {
        self.max - self.min
    }
}
