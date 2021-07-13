pub mod bag;

use gdnative::api::*;
use gdnative::prelude::*;
use rand::Rng;
use super::Terrain;
use bag::Bag;
use bag::ItemBag;
use gdnative::prelude::Vector3;

const GRAVITY: f32= 20.0;
const SPEED: f32 = 8.0;

const ANGULAR_VELOCITY: f32 = 30.0;
const ACCEL_DEFAULT: f32 = 10.0;
const ACCEL_AIR: f32 = 1.0;
const JUMP: f32 = 10.0;

#[derive(NativeClass)]
#[inherit(KinematicBody)]
pub struct Player {
  main_tool: i32,
  bag: Bag,
  velocity: Vector3,
  movement: Vector3,
  direction: Vector3,
  snap: Vector3,
  accel: f32,
  gravity_vec: Vector3
}
impl Player {
  fn new(_owner: &KinematicBody) -> Self {
    Player {
      accel: ACCEL_DEFAULT,
      bag: Bag::new(),
      direction: Vector3::zero(),
      gravity_vec:  Vector3::zero(),
      main_tool: 0,
      movement: Vector3::zero(),
      snap: Vector3::zero(),
      velocity: Vector3::new(0.0, 0.0, 0.0),
    }
  }


  fn get_head(owner: &KinematicBody) -> TRef<Spatial, Shared> {
    owner
      .get_node("Head")
      .map(|node| unsafe { node.assume_safe() })
      .and_then(|node| node.cast::<Spatial>())
      .unwrap()
  }


  fn get_cam_base(owner: &KinematicBody) -> TRef<Spatial, Shared> {
    owner
      .get_node("Head/CamBase")
      .map(|node| unsafe { node.assume_safe() })
      .and_then(|node| node.cast::<Spatial>())
      .unwrap()
  }

  fn get_graphics(owner: &KinematicBody) -> TRef<Spatial, Shared> {
    owner
      .get_node("Graphics")
      .map(|node| unsafe { node.assume_safe() })
      .and_then(|node| node.cast::<Spatial>())
      .unwrap()
  }

  fn get_raycast(owner: &KinematicBody) -> TRef<RayCast, Shared> {
    owner
      .get_node("Head/RayCast")
      .map(|node| unsafe { node.assume_safe() })
      .and_then(|node| node.cast::<RayCast>())
      .unwrap()
  }

  fn get_gridmap(owner: &KinematicBody) -> TRef<GridMap, Shared> {
    owner
      .get_node("../GridMap")
      .map(|node| unsafe { node.assume_safe() })
      .and_then(|node| node.cast::<GridMap>())
      .unwrap()
  }

  fn get_animation(owner: &KinematicBody) -> TRef<AnimationPlayer, Shared> {
    owner
      .get_node("Graphics/Armature/AnimationPlayer")
      .map(|node| unsafe { node.assume_safe() })
      .and_then(|node| node.cast::<AnimationPlayer>())
      .unwrap()
  }

  fn lerp(start: f32, end: f32, amount: f32) -> f32 {
    // Don't do anything if they are equal
    if start == end {
        return start;
    }
    let lerp_amount = (start - end).abs() * amount;
    if start < end {
        start + lerp_amount
    } else {
        start - lerp_amount
    }
  }

  fn lerp_3d(start: Vector3, end: Vector3, percent: f32) -> Vector3 {
    let mut sum = end - start;
    sum *= percent;
    start + sum
  }

  fn clamp(value: f32, min: f32, max: f32) -> f32 {
    assert!(min <= max);
    let mut x = value;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
    x
  }

  fn rotated(vector: Vector3, normal: Vector3, phi: f32) -> Vector3 {
    let rotation_matrix = euclid::Transform3D::rotation(
        normal.x,
        normal.y,
        normal.z,
        euclid::Angle::radians(phi),
    );
    rotation_matrix.transform_vector3d(vector)
}
}

#[methods]
impl Player {
  #[export]
  fn _ready(&mut self, player: &KinematicBody) {
    let anim = Player::get_animation(player).as_ref();
    anim.play("Idle", 0.0, 1.0, false);
  }

  #[export]
  fn _process(&mut self, owner: &KinematicBody, delta: f32) {
    let input = Input::godot_singleton();
    let engine = Engine::godot_singleton();
    let fps = engine.get_frames_per_second();
    let cam_base = Player::get_cam_base(owner).as_ref();
    let head = Player::get_head(owner).as_ref();
    let graphics = Player::get_graphics(owner).as_ref();

    if Input::is_action_just_pressed(&input, "editor_mode") {
      self.main_tool = if self.main_tool == 0 { 1 } else { 0 };
    }

	  //physics interpolation to reduce jitter on high refresh-rate monitors
    if fps > engine.iterations_per_second() as f64 {
      cam_base.set_as_toplevel(true);

      cam_base.global_transform().origin = Player::lerp_3d(cam_base.global_transform().origin, head.global_transform().origin, 0.25 * delta);
      cam_base.rotation().y = owner.rotation().y;
      cam_base.rotation().x = owner.rotation().x;

      graphics.global_transform().origin = Player::lerp_3d(graphics.global_transform().origin, owner.global_transform().origin, 0.25 * delta);
      cam_base.rotation().x = head.rotation().x;
    } else {
      cam_base.set_as_toplevel(false);
      cam_base.set_global_transform(head.global_transform());
      graphics.global_transform().origin = owner.global_transform().origin;
    }

    //turns body in the direction of movement
    if self.direction != Vector3::zero() {
      graphics.rotation().y = Player::lerp(graphics.rotation().y, -self.direction.x.atan2(-self.direction.z), ANGULAR_VELOCITY * delta)
    }
  }
  
  #[export]
  fn _input(&mut self, owner: &KinematicBody, event: Ref<InputEvent>) {
    let ev_input = unsafe { event.assume_safe() };	

    if ev_input.get_class().to_string() == "InputEventMouseMotion" {
      let ev = ev_input.cast::<InputEventMouseMotion>().unwrap();

      let cam = Player::get_cam_base(owner).as_ref();
      let mut current_camera_rotation = cam.rotation();
      let head = Player::get_head(owner).as_ref();

      
      owner.rotate_y((-ev.relative().x * 0.25).to_radians() as f64);
      head.rotate_x((-ev.relative().y * 0.25).to_radians() as f64);
      current_camera_rotation.x = Player::clamp(head.rotation().x, -89.0f32.to_radians(), 89.0f32.to_radians());

    }

  }
  
  #[export]
  fn _physics_process(&mut self, owner: &KinematicBody, delta: f32) {
    let anim = Player::get_animation(owner).as_ref();

    let input = Input::godot_singleton();
    self.direction = Vector3::zero();
    let h_rot = owner.global_transform().basis.to_euler().y;
    let f_input = Input::get_action_strength(&input, "down") - Input::get_action_strength(&input, "up");
    let h_input = Input::get_action_strength(&input, "right") - Input::get_action_strength(&input, "left");
    let new_direction = Player::rotated(Vector3::new(h_input as f32, 0.0, f_input as f32), Vector3::new(0.0, 1.0, 0.0), h_rot); 

    if Input::is_action_pressed(&input, "down") ||
      Input::is_action_pressed(&input, "up") ||
      Input::is_action_pressed(&input, "left") ||
      Input::is_action_pressed(&input, "right")
    {
      anim.play("Run", 0.0, 1.0, false);
    } else {
      anim.play("Idle", 0.0, 1.0, false);
    }

    if new_direction.length() as f32 > 0.0 {
      self.direction = new_direction.normalize();
    }

    if owner.is_on_floor() {
      self.snap = -owner.get_floor_normal();
      self.accel = ACCEL_DEFAULT;
      self.gravity_vec = Vector3::zero();
    } else {
      self.snap = Vector3::new(0.0, -1.0, 0.0);
      self.accel = ACCEL_AIR;
      self.gravity_vec += Vector3::new(0.0, -1.0, 0.0) * GRAVITY * delta;
      anim.play("JumpUp", 0.0, 1.0, false);
    }

    if Input::is_action_just_pressed(&input, "jump") && owner.is_on_floor() {
      self.snap = Vector3::zero();
      self.gravity_vec += Vector3::new(0.0, 1.0, 0.0) * JUMP;
      anim.play("JumpUp", 0.0, 1.0, false);
    }
    
    self.velocity = Player::lerp_3d(self.velocity, self.direction * SPEED, self.accel * delta);
    self.movement = self.velocity + self.gravity_vec;
    owner.move_and_slide_with_snap(self.movement, self.snap, Vector3::new(0.0, 1.0, 0.0), false, 4, 0.785398, true);

    if Input::is_action_just_pressed(&input, "mouse_click_left") {
      // NOE: This should be splitted in its own method
      if let Some(viewport) = owner
        .get_viewport()
        .map(|node| unsafe { node.assume_safe() })
      {
        let mouse_pos = viewport.as_ref().get_mouse_position();
        godot_print!("viewport: {:?}", mouse_pos);
        let ray = Player::get_raycast(owner).as_ref();
        ray.force_raycast_update();
        let ray_point = ray.get_collision_point();
        let ray_collider = ray.get_collider();
        let map = Player::get_gridmap(owner).as_ref();
        let local_point = map.to_local(ray_point);
        let grid_point = map.world_to_map(local_point);
        let touched_tile = map.get_cell_item(
          grid_point.x as i64,
          grid_point.y as i64,
          grid_point.z as i64,
        );
        godot_print!(
          "collider: {:?},raypoint: {:?}, grid point {:?}, tile point {:?}",
          ray_collider,
          ray_point,
          grid_point,
          touched_tile
        );
        if self.main_tool == 1 {
          let view_insance_opt: Option<RefInstance<Terrain, Shared>> = RefInstance::try_from_base(Player::get_gridmap(owner));
          if let Some(view_instance) = view_insance_opt {
            view_instance.map_mut(|view, _| {
              let new_object = Vector3::new(grid_point.x, grid_point.y + 1.0, grid_point.z);
              view.map_data.place_block(&map, new_object, 4);
            }).expect("Building from map failed!");
          }
        } else {
          let mut rng = rand::thread_rng();
          let seed = rng.gen_range(0..4);
          if let Some(res) = self.bag.objects.iter_mut().find(|x| x.id == seed) {
            res.amount += 1;
            godot_print!("Mineado encontró! {:?}", res);
            godot_print!("Mineado ! {:?}", self.bag.objects);
          } else {
            let new_item = ItemBag {
              id: seed,
              name: "Test".to_string(),
              amount: 0,
            };
            self.bag.objects.push(new_item);
            godot_print!("Mineado agregó! {:?}", self.bag.objects);
          }
        }
      }
    }
	}
}