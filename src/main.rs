extern crate tcod;

use tcod::console::*;
use tcod::colors::{self, Color};

// const window modifiers
const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 60;

// basic game Object
struct Object {
    x: i32,
    y: i32,
    char: char,
    color: Color,
}

// implementaion of game Object
impl Object {
    // new creates gaem Object
    pub fn new(x: i32, y: i32, char: char, color: Color) -> Self {
        Object {
            x: x,
            y: y,
            char: char,
            color: color,
        }
    }

    // move by dx and dy
    pub fn move_by(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    // set color and draw character
    pub fn draw(&self, con: &mut Console) {
        con.set_default_foreground(self.color);
        con.put_char(self.x, self.y, self.char, BackgroundFlag::None);
    }

    // erase character
    pub fn clear(&self, con: &mut Console) {
        con.put_char(self.x, self.y, ' ', BackgroundFlag::None);
    }
}

// main loop
fn main() {

    // set up window
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("libtcod-rs Roguelike")
        .init();

    // offscreen console
    let mut con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    // limits loop times
    tcod::system::set_fps(LIMIT_FPS);

    // player
    let player = Object::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2, '@', colors::WHITE);

    // NPC
    let npc = Object::new(SCREEN_WIDTH / 2 - 5, SCREEN_HEIGHT / 2, '@', colors::YELLOW);

    // list of those two
    let mut objects = [player, npc];


    // main loop
    while !root.window_closed() {
        // draw all objects in the list
        for object in &objects {
            object.draw(&mut con);
        }

        // blit the contents of "con" to the root console and present it
        blit(&mut con,
             (0, 0),
             (SCREEN_WIDTH, SCREEN_HEIGHT),
             &mut root,
             (0, 0),
             1.0,
             1.0);
        root.flush();

        // erase all objects at their old locations, before they move
        for object in &objects {
            object.clear(&mut con)
        }

        // handle keys and exit game if needed
        let player = &mut objects[0];
        let exit = handle_keys(&mut root, player);
        if exit {
            break;
        }
    }
}

// Input handling
fn handle_keys(root: &mut Root, player: &mut Object) -> bool {
    use tcod::input::Key;
    use tcod::input::KeyCode::*;

    let key = root.wait_for_keypress(true);
    match key {
        Key { code: Enter, alt: true, .. } => {
            // Alt+Enter: toggle fullscreen
            let fullscreen = root.is_fullscreen();
            root.set_fullscreen(!fullscreen);
        }
        Key { code: Escape, .. } => return true,  // exit game

        // movement keys
        Key { code: Up, .. } => player.move_by(0, -1),
        Key { code: Down, .. } => player.move_by(0, 1),
        Key { code: Left, .. } => player.move_by(-1, 0),
        Key { code: Right, .. } => player.move_by(1, 0),

        _ => {}
    }

    false
}
