extern crate sdl2;

mod entity;
mod sprite;

use crate::entity::direction_to_target;
use crate::entity::Entity;
use crate::entity::EntityType;
use crate::entity::LocationHash;
use crate::entity::Mesh;
use crate::sprite::Sprite;
use libm::atan2f;
use rand::Rng;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use sdl2::pixels::Color;
use std::collections::HashSet;
use std::time::Duration;
use std::{thread, time};

const PI: f32 = std::f32::consts::PI;

struct Game<'running, T> {
    player: Entity,
    entities: Vec<Entity>,
    phantom: std::marker::PhantomData<&'running T>,
}

const TICKRATE: time::Duration = time::Duration::from_millis(1000);

impl<'running, T> Game<'running, T> {
    fn update_entities(&mut self) {
        for entity in &mut self.entities {
            entity.update_location();
        }
        self.player.update_location();
    }
    fn new(player: Entity, entities: Vec<Entity>) -> Self {
        Self {
            player,
            entities,
            phantom: std::marker::PhantomData,
        }
    }
    fn check_walking(&mut self, e: &sdl2::EventPump) {
        let keyboard_state = e.keyboard_state();
        let scancodes = keyboard_state.scancodes();
        let mut is_walking: bool = false;
        for (scancode, is_pressed) in scancodes {
            if (scancode == Scancode::W
                || scancode == Scancode::A
                || scancode == Scancode::S
                || scancode == Scancode::D)
                & is_pressed
                == true
            {
                is_walking = true;
                println!("I should walk {}", self.player.is_walking);
            }
        }
        self.player.is_walking = is_walking;
    }
    fn game_loop(&mut self) {
        // Graphics
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window("Matt's Game", 800, 600)
            .position_centered()
            .build()
            .unwrap();
        let mut canvas = window.into_canvas().build().unwrap();
        let mut event_pump = sdl_context.event_pump().unwrap();
        let mut locations: Vec<LocationHash> = vec![];
        for entity in &mut self.entities {
            let loc_hash = LocationHash::new(entity.pos);
            locations.push(loc_hash);
        }
        'running: loop {
            for (i, entity) in &mut self.entities.iter().enumerate() {
                let loc_hash = LocationHash::new(entity.pos);
                println!("Hash_loc {} for {}", &loc_hash.loc_hash, entity.name);
                locations[i] = loc_hash;
            }

            canvas.set_draw_color(Color::RGB(0, 0, 0));
            canvas.clear();
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'running,
                    Event::KeyDown {
                        keycode: Some(keycode),
                        repeat: false,
                        ..
                    } => match keycode {
                        Keycode::W => {
                            self.player.set_direction(-PI / 2.0);
                            self.player.walk();
                        }
                        Keycode::S => {
                            self.player.set_direction(PI / 2.0);
                            self.player.walk();
                        }
                        Keycode::D => {
                            self.player.set_direction(0.0);
                            self.player.walk();
                        }
                        Keycode::A => {
                            self.player.set_direction(PI);
                            self.player.walk();
                        }
                        _ => {}
                    },
                    _ => {}
                }
            }
            self.check_walking(&event_pump);
            println!("speed: {}", &self.player.get_speed());
            // The rest of the game loop goes here...
            canvas.set_draw_color(Color::RGB(0, 200, 0));
            canvas.fill_rect(self.player.get_mesh([50, 50])).unwrap();
            self.update_entities();
            for entity in &mut self.entities {
                entity.set_direction(direction_to_target(entity.pos, self.player.pos));
                entity.walk();
                canvas.set_draw_color(Color::RGB(200, 0, 0));
                canvas.fill_rect(entity.get_mesh([50, 50])).unwrap();
            }
            canvas.present();
            ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}

fn main() {
    let mut player = Entity::new(
        String::from("player"),
        EntityType::Player,
        [400.0, 400.0],
        10,
    );
    let mut enemy1 = Entity::new(String::from("enemy"), EntityType::Enemy, [10.0, 10.0], 10);
    let mut enemy2 = Entity::new(String::from("enemy"), EntityType::Enemy, [100.0, 10.0], 10);
    let entities: Vec<Entity> = vec![enemy1, enemy2];
    //    let mut <T> Game = Game<'_, T> {
    //        entities: entities,
    //        phantom: std::marker::PhantomData<&'_ T>,
    //    };
    let mut game: Game<char> = Game::new(player, entities);
    game.game_loop();
}
