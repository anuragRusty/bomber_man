use raylib::prelude::*;
use crate::grid::*;
use crate::bomb::*;

const SPEED:f32 = 0.03;
const MAX_DEATH_FRAME:i32 = 6;
const PLAYER_COLOR:Color = Color::WHITE;
const MARGIN:f32 = 2_f32;
const BOMB_RELOAD_TIME:f32 = 3_f32;
const BOMB_MARGIN:f32 = 8_f32;
const EXPLOSION_CROP:f32 = 90_f32;
pub enum DIR {
    UP,
    DOWN,
    RIGHT,
    LEFT
}

impl DIR{
    fn get_val(&self) -> f32 {
      match self {
          DIR::UP => {144_f32}
          DIR::DOWN => {144_f32}
          DIR::RIGHT => {0_f32}
          DIR::LEFT => {0_f32}
      }
    } 
}
#[derive(PartialEq,Clone)]
pub enum PlayerState {
   ALIVE,
   DYING,
   DEAD,
}

pub struct Player{
    pub dir:DIR,
    pub moving:bool,
    pub vec2:Vector2,
    pub rec_up:Rectangle,
    pub rec_down:Rectangle,
    pub rec_right:Rectangle,
    pub rec_left:Rectangle,
    pub rec_death:Rectangle,
    pub state:PlayerState,
    pub frames:i32,
    pub time:f32,
    pub bomb_reload_time:f32,
}

impl Player{
   pub fn new() -> Self {
     let dir = DIR::UP;
     let moving = !false;
     let vec2 = Vector2::new(48_f32,48_f32);
     let rec_up = Rectangle::new(144_f32, 48_f32, (TILE_SIZE*SCALE) as f32, (TILE_SIZE*SCALE-1) as f32);
     let rec_down = Rectangle::new(144_f32, 0_f32, (TILE_SIZE*SCALE) as f32, (TILE_SIZE*SCALE-1) as f32);    
     let rec_right = Rectangle::new(0_f32, 48_f32, (TILE_SIZE*SCALE) as f32, (TILE_SIZE*SCALE-1) as f32);
     let rec_left = Rectangle::new(0_f32, 0_f32, (TILE_SIZE*SCALE) as f32, (TILE_SIZE*SCALE-1) as f32);
     let rec_death = Rectangle::new(0_f32, 96_f32, (TILE_SIZE*SCALE) as f32, (TILE_SIZE*SCALE-1) as f32);
     let state = PlayerState::ALIVE;
     let frames = 0;
     let time = 0_f32;
     let bomb_reload_time = BOMB_RELOAD_TIME;
     Self{ dir ,moving, vec2 , rec_up, rec_down, rec_right, rec_left, rec_death, state, frames, time,bomb_reload_time}
    }

  pub fn go(&mut self,collision:&bool){
    if *collision{ // if collison is true push to opposite direction using margin value.
        match self.dir{
            DIR::DOWN => {self.vec2.y -= MARGIN}
            DIR::UP => {self.vec2.y += MARGIN}
            DIR::RIGHT => {self.vec2.x -= MARGIN}
            DIR::LEFT => {self.vec2.x += MARGIN}
         }
    }else if self.moving & !*collision{// Set direction for player movement.
      match self.dir{
         DIR::DOWN => {self.vec2.y += SPEED}
         DIR::UP => {self.vec2.y -= SPEED}
         DIR::RIGHT => {self.vec2.x += SPEED}
         DIR::LEFT => {self.vec2.x -= SPEED}
      }
    }
   }

   pub fn plant_bomb(&mut self,grid:&mut Grid){
    let row = ((self.vec2.x + BOMB_MARGIN)/ (SCALE * TILE_SIZE) as f32) as i32;
    let column =  ((self.vec2.y + BOMB_MARGIN)/ (SCALE * TILE_SIZE) as f32) as i32;

     if grid.get_cell(row as i32, column as i32) != EMPTY && self.bomb_reload_time < BOMB_RELOAD_TIME{
        return;
      } else if self.bomb_reload_time >= BOMB_RELOAD_TIME{
         let mut new_bomb = Bomb::new();

         let right = grid.get_cell(row+1, column) == WALL || grid.get_cell(row+1, column) == BLOCK ;
         let left = grid.get_cell(row-1, column) == WALL || grid.get_cell(row-1, column) == BLOCK ;
         let down = grid.get_cell(row, column+1) == WALL || grid.get_cell(row, column+1) == BLOCK ;
         let top = grid.get_cell(row, column-1) == WALL || grid.get_cell(row, column-1) == BLOCK ;

         if down{
          new_bomb.exp_rec.height -= EXPLOSION_CROP;
         }

         if right{
          new_bomb.exp_rec.width -= EXPLOSION_CROP;
         }

         if top && left{
          new_bomb.exp_rec.y = 3_f32 * EXPLOSION_TILE;
         }else if top {
         
          new_bomb.exp_rec.y = EXPLOSION_TILE;
         }else if left{
          new_bomb.exp_rec.y = 2_f32 * EXPLOSION_TILE;
         }

         grid.bomb_vec.push(new_bomb);
         grid.set_cell(row as i32, column as i32, BOMB);
         self.bomb_reload_time = 0_f32;
      }
   }

   pub fn control(&mut self,rl:&mut RaylibHandle,collision:&bool,grid:&mut Grid){
    //Set direction and start movement.
    if rl.is_key_down(KeyboardKey::KEY_UP) {
        self.dir = DIR::UP;
        self.moving = true;
        self.go(collision);
    }else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
        self.dir = DIR::DOWN;
        self.moving = true;
        self.go(collision);
    }else if rl.is_key_down(KeyboardKey::KEY_LEFT) {
        self.dir = DIR::LEFT;
        self.moving = true;
        self.go(collision);
    }else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
        self.dir = DIR::RIGHT;
        self.moving = true;
        self.go(collision);
    }else if rl.is_key_down(KeyboardKey::KEY_B) {
        self.plant_bomb(grid);
    }else{
        self.moving = false;
    }
     
   }
   // Draw and update function.
   pub fn draw(&mut self,d:&mut RaylibDrawHandle,player_texture:&Texture2D,frame_time:&f32){
    //Draw the player on screen.
    match self.dir{
        DIR::DOWN => { d.draw_texture_rec(player_texture, self.rec_down, self.vec2, PLAYER_COLOR);}
        DIR::UP => {d.draw_texture_rec(player_texture, self.rec_up, self.vec2, PLAYER_COLOR);}
        DIR::RIGHT => {d.draw_texture_rec(player_texture, self.rec_right, self.vec2, PLAYER_COLOR);}
        DIR::LEFT => {d.draw_texture_rec(player_texture, self.rec_left, self.vec2, PLAYER_COLOR);}
     }
     //Animate the player.
     self.animate(frame_time)
         
   }

   pub fn animate(&mut self,frame_time:&f32){
    //When moving true and and death false
    if self.moving && self.state == PlayerState::ALIVE{
    let mut local_rec = &mut self.rec_down;//Default rectangle down as per direction.
    match self.dir{//Check player direction and set default rectangle.
        DIR::UP => {local_rec = &mut self.rec_up}
        DIR::RIGHT => {local_rec = &mut self.rec_right}
        DIR::LEFT => {local_rec = &mut self.rec_left}
        _ => {}
     }
   //Animate the player.
     if self.time > ANIM_DURATION{
        self.time = 0_f32;
        self.frames += 1;
    }
    self.time += *frame_time;
    local_rec.x = self.dir.get_val() + (TILE_SIZE * SCALE) as f32 * self.frames as f32;
    self.frames = self.frames  % (MAX_FRAME-1);
    
    //Enable or Disable bomb planting.
    if self.bomb_reload_time <= BOMB_RELOAD_TIME{
      self.bomb_reload_time += *frame_time;
    }
   }
 }
    
}