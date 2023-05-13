use std::vec;
use raylib::prelude::*;
use crate::objects::*;
use crate::bomb::*;
use noise::{NoiseFn, Perlin};

pub const TILE_SIZE:f32 = 16_f32;
pub const SCALE:f32 = 3_f32;

pub const MAX_FRAME:usize = 3;
pub const ANIM_DURATION:f32 = 0.22_f32;
pub const MAX_RAND_FRAME:usize = 15;

//frames const
pub const O:f32 = 0_f32;
pub const FRAMES:[f32;8] = [0.0,16.0,32.0,48.0,64.0,80.0,96.0,108.0];

//dumb enum values.
pub const EMPTY:i8 = 0;
pub const BLOCK:i8 = 1;
pub const WALL:i8 = 2;
pub const BOMB:i8 = 3;

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

pub trait Obj {
   fn set_position(&mut self, x: f32, y: f32);   
}

pub trait DObj {
   fn animate(&mut self,frame_time:&f32);
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

impl_inject_flames!(Grid,inject_flames3,EXPLOSION,EMPTY,
    FLAME_MID_LEFT,FLAME_END_LEFT,FLAME_MID_RIGHT,
    FLAME_END_RIGHT,FLAME_MID_TOP,FLAME_END_TOP,
    FLAME_MID_DOWN,FLAME_END_DOWN,cells);
    
impl_inject_flames!(Grid,inject_flames_obj4,BOMB,GameObjs::Default, 
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
                    let wall = Wall::new();
                    game_objs[i][j] = GameObjs::Wall(wall);
                }
                if curr_cell == BOMB {
                    let bomb = Bomb::new();
                    game_objs[i][j] = GameObjs::Bomb(bomb);
                }
                if curr_cell == BLOCK{
                    let block = Block::new();
                    game_objs[i][j] = GameObjs::Block(block);
                }
                if curr_cell == EMPTY{
                let mut grass = Grass::new();
                grass.set_position(i as f32 * TILE_SIZE * SCALE, j as f32 * TILE_SIZE * SCALE);
                grass_vec.push(grass);
                }
                let empty = Empty::new();
                empty_vec.push(empty);
            }
        }
        return Self {cells,empty_vec,grass_vec,game_objs};
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
    

    pub fn get_cell(&self, row: usize, column: usize) -> i8 {
        self.cells[row][column]
    }

    pub fn set_cell(&mut self, row: usize, column: usize, value: i8) {
        self.cells[row][column] = value;
    }

    pub fn rm_obj(&mut self,row:usize,column: usize){
        self.game_objs[row][column] = GameObjs::Default;
    }

     pub fn eject_flames2(&mut self, l: usize, r: usize, c: usize) {
        self.cells[r][c] = EMPTY;
        for &(x, y) in [(1, 0), (-1, 0), (0, 1), (0, -1)].iter() {
            for i in 1..=l {
                let row = (r as isize + x as isize * i as isize) as usize;
                let col = (c as isize + y as isize * i as isize) as usize;
                let obj = &mut self.game_objs[row][col];
                let cell = &mut self.cells[row][col];
    
                if *cell >= EMPTY {
                    break;
                }
    
                *cell = EMPTY;
                *obj = GameObjs::Default;
    
                if i == l {
                    break;
                }
            }
        }
    }
 
    pub fn render(&mut self,d:&mut RaylibDrawHandle,sheets:&Texture2D,audio:&mut RaylibAudio,sounds:&Sound,frame_time:&f32){
       let mut empty_count = 0;
     
    //Draw the empty dynamic shadow tile map first;
      for (i,rows) in self.cells.iter().enumerate(){
        for (j,cols) in rows.iter().enumerate(){
            let curr_cell = cols;
            if *curr_cell != BLOCK{
                let curr_cell_left = self.cells[i-1][j] == WALL || self.cells[i-1][j] == BLOCK;
                let curr_cell_top = self.cells[i][j-1] == WALL || self.cells[i][j-1] == BLOCK;
                let local_empty = &mut self.empty_vec[empty_count];
                if curr_cell_left && curr_cell_top{local_empty.rec.x = FRAMES[3];
                }else if curr_cell_top{local_empty.rec.x = FRAMES[1]; 
                }else  if curr_cell_left{local_empty.rec.x = FRAMES[2];
                }else{ local_empty.rec.x = FRAMES[0];}
                
               local_empty.set_position(i as f32 * TILE_SIZE*SCALE, j as f32 * TILE_SIZE*SCALE);
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
                   local_block.set_position(i as f32 * TILE_SIZE*SCALE, j as f32*TILE_SIZE*SCALE);
                   local_block.draw(sheets, d);
                  },
 
                  GameObjs::Wall(obj) => {
                    let local_wall = obj;//Get the wall from vector for the current position.
                    local_wall.set_position(i as f32 *TILE_SIZE*SCALE, j as f32 *TILE_SIZE*SCALE);//Set position for wall.
                    local_wall.draw(sheets, d);
                    local_wall.animate(frame_time);//Animate the wall if the exploading is true.
                    local_wall.expload_wall( i, j,&self.cells);//Remove exploaded walls.
   
                     if local_wall.state == State::EXPLOADED{ //Remove the exploaded wall from the grid and vector.
                        self.rm_obj(i, j);
                        self.set_cell(i, j, EMPTY);//Remove from grid.  
                    }
                   },

                   GameObjs::Bomb(obj) => {
                    let local_bomb = obj; //Get BOMB
                    let power = local_bomb.power;
                    if local_bomb.state == State::IDEAL{
                    local_bomb.set_position(i as f32 * TILE_SIZE*SCALE, j as f32 * TILE_SIZE*SCALE); //Set position using tile_size
                    local_bomb.draw(sheets, d);
                    local_bomb.chain_exp(i, j, &self.cells);
                    local_bomb.play_sound(audio, sounds);
                    }else if local_bomb.state == State::EXPLOADING {
                    if self.cells[i][j] == EXPLOSION { // Wait for Explosion signal
                    local_bomb.draw_exp(sheets, d, i, j);
                    local_bomb.anim_exp(frame_time);
                    }
                    self.cells[i][j] = EXPLOSION; // Give Signal
                    }
                    local_bomb.animate(frame_time);//Animate the bomb.
                    local_bomb.explode();
                     if local_bomb.state == State::EXPLOADED{ //Remove Bomb if its exploaded.
                        self.rm_obj(i, j);
                        self.eject_flames2(power,i, j);        
                    }
                    self.inject_flames3(power,i, j); // Gives Signal to all flames objs to expload through matrix
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
 }
