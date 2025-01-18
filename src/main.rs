/*
* Tieney
* An asteroids clone built in Rust
*
* (C) CC0 c4tastr0phic
* This project is dependandant on Rust-SDL2, which uses the MIT license, and SDL2, which uses the zlib license
*/

// SDL2 library imports
use sdl2::render::Texture;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, TextureCreator};
use sdl2::rect::{Rect, Point};
use sdl2::video::WindowContext;

// Specs library imports
use specs::{World, WorldExt, Join, DispatcherBuilder};

use std::time::Duration;
use std::path::Path;
use std::collections::HashMap;

pub mod texture_manager;
pub mod utils;
pub mod components;
pub mod game;
pub mod asteroid;
pub mod missile;
pub mod smoke;

const GAME_WIDTH: u32 = 800;
const GAME_HEIGHT: u32 = 600;

fn render(canvas: &mut WindowCanvas, texture_manager: &mut texture_manager::TextureManager<WindowContext>, _texture_creator: &TextureCreator<WindowContext>,ecs: &World) -> Result<(), String> {
    let color = Color::RGB(15,15,10);
    canvas.set_draw_color(color);
    canvas.clear();

    let positions = ecs.read_storage::<components::Position>();
    let renderables = ecs.read_storage::<components::Renderable>();

    for (renderable, pos) in (&renderables, &positions).join() {
      let src = Rect::new(0, 0, renderable.i_w, renderable.i_h);
      let x: i32 = pos.x as i32;
      let y: i32 = pos.y as i32;
      let dest = Rect::new(x - ((renderable.o_w/2) as i32), y - ((renderable.o_h/2) as i32), renderable.o_w, renderable.o_h);

      let center = Point::new((renderable.o_w/2) as i32, (renderable.o_h/2) as i32);
      let texture = texture_manager.load(&renderable.tex_name)?;
      
      canvas.copy_ex(
        &texture,
        src,     // source rect
        dest,    // destination rect
        renderable.rot, // angle
        center,  // center of image
        false,   // flip horizontal
        false    // flip vertical
      )?;
    }
    
    canvas.present();
    Ok(())
}

// Entity component system
struct State { ecs: World }

fn main() -> Result<(), String> {
    println!("Your about to get your pants shit, kid");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to initialize video subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("Failed to initialize canvas");

    let texture_creator = canvas.texture_creator();
    let mut texture_man = texture_manager::TextureManager::new(&texture_creator);

    // Load images before the main loop to not try and load during gameplay
    texture_man.load("img/triangle.png")?;
    texture_man.load("img/square.png")?;
    texture_man.load("img/circle_small.png")?;
    texture_man.load("img/missile.png")?;
    
    let mut event_pump = sdl_context.event_pump()?;
    let mut key_manager: HashMap<String, bool> = HashMap::new();

    let mut gs = State {
      ecs: World::new()
    };
    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Renderable>();
    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Asteroid>();
    gs.ecs.register::<components::Missile>();
    gs.ecs.register::<components::Smoke>();

    let mut dispatcher = DispatcherBuilder::new()
      .with(asteroid::AsteroidMover, "asteroid_mover", &[])
      .with(asteroid::AsteroidCollider, "asteroid_collider", &[])
      .with(missile::MissileMover, "missile_mover", &[])
      .with(smoke::SmokeMover, "smoke_mover", &[])
      .build();
    
    game::load_world(&mut gs.ecs);
    
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                  utils::key_down(&mut key_manager, " ".to_string());
                },
                Event::KeyUp { keycode: Some(Keycode::Space), ..} => {
                  utils::key_up(&mut key_manager, " ".to_string());
                },
                Event::KeyDown { keycode, .. } => {
                  match keycode {
                    None => {},
                    Some(key) => {
                      utils::key_down(&mut key_manager, key.to_string());
                    }
                  }
                },
                Event::KeyUp { keycode, .. } => {
                  match keycode {
                    None => {},
                    Some(key) => {
                      utils::key_up(&mut key_manager, key.to_string());
                    }
                  }
                },
                _ => {}
            }
        }

        game::update(&mut gs.ecs, &mut key_manager);
        dispatcher.dispatch(&gs.ecs);
        gs.ecs.maintain();
        render(&mut canvas, &mut texture_man, &texture_creator, &gs.ecs)?;
        
        // Time management
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));   
    }

    Ok(())
}
