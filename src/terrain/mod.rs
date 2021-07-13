pub mod chunk;
pub mod map_terrain;

use gdnative::prelude::*;
use gdnative::api::*;
use map_terrain::MapTerrain;
use chunk::Chunk;

const CHUNK_REF: i64 = 32;

#[derive(Debug)]
#[derive(NativeClass)]
#[inherit(GridMap)]
pub struct Terrain {
  pub map_data: MapTerrain,
}

impl Terrain {
  fn new(_owner: &GridMap) -> Self {
    Terrain {
      map_data: MapTerrain::new(),
    }
  }
}

#[methods]
impl Terrain {

  #[export]
  fn _ready(&mut self, grid_map: &GridMap) {
    self.map_data.generate_map_seed();
    self.map_data.create_chunk(grid_map);
  }

  #[export]
  fn _process(&mut self, grid_map: &GridMap, _delta: f32) {
    if let Some(player) = grid_map
      .get_node("../Player")
      .map(|node| unsafe { node.assume_safe() })
      .and_then(|node| node.cast::<KinematicBody>())
    {
      let position = player.global_transform().origin;
      self.map_data.position = position;
    }
  }

  #[export]
  fn _on_timeout(&mut self, grid_map: &GridMap) {
    if let Some(player) = grid_map
      .get_node("../Player")
      .map(|node| unsafe { node.assume_safe() })
      .and_then(|node| node.cast::<KinematicBody>())
    {
      let position = player.global_transform().origin;
      if self.map_data.current_area.needs_generate_chunks(position) {
        let tile_position: Vector3 = grid_map.world_to_map(position);
        let x_pos = tile_position.x as i64;
        let z_pos = tile_position.z as i64;
        let new_chunk = Chunk {
          min_x: x_pos - CHUNK_REF,
          max_x: x_pos + CHUNK_REF,
          min_z: z_pos - CHUNK_REF,
          max_z: z_pos + CHUNK_REF,
          map_point: Vector3Array::new(),
          placed_blocks: self.map_data.current_area.placed_blocks.clone()
        };
        self.map_data.current_area = new_chunk;
        self.map_data.update_chunk(grid_map);
      }
      self.map_data.position = position;
    }
  }
}