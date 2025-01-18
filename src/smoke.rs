use specs::{System, Join};
use specs::prelude::*;

pub struct SmokeMover;

impl<'a>System<'a> for SmokeMover {
  type SystemData = (
    WriteStorage<'a, crate::components::Position>,
    WriteStorage<'a, crate::components::Renderable>,
    WriteStorage<'a, crate::components::Smoke>,
    Entities<'a>
  );

  fn run(&mut self, data: Self::SystemData) {
    let (mut position, mut renderables, mut smokes, entities) = data;
    
    for (pos, rend, smoke, entity) in (&mut position, &mut renderables, &mut smokes, &entities).join() {
      let radians = pos.rot.to_radians();

      pos.x -= smoke.speed * radians.sin();
      pos.y += smoke.speed * radians.cos();
      
      if smoke.shrink_time > smoke.shrink_speed {
        smoke.shrink_time -= smoke.shrink_speed;
      } else {
        if rend.o_w > smoke.shrink_factor
        && rend.o_h > smoke.shrink_factor {
          rend.o_w -= smoke.shrink_factor;
          rend.o_h -= smoke.shrink_factor;
        } else { entities.delete(entity).ok(); }
        smoke.shrink_time = 1.0;
      }

      smoke.speed /= smoke.slack;
    }
  }
}
