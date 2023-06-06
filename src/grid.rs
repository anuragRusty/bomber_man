use std::vec;
use raylib::prelude::*;
use crate::objects::*;
use crate::bomb::*;
use noise::{NoiseFn, Perlin};

pub type CollisonBools = (bool,bool,bool,bool,bool);
pub type Position = (usize,usize);

pub const TILE_SIZE:f32 = 16_f32;
pub const SCALE:f32 = 3_f32;
pub const SCALED_TILE:f32 = TILE_SIZE*SCALE;

pub const MAX_FRAME:usize = 3;
pub const ANIM_DURATION:f32 = 0.22_f32;
pub const MAX_RAND_FRAME:usize = 15;
//frames const
pub const O:f32 = 0_f32;
pub const FRAMES:[f32;8] = [0.0,16.0,32.0,48.0,64.0,80.0,96.0,108.0];
//dumb enum values.
pub const EMPTY:i8 = 0;
pub const BLOCK:i8 = 4;
pub const WALL:i8 = 5;
pub const BOMB:i8 = 1;
//sub neg enum
pub const EXPLOSION:i8 = -1;
pub const FLAME_MID_LEFT:i8 = -2; 
pub const FLAME_MID_RIGHT:i8 = -4;
pub const FLAME_MID_TOP:i8 = -6;
pub const FLAME_MID_DOWN:i8 = -8;

pub const FLAME_END_LEFT:i8 = -10;
pub const FLAME_END_RIGHT:i8 = -20;
pub const FLAME_END_TOP:i8 = -30;
pub const FLAME_END_DOWN:i8 = -40;

#[derive(PartialEq,Clone,Debug,Copy)]
pub enum GameObjs {
    Default,
    Wall(Wall),
    Block(Block),
    Bomb(Bomb),
    FlameLeftEnd(FlameLeftEnd),
    FlameRightEnd(FlameRightEnd),
    FlameTopEnd(FlameTopEnd),
    FlameDownEnd(FlameDownEnd),
    FlameLeftMid(FlameLeftMid),
    FlameRightMid(FlameRightMid),
    FlameTopMid(FlameTopMid),
    FlameDownMid(FlameDownMid),
}

pub struct Grid {
    pub cells:Vec<Vec<i8>>,
     empty_vec:Vec<Empty>,
     grass_vec:Vec<Grass>,
    pub game_objs:Vec<Vec<GameObjs>>,
 }

macro_rules! impl_inject_flames {
    ($obj:ident,$name:ident,$check:expr,$check2:expr,
        $flame_mid_left:expr,$flame_end_left:expr,
        $flame_mid_right:expr,$flame_end_right:expr,
        $flame_mid_top:expr,$flame_end_top:expr,
        $flame_mid_down:expr,$flame_end_down:expr,
        $matrix:ident) => {
    impl $obj {
        pub fn $name(&mut self, l: usize, r: usize, c: usize) {
            if self.cells[r][c] == $check {
                for &(x, y, mid, end) in 
                [(-1, 0,$flame_mid_left, $flame_end_left),
                 (1, 0, $flame_mid_right,$flame_end_right),
                 (0, -1,$flame_mid_top,$flame_end_top), 
                 (0, 1, $flame_mid_down,$flame_end_down)].iter() {
    
                    for i in 1..=l {
                        let row = (r as isize + x as isize * i as isize) as usize;
                        let col = (c as isize + y as isize * i as isize) as usize;
                        let cell = &mut self.$matrix[row][col];
                        if *cell != $check2 {
                            break;
                        }
                        *cell = if i == l { end } else { mid };
                    }
                }
            }
        } 
      }
    };
}

impl_inject_flames!(Grid,inject_flames,EXPLOSION,EMPTY,
    FLAME_MID_LEFT,FLAME_END_LEFT,FLAME_MID_RIGHT,
    FLAME_END_RIGHT,FLAME_MID_TOP,FLAME_END_TOP,
    FLAME_MID_DOWN,FLAME_END_DOWN,cells);
    
impl_inject_flames!(Grid,inject_flames_obj,EXPLOSION,GameObjs::Default, 
    GameObjs::FlameLeftMid(FlameLeftMid::new()),GameObjs::FlameLeftEnd(FlameLeftEnd::new()),
    GameObjs::FlameRightMid(FlameRightMid::new()),GameObjs::FlameRightEnd(FlameRightEnd::new()), 
    GameObjs::FlameTopMid(FlameTopMid::new()), GameObjs::FlameTopEnd(FlameTopEnd::new()),
    GameObjs::FlameDownMid(FlameDownMid::new()), GameObjs::FlameDownEnd(FlameDownEnd::new()),game_objs);

impl Grid {
    pub fn new() -> Self  {
        let cells = Grid::grid_gen(26, 15);
        let mut empty_vec:Vec<Empty> = vec![];
        let mut grass_vec:Vec<Grass> = vec![];
        let mut game_objs = vec![vec![GameObjs::Default;cells[1].len()]; cells.len()];

        for i in 0..cells.len(){
            for j in 0..cells[i].len(){
                let curr_cell = cells[i][j];
                if curr_cell == WALL {
                    let mut wall = Wall::new();
                    wall.set_position(i, j);
                    game_objs[i][j] = GameObjs::Wall(wall);
                }else if curr_cell == BOMB {
                    let mut bomb = Bomb::new();
                    bomb.set_position(i, j);
                    game_objs[i][j] = GameObjs::Bomb(bomb);
                }else if curr_cell == BLOCK{
                    let mut block = Block::new();
                    block.set_position(i, j);
                    game_objs[i][j] = GameObjs::Block(block);
                }else if curr_cell == EMPTY{
                    let mut grass = Grass::new();
                    grass.set_position(i,j);
                    grass_vec.push(grass);
                }

                let empty = Empty::new();
                empty_vec.push(empty);
            }
        }
        return Self {cells,empty_vec,grass_vec,game_objs};
    }

    pub fn remove_obj(&mut self,i:usize,j:usize){
        self.cells[i][j] = EMPTY;
        self.game_objs[i][j] = GameObjs::Default;  
    }

    pub fn grid_gen(r: usize, c: usize) -> Vec<Vec<i8>> {
        let mut grid = vec![vec![0; c]; r];
        let perlin = Perlin::new(120727);
        let threshold = 0.02; // controls the density of solid blocks
    
        for i in 0..r {
            for j in 0..c {
                let cell = &mut grid[i][j];
                if i == 0 || j == 0 || i == r - 1 || j == c - 1 {
                    *cell = BLOCK;
                }else {
                    let x = i as f64 / r as f64;
                    let y = j as f64 / c as f64;
                    let noise = perlin.get([x * 10.0, y * 10.0]); // adjust the frequency of noise
                    if noise >= threshold {
                        *cell = WALL;
                    }
                   if (i % 2 == 0 && j % 2 == 0) && (i != r-2 && j != c-2){
                     *cell = BLOCK;
                   }
                }
            }
        }
        grid
    }
    
     pub fn eject_flames(&mut self, r: usize, c: usize) {
        self.cells[r][c] = EMPTY;
        for &(x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            for i in 1..= MAX_BOMB_POWER {
                let row = (r as isize + x as isize * i as isize) as usize;
                let col = (c as isize + y as isize * i as isize) as usize;
                let cell = &mut self.cells[row][col];
                if *cell > EMPTY {
                    break;
                }else if *cell == EXPLOSION{
                  self.remove_obj(row, col);
                  self.eject_flames( row, col);
                  break;
                }
                self.remove_obj(row, col);
                if i == MAX_BOMB_POWER {
                    break;
                }
            }
        }
    }

    pub fn handle_shadow(&self,i:usize,j:usize) -> f32 {
        let cell_left = self.cells[i-1][j] == WALL || self.cells[i-1][j] == BLOCK;
        let cell_top = self.cells[i][j-1] == WALL || self.cells[i][j-1] == BLOCK;
        if cell_left && cell_top {
          return FRAMES[3]; 
        }else if cell_top {
          return FRAMES[1];  
        }else if cell_left{
          return FRAMES[2];
        }
        return FRAMES[0];
    }

    pub fn render(&mut self,d:&mut RaylibDrawHandle,sheets:&Texture2D,audio:&mut RaylibAudio,sounds:&Sound,frame_time:&f32){
       let mut empty_count = 0;
      //Draw the empty dynamic shadow tile map first;
      for (i,rows) in self.cells.iter().enumerate(){
        for (j,cols) in rows.iter().enumerate(){
            let cell = cols;
            if *cell != BLOCK {
                let frame_val = self.handle_shadow(i, j);
                let local_empty = &mut self.empty_vec[empty_count];
                local_empty.rec.x = frame_val;  
                local_empty.set_position(i,j);
                local_empty.draw(sheets, d);
                empty_count += 1;
             }
        }
      }
    // Draw grass
    for grass in &mut self.grass_vec{
        grass.draw(sheets, d);
    }
    //For Dynamic Objects
       for i in 0..self.game_objs.len(){
          for j in 0..self.game_objs[i].len() {
               let obj = &mut self.game_objs[i][j]; 
               match obj {
                  GameObjs::Block(obj) => {
                    let local_block = obj;
                    local_block.draw(sheets, d);
                  },
 
                  GameObjs::Wall(obj) => {
                    let local_wall = obj;//Get the wall from vector for the current position.
                    match local_wall.state {
                      State::IDEAL => {
                        local_wall.draw(sheets, d);
                        local_wall.exploade( i, j,&self.cells);
                        }
                      State::EXPLOADING => {
                        local_wall.draw(sheets, d);
                        local_wall.update(frame_time);
                        },
                      State::EXPLOADED => {
                        self.remove_obj(i, j); 
                        },
                    }
                   } ,

                   GameObjs::Bomb(obj) => {
                    let local_bomb = obj; //Get BOMB
                    let power = local_bomb.power;
                    local_bomb.update(frame_time,audio,sounds);

                    match local_bomb.state {
                      State::IDEAL => {
                        local_bomb.draw(sheets, d);
                        local_bomb.chain_exp(i, j, &self.cells);
                      }
                      State::EXPLOADING => {
                        if self.cells[i][j] == EXPLOSION { // Wait for Explosion signal
                            local_bomb.draw_exp(sheets, d, i, j);
                            local_bomb.anim_exp(frame_time);
                        }
                         self.cells[i][j] = EXPLOSION; // Give Signal
                         self.inject_flames_obj(power, i, j);
                         self.inject_flames(power,i, j); // Injects flames consts in matrix to give signal to flame objs
                      },
                      State::EXPLOADED => {
                        self.eject_flames( i, j);
                        self.remove_obj(i, j);
                      },
                    }
                  }

                  GameObjs::FlameLeftEnd(obj) => {
                   if self.cells[i][j] == FLAME_END_LEFT{
                    obj.draw(sheets, d, i, j);
                    obj.anim(frame_time);
                    }
                  }

                  GameObjs::FlameRightEnd(obj) => {
                  if self.cells[i][j] == FLAME_END_RIGHT{
                    obj.draw(sheets, d, i, j);
                    obj.anim(frame_time);
                  }
                 }

                  GameObjs::FlameTopEnd(obj) => {
                   if self.cells[i][j] == FLAME_END_TOP{
                    obj.draw(sheets, d, i, j);
                    obj.anim(frame_time);
                  }
                 }

                  GameObjs::FlameDownEnd(obj) => {
                  if self.cells[i][j] == FLAME_END_DOWN{
                    obj.draw(sheets, d, i, j);
                    obj.anim(frame_time);
                  }
                 }

                 GameObjs::FlameLeftMid(obj) => {
                  if self.cells[i][j] == FLAME_MID_LEFT{
                    obj.draw(sheets, d, i, j);
                    obj.anim(frame_time);
                  }
                }

                  GameObjs::FlameRightMid(obj) => {
                   if self.cells[i][j] == FLAME_MID_RIGHT{
                    obj.draw(sheets, d, i, j);
                    obj.anim(frame_time);
                   }
                }

                  GameObjs::FlameTopMid(obj) => {
                  if self.cells[i][j] == FLAME_MID_TOP{
                    obj.draw(sheets, d, i, j);
                    obj.anim(frame_time);
                   }
                  }

                  GameObjs::FlameDownMid(obj) => {
                  if self.cells[i][j] == FLAME_MID_DOWN{
                    obj.draw(sheets, d, i, j);
                    obj.anim(frame_time);
                  }
                 }
                 _ => {}
               }  
            }
        }
    }

    pub fn get_collisions(&self,position:Position,obj_rec:Rectangle) -> CollisonBools {
      let (i,j) = position;
      // Collison bools
      let mut fatal_coll = false; // for Explosion,Flame and Enemies
      let mut neutral_coll = false; // For Walls and Blocks.
      let bonus_coll = false; // For Bonus Coins
      let upgrade_coll = false; // For Upgrades Bombs and Life
      let win_coll = false; // For reaching the winning place

      for r in (i)..=(i+1){
         for c in (j)..=(j+1){
             let cell = self.cells[r][c];
             let cell_x = SCALED_TILE * (r as f32);
             let cell_y = SCALED_TILE * (c as f32);

             if cell_x + SCALED_TILE > obj_rec.x && cell_x < obj_rec.x + obj_rec.width &&
                cell_y + SCALED_TILE > obj_rec.y && cell_y < obj_rec.y + obj_rec.height {

                  if cell == BLOCK || cell == WALL{
                     neutral_coll = true;  
                  }
                  
                  if cell < EMPTY {
                     fatal_coll = true;  
                  }
                }
             }
          }
      return (fatal_coll,neutral_coll,bonus_coll,upgrade_coll,win_coll);
    }        
 }
