use raylib::prelude::*;
use crate::KeyboardKey::*;
use crate::bonus::*;
use crate::player::*;
use crate::grid::*;

const BACKGROUND_COLOR:Color = Color::new(28, 52, 112, 255); 
const BLUR_WHITE:Color = Color::new(255,255,255,70);
const GO_FRAMES:&[f32;2] = &[0_f32,144_f32];
const GO_WIDTH:f32 = GO_FRAMES[1];
const GO_Y:&[f32;1]= &[288_f32];

const CD_Y:&[f32;2] = &[256_f32,272_f32];
const CD_FRAMES:&[f32;5] = &[0_f32,32_f32,64_f32,108_f32,180_f32];
const CD_WIDTH:f32 = 48_f32;

const PAUSED_Y:&[f32;1] = &[304_f32];
const P_FRAMES:&[f32;2] = &[0_f32,112_f32];
const P_WIDTH:f32 = P_FRAMES[1];

pub const TEXT_SIZE:i32 = (SCALED_TILE + SCALED_TILE/4_f32) as i32;

#[macro_export]
macro_rules! anim_obj {
    ($name:ident) => {
      #[derive(PartialEq,Clone,Debug,Copy)]
       pub struct $name {
         pub rec:Rectangle,
         pub rec2:Rectangle,
         pub frames:usize,
         pub time:f32,
        }
    };
}

macro_rules! impl_text_obj {
    ($name:ident,$y:expr,$w:expr,$frames:expr,$dur:expr) => {
        impl $name {
          fn new(w:i32,h:i32) -> Self{
            let frames = 0;
            let rec = Rectangle::new(GO_FRAMES[frames],$y[frames],$w,TILE_SIZE);
            let x = (w as f32 /2.0) - ($w*SCALE/2.0);
            let y = (h as f32 /2.0) - ((SCALED_TILE/2.0));
            let rec2 = Rectangle::new(x,y,$w*SCALE,SCALED_TILE);
            let time = 0_f32;
            Self { rec, rec2, frames, time}
           }
        
            fn draw_animate(&mut self,d:&mut RaylibDrawHandle,texts:&Texture2D,frame_time:f32){
              d.draw_texture_pro(texts, self.rec,self.rec2, Vector2::default(), O, Color::WHITE);

              if self.time > $dur{
                self.time  = 0_f32;
                self.frames += 1;
              }

              self.frames %= $frames.len();
              self.time += frame_time;
              self.rec.x = $frames[self.frames];
              self.rec.y = $y[self.frames%$y.len()];
            }
        }
    };
}

anim_obj!(GameOver);
anim_obj!(Paused);
anim_obj!(CountDown);

impl_text_obj!(GameOver,GO_Y,GO_WIDTH,GO_FRAMES,ANIM_DURATION);
impl_text_obj!(Paused,PAUSED_Y,P_WIDTH,P_FRAMES,ANIM_DURATION);
impl_text_obj!(CountDown,CD_Y,CD_WIDTH,CD_FRAMES,0.35);

#[derive(PartialEq,Clone)]
pub enum GameState {
    STARTING,
    RUNNING,
    PAUSED,
    GAMEOVER,
}

pub struct Game {
    pub state:GameState,
    pub menu_enable:bool,
    pub screen_w:i32,
    pub screen_h:i32,
    pub player:Player,
    pub grid:Grid,
    pub heart:Bonus,
    pub cash:Bonus,
    pub silver_coin:Bonus,
    pub gold_coin:Bonus,
    pub diamond:Bonus,
    game_over_text:GameOver,
    paused_text:Paused,
    count_down:CountDown,
    frames:usize,
    time:f32,
}

impl Game {
 pub fn new() -> Self{
     let state = GameState::STARTING;
     let menu_enable = false;
     let player = Player::new();
     let grid = Grid::new();
     let i = grid.cells.len();
     let j = grid.cells[0].len() + 1;
     let screen_w = SCALED_TILE as i32 * i as i32;
     let screen_h = SCALED_TILE as i32 * j as i32;
   
     let heart = Bonus::new(BonusType::Heart, i-2, j-1, SCALE);
     let cash = Bonus::new(BonusType::Cash, 4, j-1, SCALE);
     let silver_coin = Bonus::new(BonusType::SilverCoin, 6, j-1, SCALE);
     let gold_coin = Bonus::new(BonusType::GoldCoin, 8, j-1, SCALE);
     let diamond = Bonus::new(BonusType::Diamond, 10, j-1, SCALE);

     let game_over_text = GameOver::new(screen_w,screen_h);
     let paused_text = Paused::new(screen_w,screen_h);
     let count_down = CountDown::new(screen_w,screen_h);
     let frames = 0;
     let time = 0_f32;
     Self { state,menu_enable,screen_w,screen_h, player,heart,cash,silver_coin,gold_coin,diamond,grid,game_over_text,paused_text,count_down,frames,time}
   }

 fn anim_count_down(&mut self,frame_time:f32){
  if self.time > ANIM_DURATION{
    self.frames += 1;
    self.time = 0_f32;
  }
  self.time += frame_time;
  self.frames %= CD_Y.len();
  self.count_down.rec.y = CD_Y[self.frames];
}  

 pub fn update_icons(&mut self,frame_time:f32){
     self.heart.animate(frame_time);
     self.cash.animate(frame_time);
     self.silver_coin.animate(frame_time);
     self.gold_coin.animate(frame_time);
     self.diamond.animate(frame_time);
 }

pub fn update_game_state(&mut self){
   if self.state != GameState::PAUSED {
    match self.player.state{
    State2::SPAWN => {self.state = GameState::STARTING}
    State2::ALIVE => {self.state = GameState::RUNNING}
    State2::DEAD => {self.state = GameState::GAMEOVER}
    _ => {}
    }
   }
    if self.count_down.frames > MAX_FRAME && self.state == GameState::STARTING {
       self.state = GameState::RUNNING;
       self.player.state = State2::SPAWNING;
    }
}

pub fn handle_game_state(&mut self,rl:&mut RaylibHandle){
      if rl.is_key_pressed(KEY_P) && self.state != GameState::GAMEOVER{
       self.state = if self.state == GameState::PAUSED{GameState::STARTING}else{GameState::PAUSED};
      }else if rl.is_key_down(KEY_ESCAPE) {
        self.menu_enable = !self.menu_enable;
      }else  if rl.is_key_pressed(KEY_R){
        self.grid = Grid::new();
        self.player = Player::new();
        self.count_down.frames = 0;
     }
    }

   pub fn draw_game_state(&mut self,d:&mut RaylibDrawHandle,texts:&Texture2D,frame_time:f32){
      match self.state{
        GameState::GAMEOVER => {self.game_over_text.draw_animate(d, texts,frame_time)}
        GameState::PAUSED => {self.paused_text.draw_animate(d, texts, frame_time)}
        GameState::STARTING => {self.count_down.draw_animate(d, texts, frame_time); self.anim_count_down(frame_time)}
      _ => {} 
   
   }  
  }

  pub fn draw_blur(&self,d:&mut RaylibDrawHandle){
      d.draw_rectangle(0, 0, self.screen_w,self.screen_h-(SCALED_TILE as i32), BLUR_WHITE)
  }

  pub fn draw_score(&self,d:&mut RaylibDrawHandle){
      let mut zeroes = "0000".to_string();
      let score = self.player.score.to_string();
      for _i in 1..score.len(){zeroes.pop();}
      let y = self.screen_h - TEXT_SIZE + TEXT_SIZE/6;
      let score_str = format!("{}{}",zeroes,score);
      d.draw_text(&score_str, 0,y, TEXT_SIZE,Color::WHITE);
  }

pub fn draw_icons(&mut self,d:&mut RaylibDrawHandle,sheets:&Texture2D){
  self.heart.draw(sheets, d);
  self.cash.draw(sheets, d);
  self.silver_coin.draw(sheets, d);
  self.gold_coin.draw(sheets, d);
  self.diamond.draw(sheets, d);
}

pub fn draw_text(&mut self,d:&mut RaylibDrawHandle){
  let life_str = format!(" x{}",self.player.lifes);
  let x = (self.heart.rec2.x + SCALED_TILE/2_f32) as i32;
  let y = (self.heart.rec2.y + SCALED_TILE/2_f32) as i32;
  d.draw_text(&life_str, x, y,TEXT_SIZE/2, Color::WHITE);
  let cash_str = format!(" x{}",self.player.cash);
  let x1 = (self.cash.rec2.x + SCALED_TILE/2_f32) as i32;
  let y1 = (self.cash.rec2.y + SCALED_TILE/2_f32) as i32;
  d.draw_text(&cash_str, x1, y1,TEXT_SIZE/2, Color::WHITE);
  let silver_str = format!(" x{}",self.player.silver_coin);
  let x2 = (self.silver_coin.rec2.x + SCALED_TILE/2_f32) as i32;
  let y2 = (self.silver_coin.rec2.y + SCALED_TILE/2_f32) as i32;
  d.draw_text(&silver_str, x2, y2,TEXT_SIZE/2, Color::WHITE);
  let gold_str = format!(" x{}",self.player.gold_coin);
  let x3 = (self.gold_coin.rec2.x + SCALED_TILE/2_f32) as i32;
  let y3 = (self.gold_coin.rec2.y + SCALED_TILE/2_f32) as i32;
  d.draw_text(&gold_str, x3, y3,TEXT_SIZE/2, Color::WHITE);
  let diamond_str = format!(" x{}",self.player.diamond);
  let x4 = (self.diamond.rec2.x + SCALED_TILE/2_f32) as i32;
  let y4 = (self.diamond.rec2.y + SCALED_TILE/2_f32) as i32;
  d.draw_text(&diamond_str, x4, y4,TEXT_SIZE/2, Color::WHITE);
}

pub fn draw(&mut self,d:&mut RaylibDrawHandle,sheets:&Texture2D,frame_time:f32){
  d.clear_background(BACKGROUND_COLOR);   
  self.grid.draw(d, sheets);
  self.player.draw(d,sheets);
  self.draw_game_state(d,sheets,frame_time);
  self.draw_score(d);
  self.draw_icons(d, sheets);
  self.draw_text(d);
  //self.draw_blur(d);
  }

  pub fn update(&mut self,rl:&mut raylib::RaylibHandle,audio:&mut RaylibAudio,sounds:GameSounds,frame_time:f32){
    let (&ref exp_sound,&ref bonus_sound,&ref gameover,&ref upgrade_sound,&ref win_sound,&ref burning_sound,&ref punch) = sounds;
    let player_sounds:PlayerSounds = (&bonus_sound,&upgrade_sound);
    self.handle_game_state(rl);
    self.update_game_state();
    if self.state == GameState::RUNNING {
    self.player.update(rl, &mut self.grid, audio, player_sounds, frame_time);
    self.grid.update(audio, exp_sound, frame_time);
    self.update_icons(frame_time);
    }
 }
 
}
