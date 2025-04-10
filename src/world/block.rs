#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Dirt,
    Grass,
    Stone,
    Wood,
    Leaves,
    Sand,
    Water,
    Bedrock,
    DiamondOre,
    IronOre,
    CoalOre,
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

impl BlockType {
    pub fn is_transparent(&self) -> bool {
        matches!(self, BlockType::Air | BlockType::Water | BlockType::Leaves)
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
            BlockType::Water => [0.0, 0.3, 0.8],
            BlockType::Bedrock => [0.2, 0.2, 0.2],
            BlockType::DiamondOre => [0.0, 0.8, 0.8],
            BlockType::IronOre => [0.8, 0.7, 0.6],
            BlockType::CoalOre => [0.2, 0.2, 0.2],
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
}
