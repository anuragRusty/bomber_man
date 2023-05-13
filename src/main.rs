mod grid;
mod bomb;
mod game;
mod player;
mod objects;

use raylib::prelude::*;
use raylib::core::audio::Sound;
use grid::*;
use game::*;

const SPRITE_SHEET:&str = "/home/anurag/bomber_man/src/res/sheets5.png";
const EXP_SOUND:&str = "/home/anurag/bomber_man/src/res/exp.ogg";

//sudo ifconfig wlp3s0 down sudo ifconfig wlp3s0 hw ether 70:A6:CC:AF:A4:15
fn main() {    
    //LOAD -->
    let mut game = Game::new();
    let (mut rl, thread) = raylib::init()
        .size(game.screen_w, game.screen_h)
        .title("BOMBER-MAN")
        .build();

    //ALl textures Assets
    let sheets = rl.load_texture(&thread, SPRITE_SHEET).unwrap();   
    // All Sounds Assests
    let mut audio = RaylibAudio::init_audio_device();
    let exp_sound = Sound::load_sound(EXP_SOUND).unwrap();

    while !rl.window_should_close() {
        //UPDATE --> 
        let frame_time = rl.get_frame_time();
        game.update(&mut rl,&frame_time);

        //Render Sound and Graphics -->
        let mut d = rl.begin_drawing(&thread);
        game.render(&mut d, &sheets ,&mut audio,&exp_sound,&frame_time);
    }        
}