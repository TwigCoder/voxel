use glam::Vec3;
use crate::world::chunk::Chunk;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Dirt,
    Grass,
    Stone,
    Wood,
    Leaves,
    Sand,
    Snow,
    Water,
    Glass,
    Bedrock,
    GoldOre,
    IronOre,
    CoalOre,
    DiamondOre,
    Obsidian,
    Lava,
    Gravel,
    Ice,
    Clay,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockFace {
    Top,
    Bottom,
    North,
    South,
    East,
    West,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockShape {
    Full,
    Slab,
    Stairs,
    Fence,
    Wall,
    Pillar,
    Cross,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockSoundType {
    Stone,
    Wood,
    Grass,
    Gravel,
    Metal,
    Glass,
    Cloth,
    Sand,
    Snow,
    Liquid,
    Slime,
}

#[derive(Debug, Clone, Copy)]
pub struct BlockSoundProperties {
    pub volume: f32,
    pub pitch: f32,
    pub break_sound: BlockSoundType,
    pub step_sound: BlockSoundType,
    pub place_sound: BlockSoundType,
    pub hit_sound: BlockSoundType,
    pub fall_sound: BlockSoundType,
}

impl Default for BlockSoundProperties {
    fn default() -> Self {
        Self {
            volume: 1.0,
            pitch: 1.0,
            break_sound: BlockSoundType::Stone,
            step_sound: BlockSoundType::Stone,
            place_sound: BlockSoundType::Stone,
            hit_sound: BlockSoundType::Stone,
            fall_sound: BlockSoundType::Stone,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlockTextureCoords {
    pub top: (f32, f32),
    pub bottom: (f32, f32),
    pub sides: [(f32, f32); 4],
}

impl Default for BlockTextureCoords {
    fn default() -> Self {
        Self {
            top: (0.0, 0.0),
            bottom: (0.0, 0.0),
            sides: [(0.0, 0.0); 4],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BlockProperties {
    pub hardness: f32,
    pub blast_resistance: f32,
    pub luminance: u8,
    pub requires_tool: bool,
    pub is_fluid: bool,
    pub is_solid: bool,
    pub is_transparent: bool,
    pub is_flammable: bool,
    pub shape: BlockShape,
    pub gravity_affected: bool,
    pub slipperiness: f32,
    pub jump_velocity_multiplier: f32,
    pub speed_multiplier: f32,
    pub temperature: f32,
    pub can_catch_fire: bool,
    pub can_melt: bool,
    pub freezing_point: f32,
    pub melting_point: f32,
    pub max_stack_size: u8,
    pub experience_drop: f32,
}

impl Default for BlockProperties {
    fn default() -> Self {
        Self {
            hardness: 1.0,
            blast_resistance: 1.0,
            luminance: 0,
            requires_tool: false,
            is_fluid: false,
            is_solid: true,
            is_transparent: false,
            is_flammable: false,
            shape: BlockShape::Full,
            gravity_affected: true,
            slipperiness: 0.6,
            jump_velocity_multiplier: 1.0,
            speed_multiplier: 1.0,
            temperature: 0.0,
            can_catch_fire: false,
            can_melt: false,
            freezing_point: 0.0,
            melting_point: 100.0,
            max_stack_size: 64,
            experience_drop: 0.0,
        }
    }
}

impl BlockType {
    pub fn is_transparent(&self) -> bool {
        matches!(
            self,
            BlockType::Air | BlockType::Water | BlockType::Glass | BlockType::Leaves | BlockType::Ice
        )
    }

    pub fn get_color(&self) -> [f32; 3] {
        match self {
            BlockType::Air => [0.0, 0.0, 0.0],
            BlockType::Dirt => [0.6, 0.3, 0.0],
            BlockType::Grass => [0.0, 0.8, 0.0],
            BlockType::Stone => [0.5, 0.5, 0.5],
            BlockType::Wood => [0.5, 0.3, 0.2],
            BlockType::Leaves => [0.0, 0.5, 0.0],
            BlockType::Sand => [0.85, 0.8, 0.6],
            BlockType::Snow => [0.9, 0.9, 0.9],
            BlockType::Water => [0.0, 0.3, 0.8],
            BlockType::Glass => [0.9, 0.9, 0.9],
            BlockType::Bedrock => [0.2, 0.2, 0.2],
            BlockType::GoldOre => [0.9, 0.8, 0.0],
            BlockType::IronOre => [0.8, 0.7, 0.6],
            BlockType::CoalOre => [0.2, 0.2, 0.2],
            BlockType::DiamondOre => [0.0, 0.8, 0.8],
            BlockType::Obsidian => [0.15, 0.1, 0.2],
            BlockType::Lava => [0.9, 0.3, 0.0],
            BlockType::Gravel => [0.5, 0.5, 0.5],
            BlockType::Ice => [0.8, 0.9, 1.0],
            BlockType::Clay => [0.7, 0.7, 0.8],
        }
    }

    pub fn get_properties(&self) -> BlockProperties {
        match self {
            BlockType::Air => BlockProperties {
                hardness: 0.0,
                is_solid: false,
                is_transparent: true,
                ..Default::default()
            },
            BlockType::Stone => BlockProperties {
                hardness: 1.5,
                blast_resistance: 6.0,
                requires_tool: true,
                ..Default::default()
            },
            BlockType::Dirt => BlockProperties {
                hardness: 0.5,
                blast_resistance: 0.5,
                ..Default::default()
            },
            BlockType::Grass => BlockProperties {
                hardness: 0.6,
                blast_resistance: 0.6,
                ..Default::default()
            },
            BlockType::Wood => BlockProperties {
                hardness: 2.0,
                blast_resistance: 2.0,
                is_flammable: true,
                ..Default::default()
            },
            BlockType::Leaves => BlockProperties {
                hardness: 0.2,
                blast_resistance: 0.2,
                is_transparent: true,
                is_flammable: true,
                ..Default::default()
            },
            BlockType::Water => BlockProperties {
                hardness: 100.0,
                is_fluid: true,
                is_solid: false,
                is_transparent: true,
                ..Default::default()
            },
            BlockType::Lava => BlockProperties {
                hardness: 100.0,
                is_fluid: true,
                is_solid: false,
                luminance: 15,
                ..Default::default()
            },
            BlockType::Glass => BlockProperties {
                hardness: 0.3,
                blast_resistance: 0.3,
                is_transparent: true,
                ..Default::default()
            },
            BlockType::Bedrock => BlockProperties {
                hardness: f32::INFINITY,
                blast_resistance: f32::INFINITY,
                ..Default::default()
            },
            BlockType::Obsidian => BlockProperties {
                hardness: 50.0,
                blast_resistance: 1200.0,
                requires_tool: true,
                ..Default::default()
            },
            _ => Default::default(),
        }
    }

    pub fn get_face_color(&self, face: BlockFace) -> [f32; 3] {
        let base_color = self.get_color();
        match face {
            BlockFace::Top => base_color,
            BlockFace::Bottom => [
                base_color[0] * 0.7,
                base_color[1] * 0.7,
                base_color[2] * 0.7,
            ],
            BlockFace::North | BlockFace::South => [
                base_color[0] * 0.8,
                base_color[1] * 0.8,
                base_color[2] * 0.8,
            ],
            BlockFace::East | BlockFace::West => [
                base_color[0] * 0.9,
                base_color[1] * 0.9,
                base_color[2] * 0.9,
            ],
        }
    }

    pub fn is_replaceable(&self) -> bool {
        matches!(self, BlockType::Air | BlockType::Water)
    }

    pub fn can_place_on(&self, other: BlockType) -> bool {
        other.get_properties().is_solid || matches!(other, BlockType::Glass)
    }

    pub fn get_mining_time(&self, tool_efficiency: f32) -> f32 {
        let hardness = self.get_properties().hardness;
        if hardness == 0.0 {
            0.0
        } else if hardness == f32::INFINITY {
            f32::INFINITY
        } else {
            hardness * 1.5 / tool_efficiency
        }
    }

    pub fn get_light_emission(&self) -> u8 {
        self.get_properties().luminance
    }

    pub fn get_light_reduction(&self) -> u8 {
        if self.is_transparent() {
            1
        } else {
            15
        }
    }

    pub fn can_conduct_redstone(&self) -> bool {
        matches!(
            self,
            BlockType::Stone
                | BlockType::Dirt
                | BlockType::Grass
                | BlockType::Sand
                | BlockType::Gravel
                | BlockType::Clay
        )
    }

    pub fn get_blast_resistance(&self) -> f32 {
        self.get_properties().blast_resistance
    }

    pub fn is_flammable(&self) -> bool {
        self.get_properties().is_flammable
    }

    pub fn requires_tool(&self) -> bool {
        self.get_properties().requires_tool
    }

    pub fn is_fluid(&self) -> bool {
        self.get_properties().is_fluid
    }

    pub fn is_solid(&self) -> bool {
        self.get_properties().is_solid
    }

    pub fn get_sound_properties(&self) -> BlockSoundProperties {
        match self {
            BlockType::Stone | BlockType::Bedrock | BlockType::Obsidian => BlockSoundProperties {
                break_sound: BlockSoundType::Stone,
                step_sound: BlockSoundType::Stone,
                ..Default::default()
            },
            BlockType::Wood => BlockSoundProperties {
                break_sound: BlockSoundType::Wood,
                step_sound: BlockSoundType::Wood,
                place_sound: BlockSoundType::Wood,
                ..Default::default()
            },
            BlockType::Grass => BlockSoundProperties {
                break_sound: BlockSoundType::Grass,
                step_sound: BlockSoundType::Grass,
                ..Default::default()
            },
            BlockType::Sand | BlockType::Gravel => BlockSoundProperties {
                break_sound: BlockSoundType::Gravel,
                step_sound: BlockSoundType::Gravel,
                fall_sound: BlockSoundType::Gravel,
                ..Default::default()
            },
            BlockType::Water | BlockType::Lava => BlockSoundProperties {
                volume: 0.5,
                break_sound: BlockSoundType::Liquid,
                step_sound: BlockSoundType::Liquid,
                ..Default::default()
            },
            _ => Default::default(),
        }
    }

    pub fn get_particle_effects(&self) -> Vec<ParticleEffect> {
        match self {
            BlockType::Dirt | BlockType::Grass => vec![ParticleEffect::Dust],
            BlockType::Stone => vec![ParticleEffect::Stone],
            BlockType::Water => vec![ParticleEffect::Drip, ParticleEffect::Splash],
            BlockType::Lava => vec![ParticleEffect::Flame, ParticleEffect::Smoke],
            BlockType::Snow => vec![ParticleEffect::Snowflake],
            _ => vec![],
        }
    }

    pub fn get_interaction_type(&self) -> BlockInteractionType {
        match self {
            BlockType::Water | BlockType::Lava => BlockInteractionType::Fluid,
            BlockType::Glass | BlockType::Ice => BlockInteractionType::Transparent,
            BlockType::Air => BlockInteractionType::Passthrough,
            _ => BlockInteractionType::Solid,
        }
    }

    pub fn get_collision_boxes(&self) -> Vec<BoundingBox> {
        match self {
            BlockType::Air => vec![],
            BlockType::Water | BlockType::Lava => vec![BoundingBox {
                min: Vec3::new(0.0, 0.0, 0.0),
                max: Vec3::new(1.0, 0.9, 1.0),
            }],
            _ => vec![BoundingBox {
                min: Vec3::new(0.0, 0.0, 0.0),
                max: Vec3::new(1.0, 1.0, 1.0),
            }],
        }
    }

    pub fn update_tick(&self, world: Option<&mut World>, pos: BlockPos) {
        if let Some(world) = world {
            match self {
                BlockType::Water => {
                    // COMING SOON IN NEXT UPDATE: FLUID LOGIC
                }
                BlockType::Lava => {
                    // COMING SOON IN NEXT UPDATE
                }
                BlockType::Ice => {
                    // COMING SOON IN NEXT UPDATE
                }
                _ => {}
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ParticleEffect {
    Dust,
    Stone,
    Drip,
    Splash,
    Flame,
    Smoke,
    Snowflake,
}

#[derive(Debug, Clone, Copy)]
pub enum BlockInteractionType {
    Solid,
    Fluid,
    Transparent,
    Passthrough,
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl BlockPos {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    pub fn get_neighbors(&self) -> Vec<BlockPos> {
        vec![
            BlockPos::new(self.x + 1, self.y, self.z),
            BlockPos::new(self.x - 1, self.y, self.z),
            BlockPos::new(self.x, self.y + 1, self.z),
            BlockPos::new(self.x, self.y - 1, self.z),
            BlockPos::new(self.x, self.y, self.z + 1),
            BlockPos::new(self.x, self.y, self.z - 1),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_properties() {
        assert!(BlockType::Air.is_transparent());
        assert!(!BlockType::Stone.is_transparent());
        assert!(BlockType::Water.is_fluid());
        assert!(BlockType::Wood.is_flammable());
        assert!(BlockType::Stone.requires_tool());
    }

    #[test]
    fn test_mining_times() {
        assert_eq!(BlockType::Air.get_mining_time(1.0), 0.0);
        assert_eq!(BlockType::Bedrock.get_mining_time(1.0), f32::INFINITY);
        assert!(BlockType::Stone.get_mining_time(1.0) > BlockType::Dirt.get_mining_time(1.0));
    }

    #[test]
    fn test_face_colors() {
        let stone = BlockType::Stone;
        let top_color = stone.get_face_color(BlockFace::Top);
        let bottom_color = stone.get_face_color(BlockFace::Bottom);
        assert!(bottom_color[0] < top_color[0]);
        assert!(bottom_color[1] < top_color[1]);
        assert!(bottom_color[2] < top_color[2]);
    }

    #[test]
    fn test_block_sounds() {
        let stone_sounds = BlockType::Stone.get_sound_properties();
        let water_sounds = BlockType::Water.get_sound_properties();
        assert_eq!(stone_sounds.break_sound, BlockSoundType::Stone);
        assert_eq!(water_sounds.break_sound, BlockSoundType::Liquid);
        assert!(water_sounds.volume < stone_sounds.volume);
    }

    #[test]
    fn test_block_particles() {
        assert!(!BlockType::Stone.get_particle_effects().is_empty());
        assert!(BlockType::Water.get_particle_effects().contains(&ParticleEffect::Splash));
        assert!(BlockType::Lava.get_particle_effects().contains(&ParticleEffect::Flame));
    }

    #[test]
    fn test_block_position() {
        let pos = BlockPos::new(0, 0, 0);
        let neighbors = pos.get_neighbors();
        assert_eq!(neighbors.len(), 6);
        assert!(neighbors.contains(&BlockPos::new(1, 0, 0)));
        assert!(neighbors.contains(&BlockPos::new(0, 1, 0)));
    }
}

pub struct World {
    pub chunks: Vec<Chunk>,
}
