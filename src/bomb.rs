use raylib::prelude::*;
use crate::{grid::{TILE_SIZE, MAX_FRAME,ANIM_DURATION,O,FRAMES,EXPLOSION,FLAME_MID_DOWN,FLAME_MID_LEFT,FLAME_MID_RIGHT,FLAME_MID_TOP, SCALED_TILE},impl_set_position,impl_static_draw};
use crate::{impl_exp};
use crate::objects::{State};

const BOMB_Y:f32 = 96_f32;
const EXP_TIME:f32 = 7_f32;
const EXP_TD_FRAMES:[f32;4] = [32_f32,112_f32,192_f32,272_f32];

const F_LEFT_END_FRAMES:[f32;4] = [0_f32,80_f32,160_f32,240_f32];
const F_RIGHT_END_FRAMES:[f32;4] = [64_f32,144_f32,224_f32,304_f32];

const F_LEFT_MID_FRAMES:[f32;4] = [16_f32,96_f32,176_f32,256_f32];
const F_RIGHT_MID_FRAMES:[f32;4] = [48_f32,128_f32,208_f32,288_f32];

const F_TOP_END_Y:f32 = 128_f32;
const F_TOP_MID_Y:f32 = 144_f32;
const F_DOWN_MID_Y:f32 = 176_f32;  
const F_DOWN_END_Y:f32 = 192_f32;
const EXP_FLAME_LR_Y:f32 = 160_f32;

macro_rules! flame_obj {
    ($name:ident) => {
      #[derive(PartialEq,Clone,Debug,Copy)]
       pub struct $name {
         pub rec:Rectangle,
         pub rec2:Rectangle,
         pub frames:usize,
         pub time:f32,
         pub state:State,
        }
    };
}

macro_rules! impl_new {
    ($name:ident,$frames_arr:expr,$y:expr) => {
        impl $name {
            pub fn new() -> Self{
                let frames = 0;
                let time = 0_f32;
                let state = State::IDEAL;
                let rec = Rectangle::new($frames_arr[frames],$y,TILE_SIZE,TILE_SIZE);
                let rec2 = Rectangle::new(O,O,SCALED_TILE,SCALED_TILE);
                Self{rec,rec2,frames,time,state}
            }
        }
    };
}

macro_rules! impl_draw_anim {
    ($name:ident,$fn_name:ident,$fn_name1:ident,$frames:ident,$rec:ident,$rec2:ident,$time:ident,$frame_arr:expr) => {
        
impl $name {

    pub fn $fn_name(&mut self,texture:&Texture2D,d:&mut RaylibDrawHandle,i:usize,j:usize){
        self.$rec2.x = i as f32 * SCALED_TILE;
        self.$rec2.y = j as f32 * SCALED_TILE;
        d.draw_texture_pro(texture, self.$rec, self.$rec2, Vector2::default(),O, Color::WHITE);  
    }

    pub fn $fn_name1(&mut self,frame_time:&f32){
        if self.$time > ANIM_DURATION {
            self.$time = 0_f32;
            self.$frames += 1;
        }
        self.$time += *frame_time;
        self.$frames %= $frame_arr.len();
        self.$rec.x = $frame_arr[self.$frames];
      }
     }
    };
}

//Flame Objects
flame_obj!(FlameLeftEnd);
flame_obj!(FlameRightEnd);
flame_obj!(FlameTopEnd);
flame_obj!(FlameDownEnd);

flame_obj!(FlameLeftMid);
flame_obj!(FlameRightMid);
flame_obj!(FlameTopMid);
flame_obj!(FlameDownMid);

impl_new!(FlameLeftEnd,F_LEFT_END_FRAMES,EXP_FLAME_LR_Y);
impl_new!(FlameRightEnd,F_RIGHT_END_FRAMES,EXP_FLAME_LR_Y);
impl_new!(FlameTopEnd,EXP_TD_FRAMES,F_TOP_END_Y);
impl_new!(FlameDownEnd,EXP_TD_FRAMES,F_DOWN_END_Y);

impl_new!(FlameLeftMid,F_LEFT_MID_FRAMES,EXP_FLAME_LR_Y);
impl_new!(FlameRightMid,F_RIGHT_MID_FRAMES,EXP_FLAME_LR_Y);
impl_new!(FlameTopMid,EXP_TD_FRAMES,F_TOP_MID_Y);
impl_new!(FlameDownMid,EXP_TD_FRAMES,F_DOWN_MID_Y);

impl_draw_anim!(FlameLeftEnd,draw,anim,frames,rec,rec2,time,F_LEFT_END_FRAMES);
impl_draw_anim!(FlameRightEnd,draw,anim,frames,rec,rec2,time,F_RIGHT_END_FRAMES);
impl_draw_anim!(FlameTopEnd,draw,anim,frames,rec,rec2,time,EXP_TD_FRAMES);
impl_draw_anim!(FlameDownEnd,draw,anim,frames,rec,rec2,time,EXP_TD_FRAMES);

impl_draw_anim!(FlameLeftMid,draw,anim,frames,rec,rec2,time,F_LEFT_MID_FRAMES);
impl_draw_anim!(FlameRightMid,draw,anim,frames,rec,rec2,time,F_RIGHT_MID_FRAMES);
impl_draw_anim!(FlameTopMid,draw,anim,frames,rec,rec2,time,EXP_TD_FRAMES);
impl_draw_anim!(FlameDownMid,draw,anim,frames,rec,rec2,time,EXP_TD_FRAMES);

impl_set_position!(Bomb,set_position,rec2,SCALED_TILE);
impl_static_draw!(Bomb);
impl_draw_anim!(Bomb,draw_exp,anim_exp,exp_frames,exp_rec,exp_rec2,exp_time,EXP_TD_FRAMES);
impl_exp!(Bomb,chain_exp);

#[derive(PartialEq,Clone,Debug,Copy)]
pub struct Bomb {
    pub rec2:Rectangle,
    pub exp_rec2:Rectangle,
    pub rec:Rectangle,
    pub exp_rec:Rectangle,
    pub exp_frames:usize,
    pub frames:usize,
    pub time:f32,
    pub exp_time:f32,
    pub exploading_time:f32,
    pub power:usize,
    pub state:State,
}

impl Bomb {
    pub fn new() -> Self{
        let rec2 = Rectangle::new(O,O,SCALED_TILE,SCALED_TILE);
        let rec = Rectangle::new(O,BOMB_Y,TILE_SIZE,TILE_SIZE);
        let frames = 0;
        let time = 0_f32;
        let exp_time = 0_f32;
        let exploading_time = EXP_TIME;
        let exp_frames = 0;
        let exp_rec2 = Rectangle::new(O,O,SCALED_TILE,SCALED_TILE);
        let exp_rec = Rectangle::new(EXP_TD_FRAMES[exp_frames],EXP_FLAME_LR_Y,TILE_SIZE,TILE_SIZE);
        let power = 7;
        let state = State::IDEAL;
        Self { rec2,exp_rec2, rec,exp_rec, exp_frames, frames, time,exp_time,exploading_time,power,state}
    }
  
    pub fn explode(&mut self){
     match self.state {
        State::EXPLOADED => {return}
        State::IDEAL => { if self.exploading_time <= 0_f32 {self.state = State::EXPLOADING;}}
        State::EXPLOADING => { if self.exploading_time <= -1_f32{self.state = State::EXPLOADED;}}
     }  
    }

    pub fn play_sound(&self,audio:&mut RaylibAudio,sound:&Sound){
       if self.state == State::EXPLOADING {
         audio.play_sound(sound);
       }
    }

    pub fn detonating(&mut self,frame_time:&f32){
      self.exploading_time -= *frame_time;
    }

    fn animate(&mut self,frame_time:&f32){
        if self.state == State::IDEAL{
        if self.time > (EXP_TIME/MAX_FRAME as f32){
            self.time = 0_f32;
            self.frames += 1;
        }
        self.time += *frame_time;
        self.rec.x = FRAMES[self.frames];
        self.frames = self.frames  % MAX_FRAME;
     }
    }

    pub fn update(&mut self,frame_time:&f32,audio:&mut RaylibAudio,sound:&Sound){
        self.play_sound(audio, sound);
        self.animate(frame_time);
        self.detonating(frame_time);
        self.explode();
    }
}
