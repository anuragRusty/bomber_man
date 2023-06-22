use raylib::prelude::*;
use crate::grid::*;
use crate::game::*;

const BLACK_BOMB:Rectangle = Rectangle::new(FRAMES[4],96_f32,TILE_SIZE,TILE_SIZE);
const BLUE_BOMB:Rectangle = Rectangle::new(FRAMES[7],96_f32,TILE_SIZE,TILE_SIZE);
const PURPLE_BOMB:Rectangle = Rectangle::new(FRAMES[10],96_f32,TILE_SIZE,TILE_SIZE);
const RED_BOMB:Rectangle = Rectangle::new(FRAMES[13],96_f32,TILE_SIZE,TILE_SIZE);

#[derive(PartialEq,Clone,Debug,Copy)]
pub enum UpgradeType {
  Default,
  BlackBomb,
  BlueBomb,
  PurpleBomb,
  RedBomb,
}

pub struct Upgrade {
    pub up_type:UpgradeType,
    pub val:usize,
    rec2:Rectangle,
}

impl Upgrade { 

   pub fn new(up_type:UpgradeType,val:usize,i:usize,j:usize,scale:f32) -> Self {
       let scaled_tile = TILE_SIZE*scale;
       let x = (i as f32)*scaled_tile;
       let y = (j as f32)*scaled_tile;
       let rec2 = Rectangle::new(x,y,scaled_tile,scaled_tile);
       return Self{up_type,val,rec2};
    }

  pub fn draw_val(&self,d:&mut RaylibDrawHandle){
      let x = (self.rec2.x as i32) + TEXT_SIZE/3;
      let y = (self.rec2.y as i32) + TEXT_SIZE/3;
      let val_str = format!("x{}",self.val);

      if self.up_type != UpgradeType::Default {
      d.draw_text(&val_str, x, y, TEXT_SIZE/3, Color::WHITE);
     }
  }

   pub fn draw(&self,sheets:&Texture2D,d:&mut RaylibDrawHandle){
       match self.up_type {
        UpgradeType::BlackBomb => {d.draw_texture_pro(sheets,BLACK_BOMB,self.rec2,Vector2::default(),O,Color::WHITE)},
        UpgradeType::BlueBomb => {d.draw_texture_pro(sheets,BLUE_BOMB,self.rec2,Vector2::default(),O,Color::WHITE)},
        UpgradeType::PurpleBomb => {d.draw_texture_pro(sheets,PURPLE_BOMB,self.rec2,Vector2::default(),O,Color::WHITE)},
        UpgradeType::RedBomb => {d.draw_texture_pro(sheets,RED_BOMB,self.rec2,Vector2::default(),O,Color::WHITE)},
         _ => {}
       }
       self.draw_val(d);
    }

   pub fn play_audio(&self,audio:&mut RaylibAudio,upgrade_sound:&Sound){
       audio.play_sound(upgrade_sound);
   }

   pub fn get_position(&self) -> Position {
    let i = ((self.rec2.x + MARGIN_POS)/ SCALED_TILE) as usize;
    let j =  ((self.rec2.y + MARGIN_POS)/ SCALED_TILE) as usize;
    return (i,j);
  }
}
