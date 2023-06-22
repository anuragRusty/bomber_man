mod grid;
mod bomb;
mod game;
mod bonus;
mod noise;
mod player;
mod objects;
mod upgrade;

use raylib::prelude::*;
use raylib::core::audio::Sound;
use grid::*;
use game::*;

const SPRITE_SHEET:&str = "assets/spritesheet.png";

const EXP_SOUND:&str = "assets/sounds/exp.ogg";
const BONUS_SOUND:&str = "assets/sounds/bonus.ogg";
const UPGRADE_SOUND:&str = "assets/sounds/upgrade.ogg";
const GAMEOVER_SOUND:&str = "assets/sounds/gameover.ogg";
const WIN_SOUND:&str = "assets/sounds/win.ogg";
const BURNING_SOUND:&str = "assets/sounds/burning.ogg";
const PUNCH_SOUND:&str = "assets/sounds/punch.ogg";

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
    let bonus_sound = Sound::load_sound(BONUS_SOUND).unwrap();
    let gameover = Sound::load_sound(GAMEOVER_SOUND).unwrap();
    let upgrade_sound = Sound::load_sound(UPGRADE_SOUND).unwrap();
    let win_sound = Sound::load_sound(WIN_SOUND).unwrap();
    let burning_sound = Sound::load_sound(BURNING_SOUND).unwrap();
    let punch = Sound::load_sound(PUNCH_SOUND).unwrap();

    let sounds:GameSounds = (&exp_sound,&bonus_sound,&gameover,&upgrade_sound,&win_sound,&burning_sound,&punch);

    while !rl.window_should_close() {
        //rl.toggle_fullscreen();
        //UPDATE --> 
        let frame_time = rl.get_frame_time();
        game.update(&mut rl,&mut audio,sounds,frame_time);
    
        //Render Sound and Graphics -->
        let mut d = rl.begin_drawing(&thread);
        game.draw(&mut d, &sheets ,frame_time);
    }        
}
