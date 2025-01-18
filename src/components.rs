use specs::prelude::*;
use specs_derive::Component;
use vector2d::Vector2D;

#[derive(Component)]
pub struct Position {
  pub x: f64,
  pub y: f64,
  pub rot: f64
}

// A renderable item and details about image
#[derive(Component)]
pub struct Renderable {
  // Name of texture to be rendered
  pub tex_name: String,
  // Width of src
  pub i_w: u32,
  // Height of src
  pub i_h: u32,
  // Witdh of dest
  pub o_w: u32,
  // Height of dest
  pub o_h: u32,
  // Offset number of width to crop
  pub frame: u32,
  // Max frame offset before
  pub total_frames: u32,
  // Rotation
  pub rot: f64
}

// Player
#[derive(Component)]
pub struct Player {
  pub impulse: Vector2D<f64>, // The next impluse to add to the speed
  pub cur_speed: Vector2D<f64> // Current speed
}

#[derive(Component)]
pub struct Asteroid {
  pub speed: f64,
  pub rot_speed: f64
}

#[derive(Component)]
pub struct Missile {
  pub speed: f64
}

#[derive(Component)]
pub struct Smoke {
  pub speed: f64,
  pub slack: f64, // Divide speed by this
  pub shrink_time: f64, // When zero, shrinks the output size by the shrink_factor and resets to 1.0
  pub shrink_speed: f64, // Subtract this from shrink_time
  pub shrink_factor: u32
}
