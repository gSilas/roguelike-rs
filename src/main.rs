extern crate tcod;

use tcod::console::*;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode::*;

// const window modifiers
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

// main loop
fn main() {

    // set up window
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("libtcod-rs Roguelike")
        .init();

    // limits loop times
    tcod::system::set_fps(LIMIT_FPS);

    // player location
    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    // draw loop
    while !root.window_closed() {
        root.set_default_foreground(colors::WHITE);
        root.put_char(player_x, player_y, '@', BackgroundFlag::None);
        root.flush();
        root.wait_for_keypress(true);

        // using input handling
        root.put_char(player_x, player_y, ' ', BackgroundFlag::None);
        let exit = handle_keys(&mut root, &mut player_x, &mut player_y);
        if exit {
            break;
        }
    }
}

// Input handling
fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Up, .. } => *player_y -= 1,
        Key { code: Down, .. } => *player_y += 1,
        Key { code: Left, .. } => *player_x -= 1,
        Key { code: Right, .. } => *player_x += 1,
        // alt+enter = fullscreen
        Key { code: Enter, alt: true, .. } => {
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        // esc = exit
        Key { code: Escape, .. } => return true,
        _ => {}
    }
    false
}
