mod grid;
mod bomb;
mod player;

use raylib::{prelude::*};
use grid::*;
use player::*;

const WIDTH:i32 = (TILE_SIZE * SCALE) * COLS;
const HEIGHT:i32 = (TILE_SIZE * SCALE) * COLS;


const BLOCK_SHEET:&str = "/home/anurag/bomber_man/src/res/block.png";
const WALLS_SHEET:&str = "/home/anurag/bomber_man/src/res/walls.png";
const PLAYER_SHEET:&str = "/home/anurag/bomber_man/src/res/player.png";
const BOMB_SHEET:&str = "/home/anurag/bomber_man/src/res/bomb.png";
const EXPLOSION_SHEET:&str = "/home/anurag/bomber_man/src/res/explosions.png";
const BACKGROUND_COLOR:Color = Color::DARKGREEN;

fn main() {
    //LOAD -->
    let (mut rl, thread) = raylib::init()
        .size(WIDTH, HEIGHT)
        .title("BOMBER-MAN")
        .build();

    //ALl textures
    let block_texture = rl.load_texture(&thread, BLOCK_SHEET).unwrap();
    let walls_texture = rl.load_texture(&thread, WALLS_SHEET).unwrap();
    let player_texture= rl.load_texture(&thread, PLAYER_SHEET).unwrap();
    let bomb_texture= rl.load_texture(&thread, BOMB_SHEET).unwrap();
    let explosion_texture = rl.load_texture(&thread, EXPLOSION_SHEET).unwrap();
    
    //All structs
    let mut grid = Grid::new();
    let mut player = Player::new();

    while !rl.window_should_close() {
        //UPDATE --> 
        let collision = grid.detect_collision(player.vec2.x as i32, player.vec2.y as i32);
        let frame_time = rl.get_frame_time();
        player.control(&mut rl, &collision,&mut grid);
        
        //DRAW -->
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(BACKGROUND_COLOR);
        grid.draw(&mut d, &block_texture,&walls_texture,&bomb_texture,&explosion_texture,&frame_time);
        player.draw(&mut d, &player_texture, &frame_time);
       // d.draw_texture_rec(&explosion_texture, Rectangle::new(200_f32,0_f32,200_f32,200_f32), Vector2::new(-20_f32,-20_f32),MAIN_COLOR );
    }   
     
}
