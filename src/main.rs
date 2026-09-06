mod textures;
mod card;

use raylib::{self, init};

const WINDOW_TITLE: &str = "StackOverflow";
const SCREEN_WIDTH: i32 = 1;
const SCREEN_HEIGHT: i32 = 1;

pub fn main() {
    let (mut rl, thread) = init()
        .resizable()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title(WINDOW_TITLE)
        .build();

    rl.set_window_min_size(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
    rl.set_exit_key(None);
    rl.set_target_fps(60);

    let textures = textures::Textures::new(&mut rl, &thread);
    // TODO: Audio
    // TODO: I18n
    

    // NOTE: don't ever rewrite a project from Avalzzzz, I mean look at the `Game` struct in main.c :'(
}
