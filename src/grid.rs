use raylib::{prelude::*};
use crate::bomb::*;

pub const ROWS:i32 = 13;
pub const COLS:i32 = 15;

pub const TILE_SIZE:i32 = 16;
pub const SCALE:i32 = 3;
pub const MARGIN:i32 = 8;
pub const MAIN_COLOR:Color = Color::WHITE;
pub const MAX_FRAME:i32 = 3;
pub const MAX_WALL_FRAMES:i32 = 7;
pub const ANIM_DURATION:f32 = 0.2_f32;

//dumb enum values.
pub const EMPTY:i32 = 0;
pub const BLOCK:i32 = 1;
pub const WALL:i32 = 2;
pub const BOMB:i32 = 3;
pub const EXPLOSION:i32 = 4;
pub const FLAME:i32 = 5;

#[derive(PartialEq,Clone)]
pub enum State{
    IDEAL,
    EXPLOADING,
    EXPLOADED,
}

#[derive(Clone)]
pub struct Wall {
    pub vec2:Vector2,
    pub rec:Rectangle,
    pub frames:i32,
    pub time:f32,
    pub state:State,
}

impl Wall {
    pub fn new() -> Self {
      let vec2 = Vector2::new(0_f32,0_f32);
      let rec =  Rectangle::new(0_f32, 0_f32, (SCALE*TILE_SIZE) as f32, (SCALE*TILE_SIZE) as f32);
      let frames = 0;
      let time = 0_f32;
      let state = State::IDEAL;
      Self { vec2, rec, frames, time, state}
    }

    pub fn set_position(&mut self,x:i32,y:i32){
       self.vec2.x = x as f32;
       self.vec2.y = y as f32;
    }
    pub fn set_frame(&mut self){
        self.rec.x = (TILE_SIZE * SCALE) as f32 * self.frames as f32;
      }

    pub fn remove_wall(&mut self,i:usize,j:usize,cells:&[[i32;15];13]){
         if cells[i+1][j] == EXPLOSION && self.state == State::IDEAL{
            self.state = State::EXPLOADING;
            }else if cells[i-1][j] == EXPLOSION &&  self.state == State::IDEAL {
            self.state = State::EXPLOADING;
            }else if cells[i][j+1] == EXPLOSION &&  self.state == State::IDEAL {
            self.state = State::EXPLOADING;
            }else if cells[i][j-1] == EXPLOSION &&  self.state == State::IDEAL {
            self.state = State::EXPLOADING;
          }
          
         }
    
    pub fn animate(&mut self,frame_time:&f32){
       if self.state == State::EXPLOADING{// Start exploading animation if exploading true.
        if self.time > ANIM_DURATION{
            self.time = 0_f32;
            self.frames += 1;
        }
        self.time += *frame_time;
        self.set_frame();

        if self.frames == 6 { //In the last frame set exploaded true.
          self.state = State::EXPLOADED;
        }
        self.frames = self.frames  % (MAX_WALL_FRAMES - 1);
     }else{ //Or set frame to default.
        self.frames = 0;
        self.set_frame();
     }
     
    }
}

pub struct Grid {
    cells: [[i32;COLS as usize];ROWS as usize],
    wall_vec:Vec<Wall>,
    pub bomb_vec:Vec<Bomb>
}

impl Grid {
    //15x13 grid.
    pub fn new() -> Self  {
        let cells = [
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 0, 1, 2, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 0, 0, 2, 2, 2, 2, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 0, 1, 2, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 2, 0, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 1],
            [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1],
            [1, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 2, 1, 0, 1],
            [1, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
        ];
        let mut wall_vec:Vec<Wall> = vec![];
        let mut bomb_vec:Vec<Bomb> = vec![];

        for i in 0..cells.len(){
            for j in 0..cells[i].len(){
                if cells[i][j] == WALL {
                    let wall = Wall::new();
                    wall_vec.push(wall);
                }
                if cells[i][j] == BOMB {
                    let bomb = Bomb::new();
                    bomb_vec.push(bomb);
                }
            }
        }
        return Self {cells,wall_vec,bomb_vec};
    }

    pub fn get_cell(&self, row: i32, column: i32) -> i32 {
        self.cells[row as usize][column as usize]
    }

    pub fn set_cell(&mut self, row: i32, column: i32, value: i32) {
        self.cells[row as usize][column as usize] = value;
    }

    pub fn draw(&mut self,d:&mut RaylibDrawHandle,block_texture:&Texture2D,wall_texture:&Texture2D,bomb_texture:&Texture2D,explosion_texture:&Texture2D,frame_time:&f32){
        let mut wall_count = 0;//walll index.
        let mut bomb_count = 0;//bomb index.
        //Iterate over the grid and get vals.
        for i in 0..ROWS{
            for j in 0..COLS{

                if self.get_cell(i, j) == BOMB || self.get_cell(i, j) == EXPLOSION{
                    let local_bomb = &mut self.bomb_vec[bomb_count]; //Get BOMB
                    if local_bomb.state == State::IDEAL{
                        local_bomb.set_position(i*TILE_SIZE*SCALE, j*TILE_SIZE*SCALE); //Set position using tile_size
                    d.draw_texture_rec(bomb_texture, local_bomb.rec, local_bomb.vec2, MAIN_COLOR); //Draw bomb when exploading is false
                    }else if local_bomb.state == State::EXPLOADING {
                    self.cells[i as usize][j as usize] = EXPLOSION;
                    local_bomb.set_position_exp(local_bomb.vec2.x - ((EXPLOSION_TILE/2_f32)  - EXPLOSION_MARGIN), local_bomb.vec2.y  - ((EXPLOSION_TILE/2_f32) - EXPLOSION_MARGIN)); //Set position using explpsion_tile_size
                    d.draw_texture_rec(explosion_texture, local_bomb.exp_rec, local_bomb.exp_vec2, MAIN_COLOR);//Draw explosion when exploading is true
                    }
                    local_bomb.animate(frame_time);//Animate the bomb.
    
                     if local_bomb.state == State::EXPLOADED{ //Remove Bomb if its exploaded.
                        self.bomb_vec.remove(bomb_count);//Remove bomb  vector.
                        self.set_cell(i, j, EMPTY);//Remove the bomb from grid.
                    }else{
                    bomb_count += 1;//Or look other bomb
                     }
                   } 

                if self.get_cell(i, j) == BLOCK{
                    d.draw_texture(&block_texture, i*TILE_SIZE*SCALE, j*TILE_SIZE*SCALE, MAIN_COLOR) // Simply draw static block object.
                   }
               if self.get_cell(i, j) == WALL{

                {// get wall mutable refrence in a private scope
                 let local_wall = &mut self.wall_vec[wall_count];//Get the wall from vector for the current position.
                 local_wall.set_position(i*TILE_SIZE*SCALE, j*TILE_SIZE*SCALE);//Set position for wall.
                 d.draw_texture_rec(wall_texture, local_wall.rec, local_wall.vec2, MAIN_COLOR);//draw static wall objects
                 local_wall.animate(frame_time);//Animate the wall if the exploading is true.
                 local_wall.remove_wall( i as usize, j as usize,&self.cells);//Remove exploaded walls.
               }//local wall will be drop after this scope

                if self.wall_vec[wall_count].state == State::EXPLOADED{ //Remove the exploaded wall from the grid and vector.
                   self.set_cell(i, j, EMPTY);//Remove from grid.
                   self.wall_vec.remove(wall_count);//Remove from vector.
                 }else{
                    wall_count += 1;//Or look for another wall
                 }
               }
             
            }
        }
    }

    pub fn detect_collision(&self, player_x: i32, player_y: i32) -> bool {
        //Set player height width with a margin.
        let player_height = (TILE_SIZE * SCALE) - MARGIN; 
        let player_width = (TILE_SIZE * SCALE) - MARGIN;
        //Iterate over grid check collison using collison formula.
        for i in 0..ROWS {
            for j in 0..COLS {
                if self.get_cell(i, j) == WALL || self.get_cell(i, j) == BLOCK {
                    let cell_x = i * (TILE_SIZE * SCALE) - MARGIN/2;
                    let cell_y = j * (TILE_SIZE * SCALE) - MARGIN/2;
                    let cell_width = (TILE_SIZE * SCALE) - MARGIN/2;
                    let cell_height = (TILE_SIZE * SCALE) - MARGIN/2;
                    if player_x + player_width > cell_x && player_x < cell_x + cell_width &&
                       player_y + player_height > cell_y && player_y < cell_y + cell_height {
                        return true;
                    }
                }
            }
        }
        false
    }
}
