use raylib::prelude::*;
use crate::grid::*;
use crate::bomb::*;

const SPEED:f32 = 30_f32 * SCALE;
const MAX_PLAYER_FRAME:usize = 4;
const P_COLORS:&[Color;3] = &[Color::RED,Color::YELLOW,Color::WHITE];
const MARGIN:f32 = 0.7*SCALE;
const BOMB_RELOAD_TIME:f32 = 1_f32;
const BOMB_MARGIN:f32 = TILE_SIZE/2_f32;

const LD_Y:f32 = 48_f32;
const RT_Y:f32 = 64_f32;
const DS_Y:f32 = 80_f32;

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
   ALIVE,
   DYING,
   DEAD,
}

#[derive(Clone,PartialEq,Debug)]
pub struct Player{
    pub dir:DIR,
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
    pub ds_delay:f32,
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
     let rec_shadow = Rectangle::new(FRAMES[4],32_f32,TILE_SIZE,TILE_SIZE);
     let state = State2::ALIVE;
     let time = 0_f32;
     let ds_delay:f32 = 0_f32;
     let bomb_reload_time = BOMB_RELOAD_TIME;
     Self{ dir ,moving,tint, rec2 , rec_up, rec_down, rec_right, rec_left,rec_spawn,rec_death,rec_shadow, state, frames, time,ds_delay,bomb_reload_time}
    }

  pub fn go(&mut self,collision:bool,frame_time:&f32){
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
         DIR::Down => {self.rec2.y += SPEED * *frame_time}
         DIR::Up => {self.rec2.y -= SPEED * *frame_time}
         DIR::Right => {self.rec2.x += SPEED * *frame_time}
         DIR::Left => {self.rec2.x -= SPEED * *frame_time}
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

   pub fn control(&mut self,rl:&mut RaylibHandle,frame_time:&f32,grid:&mut Grid){
    let obj_rec = self.get_coll_shape();
    let position = self.get_position();
    let collisions = grid.get_collisions(position, obj_rec);
    let (fatal_coll,neutral_coll,_bonus_coll,_upgrade_coll,_win_coll) = collisions;
    match self.state { 
    State2::ALIVE => {
    if fatal_coll {
       self.state = State2::DYING;
       self.frames = 0;  // BUG FIX for death ANIME;
    }
    else if rl.is_key_down(KeyboardKey::KEY_UP) && self.dir != DIR::NotUp{//Set direction and start movement.
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
   pub fn draw(&mut self,d:&mut RaylibDrawHandle,player_texture:&Texture2D){    // Draw and update function.
       self.draw_shadow(player_texture, d);
   match self.state {
     State2::ALIVE =>  match self.dir{   //Draw the player on screen.
        DIR::Down => { self.draw_down(player_texture, d)}
        DIR::Up => {self.draw_up(player_texture,d)}
        DIR::Right => {self.draw_right(player_texture, d)}
        DIR::Left => {self.draw_left(player_texture, d)}
        DIR::NotDown => { self.draw_down(player_texture, d)}
        DIR::NotUp => {self.draw_up(player_texture,d)}
        DIR::NotRight => {self.draw_right(player_texture, d)}
        DIR::NotLeft => {self.draw_left(player_texture, d)}
      }
      State2::SPAWN => {self.draw_spawn(player_texture, d);}
      State2::DYING => {self.draw_death(player_texture, d);}
    _ => {}   
   }  
}

  pub fn update(&mut self,frame_time:&f32){
    self.bomb_reload(frame_time);
    self.animate(frame_time);
  }

  pub fn bomb_reload(&mut self,frame_time:&f32){
    if self.bomb_reload_time <= BOMB_RELOAD_TIME { //Enable or Disable bomb planting.
        self.bomb_reload_time += *frame_time;
      }
  }

  pub fn animate(&mut self,frame_time:&f32){ 
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
            self.ds_delay = 0.88;
            if self.frames >= MAX_FRAME{
            self.state = State2::DEAD;
            self.frames = 0;
        }
        self.tint = P_COLORS[self.frames % (P_COLORS.len()-1)];
       }
        State2::SPAWN => {
            local_frames = &TDS_FRAMES;
            self.ds_delay = 0_f32;
            self.tint = P_COLORS[2];
            if self.frames >= MAX_FRAME{
                self.state = State2::ALIVE;
                self.dir = DIR::Down;
                self.frames = 0; 
            }
        }
        _ => {}
     }

     if self.time > (ANIM_DURATION + self.ds_delay){//Animate the player when moving true and and death false.
        self.time = 0_f32;
        self.frames += 1;
     }
    self.time += *frame_time;
    self.frames %= MAX_PLAYER_FRAME;
    
    if self.moving && self.state == State2::ALIVE{//Animate the player when moving true and and death false.
    local_rec.x = local_frames[self.frames]; 
    }else if !self.moving && self.state == State2::ALIVE {  //Animate the player when moving false and and death false.
        self.frames = STAND_FRAMES[self.frames%STAND_FRAMES.len()];
        local_rec.x = local_frames[self.frames];
    }
  }

  pub fn get_coll_shape(&self) -> Rectangle {
     let x = self.rec2.x + COLL_MARGIN_X;
     let y = self.rec2.y + COLL_MARGIN_Y;
     let rec = Rectangle::new(x,y,PLAYER_WIDTH,PLAYER_HEIGHT);
     return rec;
  }

  pub fn get_position(&self) -> Position {
    let i = ((self.rec2.x + BOMB_MARGIN)/ SCALED_TILE) as usize;
    let j =  ((self.rec2.y + BOMB_MARGIN)/ SCALED_TILE) as usize;
    return (i,j);
  }
}