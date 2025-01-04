use noise::{NoiseFn, Perlin};
use glam::Vec2;
use crate::world::block::BlockType;

#[derive(Debug, Clone)]
pub struct BiomeProperties {
    pub temperature: f32,
    pub rainfall: f32,
    pub terrain_height_multiplier: f32,
    pub terrain_roughness: f32,
    pub tree_density: f32,
    pub grass_density: f32,
    pub top_block: BlockType,
    pub under_block: BlockType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BiomeType {
    Plains,
    Desert,
    Mountains,
    Forest,
    Tundra,
    Savanna,
    Jungle,
    Ocean,
}

impl BiomeType {
    pub fn get_properties(&self) -> BiomeProperties {
        match self {
            BiomeType::Plains => BiomeProperties {
                temperature: 0.5,
                rainfall: 0.4,
                terrain_height_multiplier: 1.0,
                terrain_roughness: 0.5,
                tree_density: 0.01,
                grass_density: 0.7,
                top_block: BlockType::Grass,
                under_block: BlockType::Dirt,
            },
            BiomeType::Desert => BiomeProperties {
                temperature: 2.0,
                rainfall: 0.0,
                terrain_height_multiplier: 0.8,
                terrain_roughness: 0.3,
                tree_density: 0.001,
                grass_density: 0.0,
                top_block: BlockType::Sand,
                under_block: BlockType::Sand,
            },
            BiomeType::Mountains => BiomeProperties {
                temperature: 0.2,
                rainfall: 0.5,
                terrain_height_multiplier: 3.0,
                terrain_roughness: 1.5,
                tree_density: 0.05,
                grass_density: 0.3,
                top_block: BlockType::Stone,
                under_block: BlockType::Stone,
            },
            BiomeType::Forest => BiomeProperties {
                temperature: 0.7,
                rainfall: 0.8,
                terrain_height_multiplier: 1.1,
                terrain_roughness: 0.6,
                tree_density: 0.09,
                grass_density: 0.6,
                top_block: BlockType::Grass,
                under_block: BlockType::Dirt,
            },
            BiomeType::Tundra => BiomeProperties {
                temperature: -0.5,
                rainfall: 0.3,
                terrain_height_multiplier: 0.9,
                terrain_roughness: 0.4,
                tree_density: 0.005,
                grass_density: 0.2,
                top_block: BlockType::Snow,
                under_block: BlockType::Dirt,
            },
            BiomeType::Jungle => BiomeProperties {
                temperature: 1.2,
                rainfall: 0.9,
                terrain_height_multiplier: 1.2,
                terrain_roughness: 0.7,
                tree_density: 0.1,
                grass_density: 0.9,
                top_block: BlockType::Grass,
                under_block: BlockType::Dirt,
            },
            BiomeType::Savanna => BiomeProperties {
                temperature: 1.5,
                rainfall: 0.9,
                terrain_height_multiplier: 1.2,
                terrain_roughness: 0.7,
                tree_density: 0.1,
                grass_density: 0.9,
                top_block: BlockType::Grass,
                under_block: BlockType::Dirt,
            },
            BiomeType::Ocean => BiomeProperties {
                temperature: 0.5,
                rainfall: 1.0,
                terrain_height_multiplier: 0.3,
                terrain_roughness: 0.2,
                tree_density: 0.0,
                grass_density: 0.0,
                top_block: BlockType::Sand,
                under_block: BlockType::Sand,
            },
        }
    }
}

pub struct BiomeGenerator {
    temperature_noise: Perlin,
    rainfall_noise: Perlin,
}

impl BiomeGenerator {
    pub fn new(seed: u32) -> Self {
        Self {
            temperature_noise: Perlin::new(seed),
            rainfall_noise: Perlin::new(seed.wrapping_add(1)),
        }
    }
    
    pub fn get_biome(&self, wx: f64, wz: f64) -> BiomeType {
        let scale = 0.03;
        let temperature = self.temperature_noise.get([
            wx * scale,
            wz * scale,
        ]);
        let rainfall = self.rainfall_noise.get([
            wx * scale * 1000.0,
            wz * scale * 1000.0,
        ]);
        
        match (temperature, rainfall) {
            (t, r) if t < -0.5 => BiomeType::Tundra,
            (t, r) if t > 0.5 && r < -0.3 => BiomeType::Desert,
            (t, r) if t > 0.3 && r > 0.3 => BiomeType::Jungle,
            (t, r) if t > 0.0 && r > 0.2 => BiomeType::Forest,
            (t, r) if t > 0.2 && r < 0.0 => BiomeType::Savanna,
            (t, r) if r > 0.7 => BiomeType::Ocean,
            (t, r) if t > 0.6 || t < -0.6 => BiomeType::Mountains,
            _ => BiomeType::Plains,
        }
    }
}
