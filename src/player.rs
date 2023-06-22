use raylib::prelude::*;
use crate::bonus::*;
use crate::grid::*;
use crate::bomb::*;
use crate::noise::*;
use crate::upgrade::*;

pub type PlayerSounds<'a> = (&'a Sound, &'a Sound);

const SPEED:f32 = 30_f32 * SCALE;
const MAX_PLAYER_FRAME:usize = 4;
const P_COLORS:&[Color;3] = &[Color::RED,Color::YELLOW,Color::WHITE];
const MARGIN:f32 = 0.7*SCALE;
const BOMB_RELOAD_TIME:f32 = 1_f32;

const LD_Y:f32 = 48_f32; // Left Down
const RT_Y:f32 = 64_f32; // Right Top
const DS_Y:f32 = 80_f32; // Death Spawn

const LRD_FRAMES:&[f32;4] = &[0_f32,16_f32,32_f32,48_f32];
const TDS_FRAMES:&[f32;4] = &[64_f32,80_f32,96_f32,112_f32];
const STAND_FRAMES:&[usize;2] = &[0,3];
// Collison shape const for player
const COLL_MARGIN_X:f32 = 3_f32*SCALE;
const COLL_MARGIN_Y:f32 = 2_f32*SCALE;
const PLAYER_HEIGHT:f32 = 13_f32*SCALE;
const PLAYER_WIDTH:f32 = 9_f32*SCALE;

#[derive(Clone,PartialEq,Debug,Copy)]
pub enum DIR {
    Up,
    Down,
    Right,
    Left,
    NotUp,
    NotDown,
    NotRight,
    NotLeft,
}

impl DIR{
    fn flip(&self) -> DIR{
        match self{
            DIR::Down => {DIR::NotDown}
            DIR::Up => {DIR::NotUp}
            DIR::Left => {DIR::NotLeft}
            DIR::Right => {DIR::NotRight}
            DIR::NotDown => {DIR::Down}
            DIR::NotUp => {DIR::Up}
            DIR::NotLeft => {DIR::Left}
            DIR::NotRight => {DIR::Right}
        }
    }
}

#[derive(Clone,PartialEq,Debug)]
pub enum State2 {
   SPAWN,
   SPAWNING,
   ALIVE,
   DYING,
   DEAD,
}

#[derive(Clone,PartialEq,Debug)]
pub struct Player {
    pub dir:DIR,
    pub weapon:UpgradeType,
    pub lifes:usize,
    pub cash:usize,
    pub silver_coin:usize,
    pub gold_coin:usize,
    pub diamond:usize,
    pub black_bombs:usize,
    pub blue_bombs:usize,
    pub purple_bombs:usize,
    pub red_bombs:usize,
    pub delay_bool:bool,
    pub temp_score:usize,
    pub score:usize,
    pub moving:bool,
    pub tint:Color,
    pub rec2:Rectangle,
    pub rec_up:Rectangle,
    pub rec_down:Rectangle,
    pub rec_right:Rectangle,
    pub rec_left:Rectangle,
    pub rec_spawn:Rectangle,
    pub rec_death:Rectangle,
    pub rec_shadow:Rectangle,
    pub state:State2,
    pub frames:usize,
    pub time:f32,
    pub delay:f32,
    pub bomb_reload_time:f32,
}

macro_rules! impl_dir_draw {
    ($name:ident,$fn_name:ident,$prop:ident) => {
        impl $name {
            fn $fn_name(&mut self,player_texture:&Texture2D,d:&mut RaylibDrawHandle){
                d.draw_texture_pro(player_texture, self.$prop, self.rec2,Vector2::default(),O, self.tint);
            }
        }
    };
}

impl_dir_draw!(Player,draw_down,rec_down);
impl_dir_draw!(Player,draw_up,rec_up);
impl_dir_draw!(Player,draw_left,rec_left);
impl_dir_draw!(Player,draw_right,rec_right);
impl_dir_draw!(Player,draw_spawn,rec_spawn);
impl_dir_draw!(Player,draw_death,rec_death);
impl_dir_draw!(Player,draw_shadow,rec_shadow);

impl Player{
   pub fn new() -> Self {
     let dir = DIR::Down;
     let weapon = UpgradeType::Default;
     let lifes = 3;
     let cash = 0;
     let silver_coin = 0;
     let gold_coin = 0;
     let diamond = 0;
     let black_bombs = 0;
     let blue_bombs = 0;
     let purple_bombs = 0;
     let red_bombs = 0;
     let delay_bool = true;
     let temp_score = 0;
     let score = 0;
     let moving = false;
     let tint = P_COLORS[2];
     let frames = 0;
     let rec2 =  Rectangle::new(SCALED_TILE, SCALED_TILE, SCALED_TILE, SCALED_TILE);
     let rec_up = Rectangle::new(LRD_FRAMES[frames], RT_Y, TILE_SIZE, TILE_SIZE);
     let rec_down = Rectangle::new(TDS_FRAMES[frames], LD_Y, TILE_SIZE, TILE_SIZE); 
     let rec_right = Rectangle::new(LRD_FRAMES[frames], RT_Y, TILE_SIZE, TILE_SIZE);
     let rec_left = Rectangle::new(TDS_FRAMES[frames], LD_Y, TILE_SIZE, TILE_SIZE);
     let rec_spawn = Rectangle::new(TDS_FRAMES[frames], DS_Y, TILE_SIZE, TILE_SIZE);
     let rec_death = Rectangle::new(LRD_FRAMES[frames], DS_Y, TILE_SIZE, TILE_SIZE);
     let rec_shadow = Rectangle::new(FRAMES[5],32_f32,TILE_SIZE,TILE_SIZE);
     let state = State2::SPAWN;
     let time = 0_f32;
     let delay = 0.11_f32;
     let bomb_reload_time = BOMB_RELOAD_TIME;
     Self{dir,weapon,lifes,cash,silver_coin,gold_coin,diamond,black_bombs,blue_bombs,purple_bombs,red_bombs,delay_bool,temp_score,score ,moving,tint, rec2 , rec_up, rec_down, rec_right, rec_left,rec_spawn,rec_death,rec_shadow, state, frames, time,delay,bomb_reload_time}
    }

    pub fn get_coll_shape(&self) -> Rectangle {
        let x = self.rec2.x + COLL_MARGIN_X;
        let y = self.rec2.y + COLL_MARGIN_Y;
        let rec = Rectangle::new(x,y,PLAYER_WIDTH,PLAYER_HEIGHT);
        return rec;
     }
   
     pub fn get_position(&self) -> Position {
       let i = ((self.rec2.x + MARGIN_POS)/ SCALED_TILE) as usize;
       let j =  ((self.rec2.y + MARGIN_POS)/ SCALED_TILE) as usize;
       return (i,j);
   }

  pub fn go(&mut self,collision:bool,frame_time:f32){
    if collision{ // if collison is true push to opposite direction using margin value.
        match self.dir{
            DIR::Down=> {self.dir = self.dir.flip(); self.rec2.y -= MARGIN}
            DIR::Up => {self.dir = self.dir.flip();self.rec2.y += MARGIN}
            DIR::Right => {self.dir = self.dir.flip();self.rec2.x -= MARGIN}
            DIR::Left => {self.dir = self.dir.flip();self.rec2.x += MARGIN}
            _ => {}
         }
    }else if self.moving & !collision{// Set direction for player movement.
      match self.dir{
         DIR::Down => {self.rec2.y += SPEED * frame_time}
         DIR::Up => {self.rec2.y -= SPEED * frame_time}
         DIR::Right => {self.rec2.x += SPEED * frame_time}
         DIR::Left => {self.rec2.x -= SPEED * frame_time}
         _ => {}
      }
    }
   }

   pub fn plant_bomb(&mut self,grid:&mut Grid){
     let position = self.get_position();
     let (i,j) = position;
    if grid.cells[i][j] != EMPTY || self.bomb_reload_time < BOMB_RELOAD_TIME{
        return;
      } else if self.bomb_reload_time >= BOMB_RELOAD_TIME{
         let mut new_bomb = Bomb::new();
         new_bomb.set_position(i,j);
         self.bomb_reload_time = 0_f32;
         grid.cells[i][j] = BOMB;
         grid.game_objs[i][j] = GameObjs::Bomb(new_bomb);
      }
   }

   pub fn control(&mut self,rl:&mut RaylibHandle,frame_time:f32,grid:&mut Grid){
    let obj_rec = self.get_coll_shape();
    let position = self.get_position();
    let neutral_coll = grid.get_collisions(position, obj_rec).1;

    match self.state { 
    State2::ALIVE => {
     if rl.is_key_down(KeyboardKey::KEY_UP) && self.dir != DIR::NotUp{//Set direction and start movement.
        self.dir = DIR::Up;
        self.moving = true;
        self.go(neutral_coll,frame_time);
    }else if rl.is_key_down(KeyboardKey::KEY_DOWN) && self.dir != DIR::NotDown{
        self.dir = DIR::Down;
        self.moving = true;
        self.go(neutral_coll,frame_time);
    }else if rl.is_key_down(KeyboardKey::KEY_LEFT) && self.dir != DIR::NotLeft{
        self.dir = DIR::Left;
        self.moving = true;
        self.go(neutral_coll,frame_time);
    }else if rl.is_key_down(KeyboardKey::KEY_RIGHT) && self.dir != DIR::NotRight{
        self.dir = DIR::Right;
        self.moving = true;
        self.go(neutral_coll,frame_time);
    }else if rl.is_key_pressed(KeyboardKey::KEY_B) {
        self.moving = false;
        self.plant_bomb(grid);
    }else{
        self.moving = false;
    }
   }
   _ => {}
  } 
 }
  
  pub fn bomb_reload(&mut self,frame_time:f32){
    if self.bomb_reload_time <= BOMB_RELOAD_TIME { //Enable or Disable bomb planting.
        self.bomb_reload_time += frame_time;
      }
  }

  pub fn take(&mut self,grid:&mut Grid,audio:&mut RaylibAudio,sounds:PlayerSounds){
      let (bonus_sound,upgrade_sound) = sounds;
      let p_pos = self.get_position();

      for i in 0..grid.upgrade_vec.len(){
          let upgrade = &mut grid.upgrade_vec[i];
          let u_pos = upgrade.get_position();

          if p_pos == u_pos {
            match upgrade.up_type {
             UpgradeType::BlackBomb => { self.black_bombs += upgrade.val;  upgrade.play_audio(audio, upgrade_sound);},
             UpgradeType::BlueBomb => { self.blue_bombs += upgrade.val;  upgrade.play_audio(audio, upgrade_sound);},
             UpgradeType::PurpleBomb => { self.purple_bombs += upgrade.val;  upgrade.play_audio(audio, upgrade_sound);},
             UpgradeType::RedBomb => { self.red_bombs += upgrade.val;  upgrade.play_audio(audio, upgrade_sound);},
             _ => {}
            }
            grid.rm_upgrade_obj(i);
        }
      }

      for i in 0..grid.bonus_vec.len(){
          let bonus = &mut grid.bonus_vec[i];
          let b_pos = bonus.get_position();
          if p_pos == b_pos {
             match bonus.bonus_type {
              BonusType::Heart => {self.lifes += bonus.val; bonus.play_audio(audio, bonus_sound)},
              BonusType::Cash => {self.cash += 1; self.temp_score += bonus.val;bonus.play_audio(audio, bonus_sound)},
              BonusType::SilverCoin => {self.silver_coin += 1; self.temp_score += bonus.val;bonus.play_audio(audio, bonus_sound)},
              BonusType::GoldCoin => {self.gold_coin += 1; self.temp_score += bonus.val;bonus.play_audio(audio, bonus_sound)},
              BonusType::Diamond => {self.diamond += 1; self.temp_score += bonus.val;bonus.play_audio(audio, bonus_sound)},
               _ => {}
             }
             grid.rm_bonus_obj(i);
          }
      }
  }

  pub fn animate(&mut self,frame_time:f32){ 
    let mut local_rec = &mut self.rec_down;//Default rectangle down as per direction.
    let mut local_frames = TDS_FRAMES;

    match self.dir{//Check player direction and set default rectangle.
        DIR::Up => {local_rec = &mut self.rec_up; local_frames = TDS_FRAMES;}
        DIR::Right => {local_rec = &mut self.rec_right; local_frames = LRD_FRAMES;}
        DIR::Left => {local_rec = &mut self.rec_left; local_frames = LRD_FRAMES}
        _ => {}
     }

    match self.state {
        State2::DYING => { 
            local_frames = &LRD_FRAMES; 
            local_rec = &mut self.rec_death;
            local_rec.x = local_frames[self.frames];
            self.tint = P_COLORS[self.frames % (P_COLORS.len()-1)];}
        State2::SPAWNING => {
            local_frames = &TDS_FRAMES; self.tint = P_COLORS[2];
            local_rec = &mut self.rec_spawn;
            local_rec.x = local_frames[self.frames];
         }
        _ => {}
     }

     if self.time > ANIM_DURATION + self.delay && self.state != State2::DEAD{
        self.time = 0_f32;
        self.frames += 1;
     }
    self.time += frame_time;
    self.frames %= MAX_PLAYER_FRAME;
    
    if self.moving && self.state == State2::ALIVE{//Animate the player when moving true and and death false.
    local_rec.x = local_frames[self.frames]; 
    }else if !self.moving && self.state == State2::ALIVE {  //Animate the player when moving false and and death false.
        self.frames = STAND_FRAMES[self.frames%STAND_FRAMES.len()];
        local_rec.x = local_frames[self.frames];
    }
  }

  pub fn update_score(&mut self){
      if self.temp_score > 0 && self.delay_bool{
         self.score += 1;
         self.temp_score -= 1;
      }
      self.delay_bool = !self.delay_bool;
  }
  
  pub fn update_state(&mut self,grid:&mut Grid,frame_time:f32){
      let position = self.get_position();
      let obj_rec = self.get_coll_shape();
      let fatal_coll = grid.get_collisions(position, obj_rec).0;

      match self.state {
        State2::SPAWN => {
          self.delay += frame_time;
          if self.delay > 2_f32 {
            self.delay = 0.11;
            self.state = State2::SPAWNING;
          }
        }
        State2::SPAWNING => {
            if self.frames >= MAX_FRAME{
                self.state = State2::ALIVE;
                self.dir = DIR::Down;
                self.frames = 0;
                self.delay = 0_f32;  
            }
        }
        State2::ALIVE => {
          if fatal_coll{
             self.lifes -= 1;
             self.state = State2::DYING;
             self.delay = 0.11;
             self.frames = 0;
          }
        }
        State2::DYING => {
          if self.frames >= MAX_FRAME{
            if self.lifes <= 0 {
             self.state = State2::DEAD;
            }else{
             self.frames = 0;
             self.delay = 0_f32;
             self.state = State2::SPAWN;
             self.tint = Color::WHITE;
            }  
          }
        }
        _ => {}
      }
  }

  pub fn draw(&mut self,d:&mut RaylibDrawHandle,player_texture:&Texture2D){    // Draw and update function.
    match self.state {
      State2::ALIVE => 
       match self.dir {   //Draw the player on screen.
         DIR::Down => { self.draw_shadow(player_texture, d);self.draw_down(player_texture, d)}
         DIR::Up => {self.draw_shadow(player_texture, d);self.draw_up(player_texture,d)}
         DIR::Right => {self.draw_shadow(player_texture, d);self.draw_right(player_texture, d)}
         DIR::Left => {self.draw_shadow(player_texture, d);self.draw_left(player_texture, d)}
         DIR::NotDown => {self.draw_shadow(player_texture, d);self.draw_down(player_texture, d)}
         DIR::NotUp => {self.draw_shadow(player_texture, d);self.draw_up(player_texture,d)}
         DIR::NotRight => {self.draw_shadow(player_texture, d);self.draw_right(player_texture, d)}
         DIR::NotLeft => {self.draw_shadow(player_texture, d);self.draw_left(player_texture, d)}
       }
       State2::SPAWNING => {self.draw_spawn(player_texture, d);}
       State2::DYING => {self.draw_death(player_texture, d);}
     _ => {}   
    }  
 }

  pub fn update(&mut self,rl:&mut RaylibHandle,grid:&mut Grid,audio:&mut RaylibAudio,sounds:PlayerSounds,frame_time:f32){
    self.take(grid, audio, sounds);
    self.update_score();
    self.update_state(grid,frame_time);
    self.control(rl, frame_time, grid);
    self.bomb_reload(frame_time);
    self.animate(frame_time);
  }
}