use noise::{NoiseFn, Perlin};

use crate::BONUS_SOUND;
//dumb enum values.
pub const EMPTY:i8 = 0;
pub const BOMB:i8 = 1;
pub const BLOCK:i8 = 2;
pub const WALL:i8 = 3;
pub const WIN_CELL:i8 = 100;
// bonus enum values 
pub const BOMB_BLUE_2X:i8 = 4;
pub const BOMB_RED_2X:i8 = 5;
pub const BOMB_BLUE_3X:i8 = 6;
pub const BOMB_RED_3X:i8 = 7;

pub const BOMB_BLACK_2X:i8 = 4;
pub const BOMB_PURPLE_2X:i8 = 5;
pub const BOMB_BLACK_3X:i8 = 6;
pub const BOMB_PURPLE_3X:i8 = 7;

pub const HEART:i8 = 8;
pub const SILVER_COIN:i8 = 9;
pub const GOLD_COIN:i8 = 10;
pub const DIAMOND:i8 = 11;
pub const CASH:i8 = 12;

const MAP_SEED:u32 = 120727;

pub fn noise(r: usize, c: usize) -> Vec<Vec<i8>> {
    let mut grid = vec![vec![0; c]; r];
    let perlin = Perlin::new(MAP_SEED);
    let threshold = 0.02; // controls the density of solid blocks
    let cash_threshold = 0.0002;
    for i in 0..r {
        for j in 0..c {
            let cell = &mut grid[i][j];
            if i == 0 || j == 0 || i == r - 1 || j == c - 1 {
                *cell = BLOCK;
            }else {
                let x = i as f64 / r as f64;
                let y = j as f64 / c as f64;
                let noise = perlin.get([x * 10.0, y * 10.0]); // adjust the frequency of noise
                
                if noise > cash_threshold{
                  *cell = BOMB_BLACK_2X;
                }
                if noise >= threshold {
                  *cell = WALL;
                }
               if (i % 2 == 0 && j % 2 == 0) && (i != r-2 && j != c-2){
                 *cell = BLOCK;
               }
              
            }
        }
    }
    return grid;
}