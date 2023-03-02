use raylib::prelude::*;
use crate::grid::{TILE_SIZE, MAX_FRAME,SCALE,ANIM_DURATION,State};

pub const EXPLOSION_TILE:f32 = 240_f32;
pub const EXPLOSION_FRAMES:i32 = 4;
pub const EXPLOSION_MARGIN:f32 = 24_f32;



pub struct Bomb {
    pub vec2:Vector2,
    pub exp_vec2:Vector2,
    pub rec:Rectangle,
    pub exp_rec:Rectangle,
    pub exp_frames:i32,
    pub frames:i32,
    pub time:f32,
    pub exp_time:f32,
    pub exploading_time:f32,
    pub state:State,
}

impl Bomb {
    pub fn new() -> Self{
        let vec2 = Vector2::new(0_f32,0_f32);
        let rec = Rectangle::new(0_f32,0_f32,(TILE_SIZE*SCALE) as f32,(TILE_SIZE*SCALE) as f32);
        let frames = 0;
        let time = 0_f32;
        let exp_time = 0_f32;
        let exploading_time = 2_f32;
        let exp_frames = 0;
        let exp_vec2 = Vector2::new(0_f32,0_f32);
        let exp_rec = Rectangle::new(0_f32,0_f32,EXPLOSION_TILE as f32,EXPLOSION_TILE as f32);
        let state = State::IDEAL;
        Self { vec2,exp_vec2, rec,exp_rec, exp_frames, frames, time,exp_time,exploading_time,state}
    }

    pub fn set_position(&mut self,x:i32,y:i32){
        self.vec2.x = x as f32;
        self.vec2.y = y as f32;
     }

     pub fn set_position_exp(&mut self,x:f32,y:f32){
        self.exp_vec2.x = x;
        self.exp_vec2.y = y;
     }

    pub fn set_frame(&mut self){
        self.rec.x = (TILE_SIZE * SCALE) as f32 * self.frames as f32;
     }

     pub fn set_frame_exp(&mut self){
        self.exp_rec.x = EXPLOSION_TILE * self.exp_frames as f32;
     }

    pub fn animate(&mut self,frame_time:&f32){
        if self.state == State::IDEAL{
        if self.time > ANIM_DURATION{
            self.time = 0_f32;
            self.frames += 1;
        }
        self.time += *frame_time;
        self.set_frame();
        self.frames = self.frames  % (MAX_FRAME - 1);
        
        self.exploading_time -= *frame_time;
     }else{
        if self.exp_time > ANIM_DURATION{
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

     match self.state {
        State::EXPLOADED => {return}
        State::IDEAL => { if self.exploading_time < 0_f32 {self.state = State::EXPLOADING;}}
        State::EXPLOADING => { if self.exploading_time < -1_f32{self.state = State::EXPLOADED;}}
     }
        
    }

}

