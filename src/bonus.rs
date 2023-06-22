use raylib::prelude::*;
use crate::grid::*;

const HEART_FRAMES:[f32;4] = [48_f32,64_f32,80_f32,96_f32];
const SG_COIN_FRAMES:[f32;4] = [0_f32,16_f32,32_f32,48_f32];
const DIAMOND_FRAMES:[f32;2] = [0_f32,16_f32];
const CASH_FRAMES:[f32;12] = [32_f32,48_f32,64_f32,80_f32,96_f32,112_f32,128_f32,144_f32,160_f32,176_f32,192_f32,208_f32];

const HEART_REC:Rectangle = Rectangle::new(48_f32, 192_f32, TILE_SIZE, TILE_SIZE);
const CASH_REC:Rectangle = Rectangle::new(32_f32,240_f32,TILE_SIZE,TILE_SIZE);
const SILVER_COIN_REC:Rectangle = Rectangle::new(O,224_f32,TILE_SIZE,TILE_SIZE);
const GOLD_COIN_REC:Rectangle = Rectangle::new(O,208_f32,TILE_SIZE,TILE_SIZE);
const DIAMOND_REC:Rectangle = Rectangle::new(O,240_f32,TILE_SIZE,TILE_SIZE);


#[derive(PartialEq,Clone,Debug,Copy)]
pub enum BonusType {
    Default,
    Heart,
    Cash,
    SilverCoin,
    GoldCoin,
    Diamond,
}

pub struct Bonus {
    pub bonus_type:BonusType,
    pub val:usize,
    frame:usize,
    local_frames:Vec<f32>,
    time:f32,
    rec:Rectangle,
   pub rec2:Rectangle,
}

impl Bonus {
    pub fn new(bonus_type:BonusType,i:usize,j:usize,scale:f32) -> Self {
        let frame = 0;
        let mut local_frames = vec![];
        let time = 0_f32;
        let mut val = 0;
        let mut rec = Rectangle::default();
        let scaled_tile = TILE_SIZE*scale;
        let x = (i as f32)*scaled_tile;
        let y = (j as f32)*scaled_tile;
        let rec2 = Rectangle::new(x,y,scaled_tile,scaled_tile);
 
        match bonus_type {
            BonusType::Heart => {rec = HEART_REC; val = 1; local_frames = HEART_FRAMES.to_vec()},
            BonusType::Cash => {rec= CASH_REC; val = 10; local_frames = CASH_FRAMES.to_vec()},
            BonusType::SilverCoin => {rec= SILVER_COIN_REC; val = 20; local_frames = SG_COIN_FRAMES.to_vec()},
            BonusType::GoldCoin => {rec= GOLD_COIN_REC; val = 50; local_frames = SG_COIN_FRAMES.to_vec()},
            BonusType::Diamond => {rec= DIAMOND_REC; val = 100; local_frames = DIAMOND_FRAMES.to_vec()},
            _ => {},
        }

        Self { bonus_type,val, frame,local_frames,time, rec, rec2}
     }

     pub fn draw(&self,sheets:&Texture2D,d:&mut RaylibDrawHandle){
        if self.bonus_type != BonusType::Default{
        d.draw_texture_pro(sheets, self.rec, self.rec2,Vector2::default(),O, Color::WHITE);
        }
     }

     pub fn animate(&mut self,frame_time:f32){

        if self.time > ANIM_DURATION {
           self.frame += 1;
           self.time = 0_f32;
        }
        self.frame %= self.local_frames.len();
        self.rec.x = self.local_frames[self.frame];
        self.time += frame_time;
    }

     pub fn play_audio(&self,audio:&mut RaylibAudio,bonus_sound:&Sound){
        audio.play_sound(bonus_sound);
    }

    pub fn get_position(&self) -> Position {
        let i = ((self.rec2.x + MARGIN_POS)/ SCALED_TILE) as usize;
        let j =  ((self.rec2.y + MARGIN_POS)/ SCALED_TILE) as usize;
        return (i,j);
    }
}


 