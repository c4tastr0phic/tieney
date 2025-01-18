use specs::{World, WorldExt, Builder, Join};
use std::collections::HashMap;
use vector2d::Vector2D;

use rand::prelude::*;

const ROTATION_SPEED: f64 = 2.5;
const MOVEMENT_SPEED: f64 = 4.0;

pub fn update(ecs: &mut World, key_manager: &mut HashMap<String, bool>) {
  // Check status of game world
  let mut must_reload_world: bool = false;
  {
    let players = ecs.read_storage::<crate::components::Player>();
    if players.join().count() < 1 {
      must_reload_world = true;
    }
  }

  if must_reload_world {
    ecs.delete_all();
    load_world(ecs);
  }

  let mut player_pos = crate::components::Position{ x: 0.0, y: 0.0, rot: 0.0 };
  let mut must_fire_missile: bool = false;
  let mut must_release_smoke: bool = false;

  {
    let mut positions = ecs.write_storage::<crate::components::Position>();
    let mut players = ecs.write_storage::<crate::components::Player>();
    let mut renderables = ecs.write_storage::<crate::components::Renderable>();
  
    for (player, pos, renderable) in (&mut players, &mut positions, &mut renderables).join() {
      if crate::utils::is_key_pressed(&key_manager, "D") {
        pos.rot -= ROTATION_SPEED;
      }
      if crate::utils::is_key_pressed(&key_manager, "A") {
        pos.rot += ROTATION_SPEED;
      }

      if crate::utils::is_key_pressed(&key_manager, "W") {
        let radians = pos.rot.to_radians();

        let move_x = MOVEMENT_SPEED * radians.sin();
        let move_y = -MOVEMENT_SPEED * radians.cos();
        let move_vec = Vector2D::<f64>::new(move_x, move_y);

        player.impulse += move_vec;

        must_release_smoke = true;

        let radians = pos.rot.to_radians();
        player_pos.x = pos.x - renderable.o_h as f64/2.0 * radians.sin();
        player_pos.y = pos.y + renderable.o_h as f64/2.0 * radians.cos();
        player_pos.rot = pos.rot + rand::thread_rng().gen_range(-20.0, 20.0);
      }

      update_movement(pos, player);

      if pos.rot > 360.0 { pos.rot -= 360.0 }
      if pos.rot < 0.0 { pos.rot += 360.0 }

      if pos.x > crate::GAME_WIDTH.into() {
        pos.x -= crate::GAME_WIDTH as f64;
      }
      if pos.x < 0.0 {
        pos.x += crate::GAME_WIDTH as f64;
      }
      if pos.y > crate::GAME_HEIGHT.into() {
        pos.y -= crate::GAME_HEIGHT as f64;
      }
      if pos.y < 0.0 {
        pos.y += crate::GAME_HEIGHT as f64;
      }

      if crate::utils::is_key_pressed(&key_manager, " ") {
        crate::utils::key_up(key_manager, " ".to_string());
        must_fire_missile = true;
        player_pos.x = pos.x;
        player_pos.y = pos.y;
        player_pos.rot = pos.rot+180.0;
        if player_pos.rot > 360.0 { player_pos.rot -= 360.0 }
      }
      
      // Update the graphic to reflect the rotation
      renderable.rot = pos.rot;
    }
  }

  if must_release_smoke {
    release_smoke(ecs, player_pos);
  }
}

const MAX_SPEED: f64 = 4.5;
const FRICTION: f64 = 0.95;

pub fn update_movement(pos: &mut crate::components::Position, player: &mut crate::components::Player) {
  player.cur_speed *= FRICTION;

  player.cur_speed += player.impulse;
  if player.cur_speed.length() > MAX_SPEED {
    player.cur_speed = player.cur_speed.normalise();
    player.cur_speed = player.cur_speed * MAX_SPEED;
  }

  pos.x += player.cur_speed.x;
  pos.y += player.cur_speed.y;

  player.impulse = vector2d::Vector2D::new(0.0,0.0);
}

pub fn load_world(ecs: &mut World) {
  ecs.create_entity()
    .with(crate::components::Position{ x: 350.0, y: 250.0, rot: 0.0 })
    .with(crate::components::Renderable{
      tex_name: String::from("img/triangle.png"),
      i_w: 32,
      i_h: 32,
      o_w: 64,
      o_h: 64,
      frame: 0,
      total_frames: 1,
      rot: 0.0
    })
    .with(crate::components::Player{
      impulse: vector2d::Vector2D::new(0.0,0.0),
      cur_speed: vector2d::Vector2D::new(0.0,0.0)
    })
    .build();
  ecs.create_entity()
    .with(crate::components::Position{ x: 200.0, y: 400.0, rot: 45.0 })
    .with(crate::components::Renderable{
      tex_name: String::from("img/square.png"),
      i_w: 32,
      i_h: 32,
      o_w: 64,
      o_h: 64,
      frame: 0,
      total_frames: 1,
      rot: 0.0
    })
    .with(crate::components::Asteroid{
      speed: 2.5,
      rot_speed: 0.25
    })
    .build();
}

const MAX_MISSILES: usize = 10000;

fn fire_missile(ecs: &mut World, position: crate::components::Position) {
  {
    let missiles = ecs.read_storage::<crate::components::Missile>();
    if missiles.count() > MAX_MISSILES - 1 {
      return;
    }
  }

  ecs.create_entity()
    .with(position)
    .with(crate::components::Renderable{
      tex_name: String::from("img/missile.png"),
      i_w: 24,
      i_h: 8,
      o_w: 24,
      o_h: 8,
      frame: 0,
      total_frames: 1,
      rot: 0.0
    })
    .with(crate::components::Missile{
      speed: 6.0
    })
    .build();
}

const MAX_PARTICLES: usize = 64;

fn release_smoke(ecs: &mut World, position: crate::components::Position) {
  {
    let smoke = ecs.read_storage::<crate::components::Smoke>();
    if smoke.count() > MAX_PARTICLES - 1 {
      return;
    }
  }

  let size: u32 = rand::thread_rng().gen_range(16, 52);
  ecs.create_entity()
    .with(position)
    .with(crate::components::Renderable{
      tex_name: String::from("img/circle_small.png"),
      i_w: 16,
      i_h: 16,
      o_w: size,
      o_h: size,
      frame: 0,
      total_frames: 1,
      rot: 0.0
    })
    .with(crate::components::Smoke{
        slack: 1.0 + rand::thread_rng().gen_range(0.005, 0.1),
        speed: rand::thread_rng().gen_range(4.0, 5.0),
        shrink_time: 1.0,
        shrink_speed: rand::thread_rng().gen_range(0.3, 1.0),
        shrink_factor: rand::thread_rng().gen_range(2, 3) as u32
    })
    .build();
}
