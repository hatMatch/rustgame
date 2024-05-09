extern crate sdl2;
use libm::atan2f;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Texture;
use std::cmp;

//declare constants
const PI: f32 = std::f32::consts::PI;

pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    pub pos: [f32; 2],
    pub vel: [f32; 2],
    pub direction: f32,
    pub is_walking: bool,
    pub health: u16,
}

impl Entity {
    pub fn new(name: String, entity_type: EntityType, pos: [f32; 2], health: u16) -> Self {
        Self {
            name,
            entity_type,
            pos,
            vel: [0.0, 0.0],
            direction: 0.0,
            is_walking: false,
            health,
        }
    }

    pub fn set_direction(&mut self, direction: f32) {
        self.direction = direction;
    }

    pub fn walk(&mut self) {
        let max_velocity: f32 = self.entity_type.get_max_walk_vel();
        const acceleration: f32 = 5.0;
        let acc_x: f32 = f32::cos(self.direction) * acceleration;
        let acc_y: f32 = f32::sin(self.direction) * acceleration;
        let vel_x: f32 = if (self.vel[0] + acc_x).abs() >= max_velocity {
            0.1 * (self.vel[0] + acc_x).signum()
        } else {
            self.vel[0] + acc_x
        };
        let vel_y: f32 = if (self.vel[1] + acc_y) >= max_velocity {
            0.1 * (self.vel[0] + acc_x).signum()
        } else {
            self.vel[1] + acc_y
        };

        self.vel = [vel_x, vel_y];
    }

    pub fn update_location(&mut self) {
        let max_vel_loss: [f32; 2] = [self.vel[0].abs(), self.vel[1].abs()];

        if self.is_walking == false {
            self.vel[0] = self.vel[0] - (self.vel[0].signum() * max_vel_loss[0].min(0.2));
            self.vel[1] = self.vel[1] - (self.vel[1].signum() * max_vel_loss[1].min(0.2));
        }
        self.pos = [self.pos[0] + self.vel[0], self.pos[1] + self.vel[1]];
    }
    pub fn get_speed(&self) -> f32 {
        (&self.vel[0].powf(2.0) + &self.vel[1].powf(2.0)).sqrt()
    }

    pub fn get_entity_type(&self) -> i16 {
        match self.entity_type {
            EntityType::Player => 0,
            EntityType::Enemy => 1,
        }
    }
}

pub struct LocationHash {
    pub pos: [f32; 2],
    pub loc_hash: u32,
}

pub fn calc_loc_hash(pos: &[f32; 2]) -> u32 {
    let x: f32 = (pos[0] / 100.0).floor() * 7.0;
    let y: f32 = (pos[1] / 100.0).floor() * 5.0;
    (x + y) as u32
}

impl LocationHash {
    pub fn new(pos: [f32; 2]) -> Self {
        Self {
            pos,
            loc_hash: calc_loc_hash(&pos),
        }
    }
}

pub fn direction_to_target(pos: [f32; 2], target_pos: [f32; 2]) -> f32 {
    let direction = atan2f(target_pos[1] - pos[1], target_pos[0] - pos[0]);
    let converted_direction = 180.0 / PI * &direction;
    println!("{}", converted_direction);
    direction
}

pub enum EntityType {
    Player,
    Enemy,
}

impl EntityType {
    pub fn get_max_walk_vel(&self) -> f32 {
        match self {
            EntityType::Player => 10.0,
            EntityType::Enemy => 5.0,
        }
    }
}

pub trait Mesh {
    fn get_mesh(&self, size: [u32; 2]) -> sdl2::rect::Rect;
}

impl Mesh for Entity {
    fn get_mesh(&self, size: [u32; 2]) -> sdl2::rect::Rect {
        let x: i32 = self.pos[0] as i32;
        let y: i32 = self.pos[1] as i32;
        sdl2::rect::Rect::new(x, y, size[0], size[1])
    }
}
