use raylib::prelude::*;
use crate::KeyboardKey::{KEY_P,KEY_ESCAPE,KEY_R};
use crate::player::*;
use crate::grid::*;

const BACKGROUND_COLOR:Color = Color::new(28, 52, 112, 255); 
const GO_FRAMES:&[f32;2] = &[0_f32,144_f32];
const GO_WIDTH:f32 = GO_FRAMES[1];
const GO_Y:&[f32;1]= &[208_f32];

const CD_Y:&[f32;2] = &[240_f32,256_f32];
const CD_FRAMES:&[f32;5] = &[0_f32,32_f32,64_f32,108_f32,180_f32];
const CD_WIDTH:f32 = 48_f32;

const PAUSED_Y:&[f32;1] = &[224_f32];
const P_FRAMES:&[f32;2] = &[0_f32,112_f32];
const P_WIDTH:f32 = P_FRAMES[1];

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
            let y = (h as f32 /2.0) - ((TILE_SIZE*SCALE/2.0));
            let rec2 = Rectangle::new(x,y,$w*SCALE,TILE_SIZE*SCALE);
            let time = 0_f32;
            Self { rec, rec2, frames, time}
           }
        
            fn draw_animate(&mut self,d:&mut RaylibDrawHandle,texts:&Texture2D,frame_time:&f32){
              d.draw_texture_pro(texts, self.rec,self.rec2, Vector2::default(), 0.0, Color::WHITE);

              if self.time > $dur{
                self.time  = 0_f32;
                self.frames += 1;
              }

              self.frames %= $frames.len();
              self.time += *frame_time;
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
impl_text_obj!(CountDown,CD_Y,CD_WIDTH,CD_FRAMES,1.0);

#[derive(PartialEq,Clone)]
pub enum GameState{
    STARTING,
    RUNNING,
    PAUSED,
    GAMEOVER,
}

#[derive(PartialEq,Clone)]
pub enum GameOption{
    IDEAL,
    RESUME,
    NEW_GAME,
    RESTART_LEVEL,
    SOUND_ON,
    SOUND_OFF,
    CLOSE,
}

pub struct Game {
    pub state:GameState,
    pub option:GameOption,
    pub menu:bool,
    pub screen_w:i32,
    pub screen_h:i32,
    pub player:Player,
    pub grid:Grid,
    game_over_text:GameOver,
    paused_text:Paused,
    count_down:CountDown,
    frames:usize,
    time:f32,
}

impl Game {
 pub fn new() -> Self{
     let state = GameState::STARTING;
     let option = GameOption::IDEAL;
     let menu = false;
     let grid = Grid::new();
     let screen_w = SCALED_TILE as i32 * grid.cells.len() as i32;
     let screen_h = SCALED_TILE as i32 * grid.cells[1].len() as i32;
     let player = Player::new();
     let game_over_text = GameOver::new(screen_w,screen_h);
     let paused_text = Paused::new(screen_w,screen_h);
     let count_down = CountDown::new(screen_w,screen_h);
     let frames = 0;
     let time = 0_f32;
     Self { state, option,menu,screen_w,screen_h, player,grid,game_over_text,paused_text,count_down,frames,time}
   }

 pub fn update(&mut self,rl:&mut raylib::RaylibHandle,frame_time:&f32){
    self.handle_game_state(rl);
    
    if self.state == GameState::RUNNING {
    self.player.control(rl,frame_time,&mut self.grid);
    self.player.update(frame_time);
    }
 }

 pub fn render(&mut self,d:&mut RaylibDrawHandle,sheets:&Texture2D,audio:&mut RaylibAudio,sounds:&Sound,frame_time:&f32){
       d.clear_background(BACKGROUND_COLOR);   
       self.grid.render(d, sheets,audio,sounds,frame_time);
       self.player.draw(d,sheets);
       self.draw_game_state(d,sheets,frame_time);
    }

pub fn handle_game_state(&mut self,rl:&mut RaylibHandle){
        if self.state != GameState::PAUSED {
        match self.player.state{
        State2::SPAWN => {self.state = GameState::STARTING}
        State2::ALIVE => {self.state = GameState::RUNNING}
        State2::DEAD => {self.state = GameState::GAMEOVER}
        _ => {}
        }
      }

      if rl.is_key_pressed(KEY_P) && self.state != GameState::GAMEOVER{
       self.state = if self.state == GameState::PAUSED{GameState::STARTING}else{GameState::PAUSED};
      }else if rl.is_key_down(KEY_ESCAPE) {
        self.menu = !self.menu;
      }else  if rl.is_key_pressed(KEY_R){
        self.grid = Grid::new();
        self.player = Player::new();
     }
    }

   pub fn draw_game_state(&mut self,d:&mut RaylibDrawHandle,texts:&Texture2D,frame_time:&f32){
      match self.state{
        GameState::GAMEOVER => {self.game_over_text.draw_animate(d, texts,frame_time)}
        GameState::PAUSED => {self.paused_text.draw_animate(d, texts, frame_time)}
        GameState::STARTING => {self.count_down.draw_animate(d, texts, frame_time); self.anim_cd(frame_time)}
      _ => {} 
   
   }  
  }
  fn anim_cd(&mut self,frame_time:&f32){
    if self.time > ANIM_DURATION{
      self.frames += 1;
      self.time = 0_f32;
    }
    self.time += *frame_time;
    self.frames %= CD_Y.len();
    self.count_down.rec.y = CD_Y[self.frames];
  }   
}
