use gdnative::prelude::*;
use gdnative::api::*;
use interpolation::Lerp;
use super::CHUNK_REF;

/* ROTATION OF TILES:
It seems they form a "nice" sequence (except for the last 4) if we assume that they're rotation matrices written in ZYX-convention Euler angles (which is different from Godot's overall X-Y-Z convention).

First, let me define a simple notation, in which trio of ZYX Euler angles are written as a vector in units of π/2. For example, a rotation of R_z(π/2).R_y(2π/2).R_x(3π/2) is written as (1,2,3). To further simplify the notation, I'll just write 1 2 3

Then the 24 orthogonal matrices correspond to Euler rotations given as
*/

#[derive(Debug, Clone)]
pub struct CustomBlock {
  position: Vector3,
  type_block: i64
}

#[derive(Debug, Clone)]
pub struct Chunk {
  pub min_x: i64,
  pub max_x: i64,
  pub min_z: i64,
  pub max_z: i64,
  pub map_point: Vector3Array,
  pub placed_blocks: Vec<CustomBlock>,
}
impl Chunk {
  pub fn new() -> Self {
    Chunk {
      min_x: -CHUNK_REF,
      max_x: CHUNK_REF,
      min_z: -CHUNK_REF,
      max_z: CHUNK_REF,
      map_point: Vector3Array::new(),
      placed_blocks: Vec::new()
    }
  }

  pub fn generate_map_points(&mut self, noise: TRef<OpenSimplexNoise, Unique>) {
    let noise_ref = noise.as_ref();
    for x in self.min_x..self.max_x {
      for z in self.min_z..self.max_z {
        let noise_value = noise_ref.get_noise_2d(x as f64, z as f64);        
        let weight = (noise_value + 1.0) / 2.0;
        let interpolation: f64 = Lerp::lerp(&0.0, &25.0, &weight);
        let y = interpolation.floor() as i64;
        self.map_point.push(Vector3::new(x as f32, y as f32, z as f32));
      }
    }
  }

  pub fn generate_chunk(&mut self, grid: &GridMap, noise: TRef<OpenSimplexNoise, Unique>) {
    self.generate_map_points(noise);
    grid.clear();
    self.place_floor_blocks(grid);
    self.place_wall_blocks(grid);
  }

  pub fn redraw(&mut self, grid: &GridMap) {
    self.place_custom_blocks(grid);
  }


  pub fn place_floor_blocks(&mut self, grid: &GridMap) {
    for point in self.map_point.read().iter() {
      let x = point.x  as i64;
      let y = point.y  as i64;
      let z = point.z  as i64;
      grid.set_cell_item(x, y, z, 0, 0);
    }
  }

  pub fn place_wall_blocks(&mut self, grid: &GridMap) {
  for point in self.map_point.read().iter() {
    let x = point.x  as i64;
    let y = point.y  as i64;
    let z = point.z  as i64;
    let ant_x = grid.get_cell_item(x - 1, y, z);
    let pos_x = grid.get_cell_item(x + 1, y, z);
    let ant_z = grid.get_cell_item(x, y, z - 1);
    let pos_z = grid.get_cell_item(x, y, z + 1);
    let pre_yz = grid.get_cell_item(x, y + 1, z - 1);
    let pos_yz = grid.get_cell_item(x, y + 1, z + 1);
    let pre_xy = grid.get_cell_item(x - 1, y + 1, z);
    let pos_xy = grid.get_cell_item(x + 1, y + 1, z);

    if pre_xy == -1 && pos_xy  == -1 && pre_yz  == -1 && pos_yz == -1 {
      match (ant_x, pos_x, ant_z, pos_z) {
        (0..=10, -1, -1, -1) => grid.set_cell_item(x, y, z, 3, 19),
        (-1, 0..=10, -1, -1) => grid.set_cell_item(x, y, z, 3, 16),
        (-1, -1, 0..=10, -1) => grid.set_cell_item(x, y, z, 3, 4),
        (-1, -1, -1, 0..=10) => grid.set_cell_item(x, y, z, 3, 0),

        (0..=10, 0..=10, -1, -1) => grid.set_cell_item(x, y, z, 2, 0),
        (0..=10, -1, 0..=10, -1) => grid.set_cell_item(x, y, z, 2, 4),
        (0..=10, -1, -1, 0..=10) => grid.set_cell_item(x, y, z, 2, 0),
        (-1, 0..=10, 0..=10, -1) => grid.set_cell_item(x, y, z, 2, 5),
        (-1, 0..=10, -1, 0..=10) => grid.set_cell_item(x, y, z, 2, 1),
        (-1, -1, 0..=10, 0..=10) => grid.set_cell_item(x, y, z, 2, 0),

        (-1, 0..=10, 0..=10, 0..=10) => grid.set_cell_item(x, y, z, 1, 1),
        (0..=10, -1, 0..=10, 0..=10) => grid.set_cell_item(x, y, z, 1, 0),
        (0..=10, 0..=10, -1, 0..=10) => grid.set_cell_item(x, y, z, 1, 13),
        (0..=10, 0..=10, 0..=10, -1) => grid.set_cell_item(x, y, z, 1, 5),

        //TODO: Pending to add double height faces, and 2 open sides for z

        (-1, -1, -1, -1) => grid.set_cell_item(x, y, z, 4, 0),
        _ => (),
      };
    }
  }
  }

  pub fn place_custom_blocks(&mut self, grid: &GridMap) {
    for point in self.placed_blocks.iter() {
      let x = point.position.x  as i64;
      let y = point.position.y  as i64;
      let z = point.position.z  as i64;
      let value = point.type_block;

      if x >= self.min_x && x <= self.max_x && z >= self.min_z && z <=self.max_z {
        grid.set_cell_item(x, y, z, value, 0);
      }

    }
  }

  pub fn insert_block(&mut self, position: Vector3, type_block: i64) {
    let new_block = CustomBlock {
      position: Vector3::new(position.x as f32, position.y as f32, position.z as f32),
      type_block: type_block
    };
    self.placed_blocks.push(new_block);
  }

  pub fn needs_generate_chunks( &self, position: Vector3) -> bool {
    let x = position.x as i64;
    let z = position.z as i64;
    let res_x = -(self.min_x + (-x) ) < 16 || self.max_x - x < 16;
    let res_z = -(self.min_z + (-z) ) < 16 || self.max_z - z < 16;
    if res_x || res_z {
      true
    } else {
      false
    }
  }
}