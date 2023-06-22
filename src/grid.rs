use raylib::prelude::*;
use crate::objects::*;
use crate::bomb::*;
use crate::noise::*;
use crate::bonus::*;
use crate::upgrade::*;

const ROWS:usize = 32;
const COLS:usize = 18;

pub type GameSounds<'a> = (&'a Sound, &'a Sound, &'a Sound, &'a Sound, &'a Sound, &'a Sound,&'a Sound);
pub type CollisonBools = (bool,bool,bool,bool,bool);
pub type Position = (usize,usize);

pub const TILE_SIZE:f32 = 16_f32;
pub const SCALE:f32 = 3_f32;
pub const SCALED_TILE:f32 = TILE_SIZE*SCALE;
pub const MARGIN_POS:f32 = TILE_SIZE/8_f32*SCALE;

pub const MAX_FRAME:usize = 3;
pub const ANIM_DURATION:f32 = 0.22_f32;
pub const MAX_RAND_FRAME:usize = 15;
//frames const
pub const O:f32 = 0_f32;
pub const FRAMES:[f32;14] = [0.0,16.0,32.0,48.0,64.0,80.0,96.0,112.0,128.0,144.0,160.0,172.0,188.0,204.0];
//sub neg mid and end flamns enum values

#[derive(PartialEq,Clone,Debug,Copy)] // Seprate the flamne objs and add bonus and upgrade objs and create a new accumulator for flame obj
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

pub struct Grid {
  pub empty_vec:Vec<Empty>,
  pub grass_vec:Vec<Grass>,
  pub bonus_vec:Vec<Bonus>,
  pub upgrade_vec:Vec<Upgrade>,
  pub cells:Vec<Vec<i8>>,
  pub game_objs:Vec<Vec<GameObjs>>,
}

impl Grid {
    pub fn new() -> Self  {
        let mut cells = noise(ROWS, COLS);
        let mut empty_vec:Vec<Empty> = vec![];
        let mut grass_vec:Vec<Grass> = vec![];
        let mut bonus_vec:Vec<Bonus> = vec![];
        let mut upgrade_vec:Vec<Upgrade> = vec![];
        let mut game_objs:Vec<Vec<GameObjs>> = vec![vec![GameObjs::Default;COLS]; ROWS];

        for (i,rows) in cells.iter_mut().enumerate(){
          for (j,cell) in rows.iter_mut().enumerate(){
                if *cell == WALL {
                    let wall = Wall::new(i,j,SCALE);
                    game_objs[i][j] = GameObjs::Wall(wall);
                }else if *cell == BLOCK{
                    let block = Block::new(i,j,SCALE);
                    game_objs[i][j] = GameObjs::Block(block);
                }else if *cell == EMPTY{
                    let grass = Grass::new(i,j,SCALE);
                    grass_vec.push(grass);
                }else if *cell == HEART {
                   let bonus = Bonus::new(BonusType::Heart, i, j, SCALE);
                   bonus_vec.push(bonus);
                   *cell = EMPTY;
                }else if *cell == CASH {
                  let bonus = Bonus::new(BonusType::Cash, i, j, SCALE);
                  bonus_vec.push(bonus);
                  *cell = EMPTY;
               }else if *cell == SILVER_COIN {
                   let bonus = Bonus::new(BonusType::SilverCoin, i, j, SCALE);
                   bonus_vec.push(bonus);
                  *cell = EMPTY;
               }else if *cell == GOLD_COIN {
                  let bonus = Bonus::new(BonusType::GoldCoin, i, j, SCALE);
                  bonus_vec.push(bonus);
                  *cell = EMPTY;
               }else if *cell == DIAMOND {
                  let bonus = Bonus::new(BonusType::Diamond, i, j, SCALE);
                  bonus_vec.push(bonus);
                  *cell = EMPTY;
               }else if *cell == BOMB_BLACK_2X {
                   let upgrade = Upgrade::new(UpgradeType::BlackBomb,2, i, j, SCALE);
                   upgrade_vec.push(upgrade);
                  *cell = EMPTY;
               }else if *cell == BOMB_BLACK_3X {
                   let upgrade = Upgrade::new(UpgradeType::BlackBomb,3, i, j, SCALE);
                   upgrade_vec.push(upgrade);
                  *cell = EMPTY;
               }else if *cell == BOMB_BLUE_2X {
                  let upgrade = Upgrade::new(UpgradeType::BlueBomb,2, i, j, SCALE);
                  upgrade_vec.push(upgrade);
                 *cell = EMPTY;
                }else if *cell == BOMB_BLUE_3X {
                  let upgrade = Upgrade::new(UpgradeType::BlueBomb,3, i, j, SCALE);
                  upgrade_vec.push(upgrade);
                 *cell = EMPTY;
                }else if *cell == BOMB_PURPLE_2X {
                  let upgrade = Upgrade::new(UpgradeType::PurpleBomb,2, i, j, SCALE);
                  upgrade_vec.push(upgrade);
                  *cell = EMPTY;
               }else if *cell == BOMB_PURPLE_3X {
                  let upgrade = Upgrade::new(UpgradeType::PurpleBomb,3, i, j, SCALE);
                  upgrade_vec.push(upgrade);
                 *cell = EMPTY;
               }else if *cell == BOMB_RED_2X {
                  let upgrade = Upgrade::new(UpgradeType::RedBomb,2, i, j, SCALE);
                  upgrade_vec.push(upgrade);
                 *cell = EMPTY;
                }else if *cell == BOMB_RED_3X {
                  let upgrade = Upgrade::new(UpgradeType::RedBomb,3, i, j, SCALE);
                  upgrade_vec.push(upgrade);
                 *cell = EMPTY;
                }else if *cell == WIN_CELL {
                  *cell = EMPTY;
                }
                let empty = Empty::new(i,j,SCALE);
                empty_vec.push(empty);
            }
        }
        return Self {empty_vec,bonus_vec,upgrade_vec,grass_vec,cells,game_objs};
    }

    pub fn rm_game_obj(&mut self,i:usize,j:usize){
        self.cells[i][j] = EMPTY;
        self.game_objs[i][j] = GameObjs::Default;  
    }

    pub fn rm_bonus_obj(&mut self,i:usize){
      self.bonus_vec[i].bonus_type = BonusType::Default;
    }

    pub fn rm_upgrade_obj(&mut self,i:usize){
      self.upgrade_vec[i].up_type = UpgradeType::Default;
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
                  self.rm_game_obj(row, col);
                  self.eject_flames( row, col);
                  break;
                }
                self.rm_game_obj(row, col);
                if i == MAX_BOMB_POWER {
                    break;
                }
            }
        }
    }

    fn get_shadow_val(&self,i:usize,j:usize) -> f32 {
        let left = self.cells[i-1][j] == WALL || self.cells[i-1][j] == BLOCK;
        let top = self.cells[i][j-1] == WALL || self.cells[i][j-1] == BLOCK;
        let top_left = self.cells[i-1][j-1] == WALL || self.cells[i-1][j-1] == BLOCK;

        if !left && !top && !top_left {
          return FRAMES[0]; 
        }else if left && top {
          return FRAMES[4]; 
        }else if top {
          return FRAMES[2];  
        }else if left{
          return FRAMES[3];
        }
        return FRAMES[1];
    }

    pub fn draw(&mut self,d:&mut RaylibDrawHandle,sheets:&Texture2D){
       let mut empty_count = 0;
    // Draw the empty dynamic shadow tile map first;
      for (i,rows) in self.cells.iter().enumerate(){
        for (j,cell) in rows.iter().enumerate(){
            if *cell != BLOCK {
                let frame_val = self.get_shadow_val(i, j);
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
    // Draw Bonus
    for bonus in &mut self.bonus_vec{
        bonus.draw(sheets, d);
    }
     // Draw Upgrades
    for upgrade in &mut self.upgrade_vec{
        upgrade.draw(sheets, d);
    }
    //For Dynamic Objects
       for (i,rows) in self.game_objs.iter_mut().enumerate(){
          for (j,obj) in rows.iter_mut().enumerate() {
               let cell = self.cells[i][j];
               match obj {
                  GameObjs::Block(obj) => {obj.draw(sheets, d);},
                  GameObjs::Wall(obj) => {
                    let local_wall = obj;//Get the wall from vector for the current position.
                    match local_wall.state {
                      State::IDEAL => {local_wall.draw(sheets, d);}
                      State::EXPLOADING => {local_wall.draw(sheets, d);},
                      _ => {},
                    }
                   },
                   GameObjs::Bomb(obj) => {
                    let local_bomb = obj; //Get BOMB
                    match local_bomb.state {
                      State::IDEAL => {local_bomb.draw(sheets, d);}
                      State::EXPLOADING => {local_bomb.draw_exp(sheets, d, i, j,cell);},
                      _ => {}
                    }
                  }
                  GameObjs::FlameLeftEnd(obj) => {obj.draw(sheets, d, i, j,cell);}
                  GameObjs::FlameRightEnd(obj) => {obj.draw(sheets, d, i, j,cell);}
                  GameObjs::FlameTopEnd(obj) => {obj.draw(sheets, d, i, j,cell);}
                  GameObjs::FlameDownEnd(obj) => {obj.draw(sheets, d, i, j,cell);}
                  GameObjs::FlameLeftMid(obj) => {obj.draw(sheets, d, i, j,cell);}
                  GameObjs::FlameRightMid(obj) => {obj.draw(sheets, d, i, j,cell);}
                  GameObjs::FlameTopMid(obj) => {obj.draw(sheets, d, i, j,cell);}
                  GameObjs::FlameDownMid(obj) => {obj.draw(sheets, d, i, j,cell);}
                  _ => {}
               }  
            }
          }
    }

    pub fn update(&mut self,audio:&mut RaylibAudio,exp_sound:&Sound,frame_time:f32){
      // Update Bonus objects
      for bonus in &mut self.bonus_vec{
        bonus.animate(frame_time);
      }
      // Update dynamic objects
      for i in 0..self.game_objs.len(){
        for j in 0..self.game_objs[i].len() {
             let obj = &mut self.game_objs[i][j];
             let cell = self.cells[i][j]; 
             match obj {
                GameObjs::Wall(obj) => {
                  let local_wall = obj;//Get the wall from vector for the current position.
                  match local_wall.state {
                    State::IDEAL => {local_wall.exploade( i, j,&self.cells);}
                    State::EXPLOADING => {local_wall.update(frame_time);},
                    State::EXPLOADED => {self.rm_game_obj(i, j);},
                  }
                 } ,
                 GameObjs::Bomb(obj) => {
                  let local_bomb = obj; //Get BOMB
                  let power = local_bomb.power;
                  local_bomb.update(frame_time,audio,exp_sound);
                  match local_bomb.state {
                    State::IDEAL => {local_bomb.chain_exp(i, j, &self.cells);}
                    State::EXPLOADING => {
                       local_bomb.anim_exp(cell,frame_time);
                       self.cells[i][j] = EXPLOSION; // Give Signal
                       self.inject_flames_obj(power, i, j);// Inject Flame obj 
                       self.inject_flames(power,i, j); // Injects flames consts in matrix to give signal to flame objs
                    },
                    State::EXPLOADED => {self.eject_flames( i, j);self.rm_game_obj(i, j);},
                  }
                }
               GameObjs::FlameLeftEnd(obj) => {obj.anim(cell,frame_time);}
               GameObjs::FlameRightEnd(obj) => {obj.anim(cell,frame_time);}
               GameObjs::FlameTopEnd(obj) => {obj.anim(cell,frame_time);}
               GameObjs::FlameDownEnd(obj) => {obj.anim(cell,frame_time);}
               GameObjs::FlameLeftMid(obj) => {obj.anim(cell,frame_time);}
               GameObjs::FlameRightMid(obj) => {obj.anim(cell,frame_time);}
               GameObjs::FlameTopMid(obj) => {obj.anim(cell,frame_time);}
               GameObjs::FlameDownMid(obj) => {obj.anim(cell,frame_time);}
               _ => {}
             }  
          }
      }
    }        
 }
