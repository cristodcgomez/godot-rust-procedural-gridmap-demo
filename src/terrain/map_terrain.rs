use gdnative::prelude::*;
use gdnative::api::*;
use super::Chunk;
use rand::Rng;

#[derive(Debug)]
pub struct MapTerrain {
  pub block: i64,
  pub seed: i64,
  pub position: Vector3,
  pub current_area: Chunk,
  pub map_noise: Ref<OpenSimplexNoise, Unique>
}
impl MapTerrain {
  pub fn new() -> Self {
    MapTerrain {
      block: 0,
      seed: rand::thread_rng().gen_range(0..75000000),
      position: Vector3::new(0.0, 0.0, 0.0),
      current_area: Chunk::new(),
      map_noise: OpenSimplexNoise::new()
    }
  }

  pub fn generate_map_seed(&self) {
    self.map_noise.set_seed(self.seed);
    godot_print!("seed: {:?}", self.map_noise.seed());
  }

  pub fn create_chunk(&mut self, grid: &GridMap) {
    self.current_area.generate_chunk(grid, self.map_noise.as_ref());
  }

  pub fn update_chunk(&mut self, grid: &GridMap) {
    self.current_area.generate_chunk(grid, self.map_noise.as_ref());
    self.current_area.redraw(grid);
  }

  pub fn place_block(&mut self, grid: &GridMap, position: Vector3, id: i64) {
    self.current_area.insert_block(position, id);
    self.current_area.redraw(grid);
  }
}
