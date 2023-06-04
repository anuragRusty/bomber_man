use raylib::prelude::*;
use crate::grid::*;
use crate::bomb::*;

const SPEED:f32 = 30_f32 * SCALE;
const MAX_PLAYER_FRAME:usize = 4;
const P_COLORS:&[Color;3] = &[Color::RED,Color::YELLOW,Color::WHITE];
const MARGIN:f32 = 2_f32;
const BOMB_RELOAD_TIME:f32 = 1_f32;
const BOMB_MARGIN:f32 = TILE_SIZE/2_f32;

const LD_Y:f32 = 48_f32;
const RT_Y:f32 = 64_f32;
const DS_Y:f32 = 80_f32;

const LRD_FRAMES:&[f32;4] = &[0_f32,16_f32,32_f32,48_f32];
const TDS_FRAMES:&[f32;4] = &[64_f32,80_f32,96_f32,112_f32];
const STAND_FRAMES:&[usize;2] = &[0,3];

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
impl_dir_draw!(Player,draw_death,rec_death);
impl_dir_draw!(Player,draw_spawn,rec_spawn);

impl Player{
   pub fn new() -> Self {
     let tile_size = SCALE * TILE_SIZE;
     let dir = DIR::Down;
     let moving = false;
     let tint = P_COLORS[2];
     let frames = 0;
     let rec2 =  Rectangle::new(tile_size, tile_size, tile_size, tile_size);
     let rec_up = Rectangle::new(LRD_FRAMES[frames], RT_Y, TILE_SIZE, TILE_SIZE);
     let rec_down = Rectangle::new(TDS_FRAMES[frames], LD_Y, TILE_SIZE, TILE_SIZE); 
     let rec_right = Rectangle::new(LRD_FRAMES[frames], RT_Y, TILE_SIZE, TILE_SIZE);
     let rec_left = Rectangle::new(TDS_FRAMES[frames], LD_Y, TILE_SIZE, TILE_SIZE);
     let rec_spawn = Rectangle::new(TDS_FRAMES[frames], DS_Y, TILE_SIZE, TILE_SIZE);
     let rec_death = Rectangle::new(LRD_FRAMES[frames], DS_Y, TILE_SIZE, TILE_SIZE);
     let state = State2::ALIVE;
     let time = 0_f32;
     let ds_delay:f32 = 0_f32;
     let bomb_reload_time = BOMB_RELOAD_TIME;
     Self{ dir ,moving,tint, rec2 , rec_up, rec_down, rec_right, rec_left,rec_spawn,rec_death, state, frames, time,ds_delay,bomb_reload_time}
    }

  pub fn go(&mut self,collision:&bool,frame_time:&f32){
    if *collision{ // if collison is true push to opposite direction using margin value.
        match self.dir{
            DIR::Down=> {self.dir = self.dir.flip(); self.rec2.y -= MARGIN}
            DIR::Up => {self.dir = self.dir.flip();self.rec2.y += MARGIN;}
            DIR::Right => {self.dir = self.dir.flip();self.rec2.x -= MARGIN;}
            DIR::Left => {self.dir = self.dir.flip();self.rec2.x += MARGIN;}
            _ => {}
         }
    }else if self.moving & !*collision{// Set direction for player movement.
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
    let row = ((self.rec2.x + BOMB_MARGIN)/ SCALED_TILE) as usize;
    let column =  ((self.rec2.y + BOMB_MARGIN)/ SCALED_TILE) as usize;

    if grid.cells[row][column] != EMPTY && self.bomb_reload_time < BOMB_RELOAD_TIME{
        return;
      } else if self.bomb_reload_time >= BOMB_RELOAD_TIME{
         let mut new_bomb = Bomb::new();
         new_bomb.set_position(row, column);
         let power = new_bomb.power;
         self.bomb_reload_time = 0_f32;

         grid.cells[row][column] = BOMB;
         grid.game_objs[row][column] = GameObjs::Bomb(new_bomb);
         grid.inject_flames_obj(power,row, column);
      }
   }

   pub fn control(&mut self,rl:&mut RaylibHandle,collisions:&[bool;2],frame_time:&f32,grid:&mut Grid){
    let wb_collision =  &collisions[0];
    let flame_exp = collisions[1];
    
    match self.state {
    State2::ALIVE => {
    if flame_exp {
       self.state = State2::DYING;
       self.frames = 0; 
    }
    else if rl.is_key_down(KeyboardKey::KEY_UP) && self.dir != DIR::NotUp{//Set direction and start movement.
        self.dir = DIR::Up;
        self.moving = true;
        self.go(wb_collision,frame_time);
    }else if rl.is_key_down(KeyboardKey::KEY_DOWN) && self.dir != DIR::NotDown{
        self.dir = DIR::Down;
        self.moving = true;
        self.go(wb_collision,frame_time);
    }else if rl.is_key_down(KeyboardKey::KEY_LEFT) && self.dir != DIR::NotLeft{
        self.dir = DIR::Left;
        self.moving = true;
        self.go(wb_collision,frame_time);
    }else if rl.is_key_down(KeyboardKey::KEY_RIGHT) && self.dir != DIR::NotRight{
        self.dir = DIR::Right;
        self.moving = true;
        self.go(wb_collision,frame_time);
    }else if rl.is_key_down(KeyboardKey::KEY_B) {
        self.plant_bomb(grid);
    }else{
        self.moving = false;
    }
   }
   _ => {}
  }
     
 }
   pub fn draw(&mut self,d:&mut RaylibDrawHandle,player_texture:&Texture2D,frame_time:&f32){    // Draw and update function.
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
   self.animate(frame_time);  //Animate the player.  
}

   pub fn animate(&mut self,frame_time:&f32){
    if self.bomb_reload_time <= BOMB_RELOAD_TIME{ //Enable or Disable bomb planting.
        self.bomb_reload_time += *frame_time;
      }
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
            self.rec2.x = TILE_SIZE*SCALE;
            self.rec2.y = TILE_SIZE*SCALE;
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

  pub fn collision(&self,cells:&Vec<Vec<i8>>) -> [bool;2] {
    let mut w_b = false;
    let mut flame_exp = false;
    let scaled_tile = (TILE_SIZE * SCALE) as usize;
    let scaled_margin:usize = (MARGIN * SCALE) as usize;
    //Set player height width with a margin.
    let player_height = scaled_tile -  scaled_margin;
    let player_width = scaled_tile - scaled_margin;
    let player_i = ((self.rec2.x + BOMB_MARGIN) / (SCALE * TILE_SIZE)) as usize;
    let player_j = ((self.rec2.y + BOMB_MARGIN) / (SCALE * TILE_SIZE)) as usize;
    let player_x = self.rec2.x as usize +  scaled_margin; 
    let player_y = self.rec2.y as usize +  scaled_margin;
    //Iterate over grid check collison using collison formula.
    for i in (player_i-1)..=(player_i+1)  {
        for j in (player_j-1)..=(player_j+1) {
            let cell = cells[i][j];
            if  cell < WALL + BLOCK {
                let cell_x = i * scaled_tile + scaled_margin;
                let cell_y = j * scaled_tile + scaled_margin;
                let cell_width = scaled_tile - scaled_margin;
                let cell_height = scaled_tile - scaled_margin;
                if player_x + player_width > cell_x && player_x < cell_x + cell_width &&
                   player_y + player_height > cell_y && player_y < cell_y + cell_height {
                    if  cell == WALL || cell == BLOCK{
                    w_b = true;
                    }else if  cell < 0{
                        flame_exp = true;
                        }
                   }
                 }
               }
            }
    return [w_b,flame_exp]
}
}