use std::collections::HashMap;

use graphics::Renderer;
use util::Vector2D;

pub mod chunk;
pub mod portal;

pub use self::chunk::{Chunk, CHUNK_SIZE};
pub use self::portal::Portal;

/// World
/// Manages the map chunks within the world
pub struct World {
    //error_chunk: Chunk,
    chunks: HashMap<Vector2D<i32>, Chunk>,
}

impl World {
    pub fn new() -> World {
        World {
            //error_chunk: Chunk::load_from_name(String::from("error").unwrap(),
            chunks: HashMap::with_capacity(20),
        }
    }

    pub fn render(&mut self, renderer: &Renderer, centre_position: Vector2D<i32>) {
        let world_chunk_pos = World::world_to_chunk_position(centre_position);

        for y in -1..=1 {
            for x in -1..=1 {
                let chunk_pos = world_chunk_pos + Vector2D::new(x, y);

                //Load chunks around the center point
                if !self.chunks.contains_key(&chunk_pos) {
                    if let Some(chunk) = Chunk::load(chunk_pos) {
                        self.chunks.insert(chunk_pos, chunk);
                    }
                }

                if let Some(chunk) = self.chunks.get(&chunk_pos) {
                    chunk.render(renderer, centre_position);
                }
            }
        }
    }

    pub fn get_tile(&self, world_position: Vector2D<i32>) -> char {
        let chunk_position = World::world_to_chunk_position(world_position);
        self.chunks.get(&chunk_position).map_or(' ', |chunk| {
            let local_x = world_position.x % CHUNK_SIZE.x;
            let local_y = world_position.y % CHUNK_SIZE.y;
            chunk.get_tile(local_x as usize, local_y as usize)
        })
    }

    fn world_to_chunk_position(world_position: Vector2D<i32>) -> Vector2D<i32> {
        Vector2D::new(
            world_position.x / CHUNK_SIZE.x,
            world_position.y / CHUNK_SIZE.y,
        )
    }

    pub fn is_portal_at(&self, world_position: Vector2D<i32>) -> bool {
        self.get_tile(world_position) == '1'
    }

    pub fn get_portal_at(&self, world_position: Vector2D<i32>) -> Option<&Portal> {
        let chunk_position = World::world_to_chunk_position(world_position);
        let local_x = world_position.x % CHUNK_SIZE.x;
        let local_y = world_position.y % CHUNK_SIZE.y;

        self.chunks
            .get(&chunk_position)
            .unwrap()
            .get_portal(Vector2D::new(local_x, local_y))
    }
}
