use crate::{
    blocks::{
        AIR, BEDROCK, BROWN_TERRACOTTA, RED_SAND, RED_TERRACOTTA, STONE, TERRACOTTA,
        YELLOW_TERRACOTTA,
    },
    chunk::{Chunk, CHUNK_SIZE},
};
use bevy::math::Vec3;
use noise::{NoiseFn, Perlin};

pub struct Generator {
    pub seed: u32,
    perlin: Perlin,
}

impl Generator {
    pub fn new(seed: u32) -> Generator {
        Generator {
            seed,
            perlin: Perlin::new(seed),
        }
    }

    pub fn generate_chunk(&self, chunk: &mut Chunk) {
        let chunk_origin = chunk.position * CHUNK_SIZE as f32;
        if chunk_origin.y < 0.0 {
            return;
        }

        for x in -1..=CHUNK_SIZE {
            for y in -1..=CHUNK_SIZE {
                for z in -1..=CHUNK_SIZE {
                    let local_position = Vec3::new(x as f32, y as f32, z as f32);
                    let block_position = chunk_origin + local_position;

                    let block = self.generate_block(block_position);
                    chunk.set(x as usize, y as usize, z as usize, block);
                }
            }
        }
    }

    fn generate_block(&self, position: Vec3) -> u8 {
        let y = position.y as f64;
        let amplitude = 60.0;
        let base_height = 5.0;

        let height = (self.get_layered_2d_noise(3, position) + 1.0) * amplitude + base_height;

        if y == 0.0 {
            return BEDROCK;
        }

        if y > height {
            return AIR;
        }

        if y < 5.0 && y > height - 3.0 {
            return RED_SAND;
        }

        if y as i32 % 8 == 0 {
            return YELLOW_TERRACOTTA;
        }
        if (y as i32 + 1) % 8 == 0 {
            return RED_TERRACOTTA;
        }
        if (y as i32 + 2) % 8 == 0 {
            return BROWN_TERRACOTTA;
        }

        if height - y < 1.0 {
            return TERRACOTTA;
        }
        if height - y < 3.0 {
            return RED_SAND;
        }
        return STONE;
    }

    fn get_layered_2d_noise(&self, octaves: i32, position: Vec3) -> f64 {
        let mut height = 0.0;
        let mut frequency = 0.015;
        let mut persistence = 0.5;
        let lacuranity = 2.0;

        for _ in 0..octaves {
            height += self.get_2d_noise(position * frequency) * persistence;
            frequency *= 2.0;
            persistence *= 0.5;
        }

        height
    }

    fn get_2d_noise(&self, position: Vec3) -> f64 {
        self.perlin.get([position.x as f64, position.z as f64])
    }
}
