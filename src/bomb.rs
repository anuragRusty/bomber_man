use raylib::prelude::*;
use crate::grid::{TILE_SIZE, MAX_FRAME};

pub const BOMB_SCALE:i32 = 2;

pub const EXPLOSION_TILE:f32 = 200_f32;
pub const EXPLOSION_FRAMES:i32 = 4;
pub const EXPLOSION_MARGIN:i32 = 72;
pub struct Bomb {
    pub vec2:Vector2,
    pub exp_vec2:Vector2,
    pub rec:Rectangle,
    pub exp_rec:Rectangle,
    pub exp_frames:i32,
    pub frames:i32,
    pub duration:f32,
    pub time:f32,
    pub exp_tile:f32,
    pub exp_time:f32,
    pub exploading_time:f32,
    pub exploading:bool,
    pub exploaded:bool,
    pub time_to_exploade:f32,
}

impl Bomb {
    pub fn new() -> Self{
        let vec2 = Vector2::new(0_f32,0_f32);
        let rec = Rectangle::new(0_f32,0_f32,(TILE_SIZE*BOMB_SCALE) as f32,(TILE_SIZE*BOMB_SCALE) as f32);
        let frames = 0;
        let duration = 0.2_f32;
        let time = 0_f32;
        let exp_tile = EXPLOSION_TILE;
        let exp_time = 0_f32;
        let exploading_time = 2_f32;
        let time_to_exploade = 0_f32;
        let exploading = false;
        let exploaded = false;
        let exp_frames = 0;
        let exp_vec2 = Vector2::new(0_f32,0_f32);
        let exp_rec = Rectangle::new(0_f32,0_f32,EXPLOSION_TILE as f32,EXPLOSION_TILE as f32);
        Self { vec2,exp_vec2, rec,exp_rec, exp_frames, frames, duration, time,exp_tile,exp_time,exploading_time,exploading,exploaded ,time_to_exploade }
    }

    pub fn set_position(&mut self,x:i32,y:i32){
        self.vec2.x = x as f32;
        self.vec2.y = y as f32;
     }

     pub fn set_position_exp(&mut self,x:i32,y:i32){
        self.exp_vec2.x = x as f32;
        self.exp_vec2.y = y as f32;
     }

    pub fn set_frame(&mut self){
        self.rec.x = (TILE_SIZE * BOMB_SCALE) as f32 * self.frames as f32;
     }

     pub fn set_frame_exp(&mut self){
        self.exp_rec.x = (self.exp_tile) as f32 * self.exp_frames as f32;
     }

    pub fn animate(&mut self,frame_time:&f32){
    if !self.exploading{
        if self.time > self.duration{
            self.time = 0_f32;
            self.frames += 1;
        }
        self.time += *frame_time;
        self.set_frame();
        self.frames = self.frames  % (MAX_FRAME - 1);
        
        self.exploading_time -= *frame_time;
     }else{
        if self.exp_time > self.duration{
            self.exp_time = 0_f32;
            self.exp_frames += 1;
        }
        self.exp_time += *frame_time;
        self.set_frame_exp();
        self.exp_frames = self.exp_frames  % (EXPLOSION_FRAMES - 1);
        self.exploading_time -= *frame_time;
     }
     self.explode();
     
    }

    pub fn explode(&mut self){

        if self.exploaded {
            return;
        }
        if self.exploading_time < 0_f32 {
            self.exploading = true;
        }
        if self.exploading_time < -1_f32{
            self.exploaded = true;
        }
    }

}

