use raylib::{prelude::*};
use rand::Rng;
use crate::bomb::*;
use crate::grid::{SCALED_TILE,TILE_SIZE,MAX_RAND_FRAME,FRAMES,O,ANIM_DURATION};

const EMPTY_Y:f32 = 32_f32;
const GRASS_Y:f32 = 112_f32;
const BLOCK_Y:f32 = 16_f32;

pub const MAX_WALL_FRAMES:usize = 7;

#[derive(PartialEq,Clone,Debug,Copy)]
pub enum State {
    IDEAL,
    EXPLOADING,
    EXPLOADED,
}

#[macro_export]
macro_rules! static_obj {
    ($name:ident) => {
        #[derive(PartialEq,Clone,Debug,Copy)]
        pub struct $name{
           pub rec:Rectangle,
           pub rec2:Rectangle,
        }
    };
}

macro_rules! impl_rand_obj {
    ($name:ident,$max:literal,$min:literal,$y:expr) => {
        impl $name{
            pub fn new(i:usize,j:usize,scale:f32) -> $name {
                let scaled_tile = TILE_SIZE*scale;
                let x = (i as f32) * scaled_tile;
                let y = (j as f32) * scaled_tile;
                let rec2 =  Rectangle::new(x,y,SCALED_TILE,SCALED_TILE);
                let mut rng = rand::thread_rng();
                let mut i = rng.gen_range(0..MAX_RAND_FRAME) as usize;
                if i >= $max{i = $min;}
                let rec = Rectangle::new(FRAMES[i],$y,TILE_SIZE,TILE_SIZE);
                Self { rec, rec2}
              }
        }
    };
}


#[macro_export]
macro_rules! impl_set_position {
    ($name:ident,$fn:ident,$vec_field:ident,$scale:expr) => {
        impl $name {
          pub fn $fn(&mut self, i:usize, j: usize) {
                self.$vec_field.x = i as f32 * $scale;
                self.$vec_field.y = j as f32 * $scale;
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
    pub frame:usize,
    pub time:f32,
    pub state:State,
}

impl Wall {
    pub fn new(i:usize,j:usize,scale:f32) -> Self {
      let scaled_tile = TILE_SIZE*scale;
      let x = (i as f32) * scaled_tile;
      let y = (j as f32) * scaled_tile;
      let rec2 =  Rectangle::new(x, y, scaled_tile , scaled_tile);
      let rec =  Rectangle::new(O, O, TILE_SIZE , TILE_SIZE);
      let frame = 0;
      let time = O;
      let state = State::IDEAL;
      Self { rec2, rec, frame, time, state}
    }

    pub fn update(&mut self,frame_time:f32){
        match self.state {
            State::EXPLOADING => {
                self.update_state();
                self.animate(frame_time);
            }
            _ => {}
        }
    }

    fn update_state(&mut self){
        if self.frame >= 6 { // On the last frame set the state exploaded
          self.state = State::EXPLOADED;
        }
    }

    fn animate(&mut self,frame_time:f32){
        if self.time > ANIM_DURATION{
            self.time = 0_f32;
            self.frame += 1;
        }
        self.time += frame_time;
        self.rec.x = FRAMES[self.frame];
        self.frame = self.frame  % MAX_WALL_FRAMES;
    }
}

impl_static_draw!(Wall);
impl_exp!(Wall,exploade);

static_obj!(Empty);
static_obj!(Block);
static_obj!(Grass);

impl_set_position!(Empty,set_position,rec2,SCALED_TILE);

impl_rand_obj!(Block,4,0,BLOCK_Y);
impl_rand_obj!(Grass,8,7,GRASS_Y);
impl_rand_obj!(Empty,4,0,EMPTY_Y);

impl_static_draw!(Grass);
impl_static_draw!(Empty);
impl_static_draw!(Block);
  