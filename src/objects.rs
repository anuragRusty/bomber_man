use raylib::{prelude::*};
use rand::Rng;
use crate::grid::{SCALE,TILE_SIZE,MAX_RAND_FRAME,Obj,DObj,FRAMES,O,EXPLOSION,ANIM_DURATION,FLAME_MID_DOWN,FLAME_MID_LEFT,FLAME_MID_RIGHT,FLAME_MID_TOP};

const EMPTY_Y:f32 = 32_f32;
const GRASS_Y:f32 = 112_f32;
const BLOCK_Y:f32 = 16_f32;

pub const MAX_WALL_FRAMES:usize = 7;

#[derive(PartialEq,Clone,Debug,Copy)]
pub enum State{
    IDEAL,
    EXPLOADING,
    EXPLOADED,
}

#[macro_export]
macro_rules! static_obj {
    ($name:ident) => {
        #[derive(PartialEq,Clone,Debug,Copy)]
        pub struct $name{
           pub rec2:Rectangle,
           pub rec:Rectangle,
        }
    };
}

macro_rules! impl_rand_obj {
    ($name:ident,$max:literal,$min:literal,$y:expr) => {
        impl $name{
            pub fn new() -> $name {
                let rec2 =  Rectangle::new(O,O,TILE_SIZE*SCALE,TILE_SIZE*SCALE);
                let mut rng = rand::thread_rng();
                let mut i = rng.gen_range(0..MAX_RAND_FRAME) as usize;
                if i >= $max{i = $min;}
                let rec = Rectangle::new(FRAMES[i],$y,TILE_SIZE,TILE_SIZE);
                Self { rec2, rec}
              }
        }
    };
}


#[macro_export]
macro_rules! impl_set_position {
    ($trt:ident,$name:ident,$fn:ident,$vec_field:ident) => {
        impl $trt for $name {
           fn $fn(&mut self, x: f32, y: f32) {
                self.$vec_field.x = x;
                self.$vec_field.y = y;
            }
        }
    };
}

#[macro_export]
macro_rules! impl_static_draw {
    ($name:ident) => {
        impl $name {
           pub fn draw(&mut self,sheets:&Texture2D,d:&mut RaylibDrawHandle){
                d.draw_texture_pro(sheets, self.rec, self.rec2,Vector2::default(),O, Color::WHITE);
            }
        }
    };

    ($name:ident,$prop:ident,$prop2:ident) => {
        impl $name {
           pub fn draw2(&mut self,sheets:&Texture2D,d:&mut RaylibDrawHandle){
                d.draw_texture_pro(sheets, self.$prop, self.$prop2,Vector2::default(),O, Color::WHITE);
            }
        }
    };
}

#[macro_export]
macro_rules! impl_exp {
    ($obj:ident,$name:ident) => {
        impl $obj {
            pub fn $name(&mut self,i:usize,j:usize,cells:&Vec<Vec<i8>>){
                if self.state == State::IDEAL{
                for &(x, y, mid) in [(-1, 0, FLAME_MID_RIGHT),(1, 0, FLAME_MID_LEFT),(0, -1, FLAME_MID_DOWN),(0, 1, FLAME_MID_TOP)].iter() {
                        let row = (i as isize + x as isize) as usize;
                        let col = (j as isize + y as isize) as usize;
                        let cell = cells[row][col];
                        if cell == EXPLOSION || cell == mid{
                            self.state = State::EXPLOADING;
                            break;
                        }
                     }
                }
            }
        }
    };
}

#[derive(PartialEq,Clone,Debug,Copy)]
pub struct Wall {
    pub rec2:Rectangle,
    pub rec:Rectangle,
    pub frames:usize,
    pub time:f32,
    pub state:State,
}

impl Wall {
    pub fn new() -> Self {
      let rec2 =  Rectangle::new(O, O, TILE_SIZE*SCALE , TILE_SIZE*SCALE);
      let rec =  Rectangle::new(O, O, TILE_SIZE , TILE_SIZE);
      let frames = 0;
      let time = O;
      let state = State::IDEAL;
      Self { rec2, rec, frames, time, state}
    }
}

impl DObj for Wall{
    fn animate(&mut self,frame_time:&f32){
        if self.state == State::EXPLOADING{// Start exploading animation if exploading true.
         if self.time > ANIM_DURATION{
             self.time = 0_f32;
             self.frames += 1;
         }
         self.time += *frame_time;
         self.rec.x = FRAMES[self.frames];
 
         if self.frames == 6 { //In the last frame set exploaded true.
           self.state = State::EXPLOADED;
         }
         self.frames = self.frames  % (MAX_WALL_FRAMES - 1);
      }else{ //Or set frame to default.
         self.frames = 0;
         self.rec.x = FRAMES[self.frames];
      }
     }
}

impl_set_position!(Obj,Wall,set_position,rec2);
impl_static_draw!(Wall);
impl_exp!(Wall,expload_wall);

static_obj!(Empty);
static_obj!(Block);
static_obj!(Grass);

impl_set_position!(Obj,Block,set_position,rec2);
impl_set_position!(Obj,Empty,set_position,rec2);
impl_set_position!(Obj,Grass,set_position,rec2);

impl_rand_obj!(Block,4,0,BLOCK_Y);
impl_rand_obj!(Grass,8,7,GRASS_Y);
impl_rand_obj!(Empty,4,0,EMPTY_Y);

impl_static_draw!(Grass);
impl_static_draw!(Empty);
impl_static_draw!(Block);
  