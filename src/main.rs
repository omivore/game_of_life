extern crate tcod;
extern crate time;
use tcod::console::{Console, Root, BackgroundFlag};
use tcod::input::{KeyCode, KEY_PRESSED};
use tcod::colors::Color;
use time::{now, Duration};

fn main() {
    let mut root = Root::initializer()
        .size(80, 50)
        .title("Conway's Game of Life")
        .fullscreen(false)
        .init();
    
    let mut running = false;
    let mut execute = false;
    let mut tabbing: Option<i32> = None;
    let mut cursor = (40, 25);
    let mut state: [[bool; 45]; 79] = [[false; 45]; 79];
    let mut last_iter = now() - Duration::seconds(1);   // Pushed back to ensure execution the first time.
    
    let bg_color = Color::new(0, 0, 0);
    let cursor_color = Color::new(255, 225, 255);
    let live_cell_color = Color::new(10, 215, 70);
    let tabbity_colors = (Color::new(65, 170, 235), Color::new(50, 135, 185));
    let border_color = Color::new(155, 155, 155);
    
    while !root.window_closed() {
        
        // ----- Process Input -----
        {
            let keypress = root.check_for_keypress(KEY_PRESSED);
            if keypress.is_some() {
                match keypress.unwrap().code {
                    KeyCode::Spacebar => execute = true,
                    KeyCode::Tab => {
                        tabbing = if tabbing.is_some() {
                            None
                        } else {
                            Some(0)
                        }
                    },
                    KeyCode::Up => {
                        if !(running || tabbing.is_some()) && cursor.1 > 1 {
                            cursor.1 -= 1;
                        }
                    },
                    KeyCode::Down => {
                        if !(running || tabbing.is_some()) && cursor.1 < 44 {
                            cursor.1 += 1;
                        }
                    },
                    KeyCode::Right => {
                        if !(running || tabbing.is_some()) && cursor.0 < 78 {
                            cursor.0 += 1;
                        } else if tabbing.is_some() && tabbing.unwrap() < 3 {
                            tabbing = Some(tabbing.unwrap() + 1);
                        }
                    },
                    KeyCode::Left => {
                        if !(running || tabbing.is_some()) && cursor.0 > 1 {
                            cursor.0 -= 1;
                        } else if tabbing.is_some() && tabbing.unwrap() > 0 {
                            tabbing = Some(tabbing.unwrap() - 1);
                        }
                    },
                    KeyCode::Escape => break,
                    _ => {} 
                }
            }
        }
        
        // ----- Update Game State -----
        {
            if running {
                // pause a bit if necessary so user can see the results
                if now() >= last_iter + Duration::milliseconds(300) {
                    // update life on the grid.
                    let mut new_state: [[bool; 45]; 79] = [[false; 45]; 79];
                    for x in 0..79 {
                        for y in 0..45 {
                            let neighbors = get_live_neighbors(x as isize, y as isize, state);
                            // this part could let bugs fall through if neighbors somehow exceeded the expected number...
                            if state[x][y] {
                                new_state[x][y] = match neighbors {
                                    2 => true,
                                    3 => true,
                                    _ => false
                                };
                            } else {
                                new_state[x][y] = if neighbors == 3 {
                                    true
                                } else {
                                    false
                                };
                            }
                        }
                    }
                    state = new_state;
                    last_iter = now();
                }
            } 
            if execute {
                if tabbing.is_some() {
                    match tabbing {
                        Some(0) => {
                            running = false;
                            tabbing = None;
                        },
                        Some(1) => running = true,
                        Some(2) => {
                            if running {
                                // If we're running just go to edit mode before clearing.
                                running = false;
                                tabbing = None;
                            }
                            clear_stage(&mut state);
                        },
                        Some(3) => break,
                        None => panic!("tabbing.is_some what are you even doing. Seriously."),
                        _ => panic!("Well now there's more tabs? Nobody told tabbing update... :<")
                    }
                } else {
                    if state[cursor.0][cursor.1] {
                        state[cursor.0][cursor.1] = false;
                    } else {
                        state[cursor.0][cursor.1] = true;
                    }
                }
                execute = false;
            }
        }
        
        // ----- Render Results -----
        {
            root.clear();
            
            // top and bottom borders
            for x in (0..80) {
                root.set_char_background(x, 0, border_color, BackgroundFlag::Set);
                root.set_char_background(x, 45, border_color, BackgroundFlag::Set);
            }
            // side borders
            for y in (1..45) {
                root.set_char_background(0, y, border_color, BackgroundFlag::Set);
                root.set_char_background(79, y, border_color, BackgroundFlag::Set);
            }
            
            // menu items
            root.print(12, 47, "Edit");
            root.print(31, 47, "Run");
            root.print(48, 47, "Clear");
            root.print(67, 47, "Quit");
            
            // draw the tabbing
            match tabbing {
                Some(0) => {
                    for x in (12..16) {
                        root.set_char_foreground(x, 47, tabbity_colors.0);
                        root.set_char_background(x, 47, tabbity_colors.1, BackgroundFlag::Set);
                    }
                },
                Some(1) => {
                    for x in (31..34) {
                        root.set_char_foreground(x, 47, tabbity_colors.0);
                        root.set_char_background(x, 47, tabbity_colors.1, BackgroundFlag::Set);
                    }
                },
                Some(2) => {
                    for x in (48..53) {
                        root.set_char_foreground(x, 47, tabbity_colors.0);
                        root.set_char_background(x, 47, tabbity_colors.1, BackgroundFlag::Set);
                    }
                },
                Some(3) => {
                    for x in (67..71) {
                        root.set_char_foreground(x, 47, tabbity_colors.0);
                        root.set_char_background(x, 47, tabbity_colors.1, BackgroundFlag::Set);
                    }
                },
                None => {},
                _ => panic!("tabbing off by one errors :(")
            }
            if !running {
                // we're editing here            
                // draw the stage (and then cursor, which is last, so as to cover any others)
                for x in 0..79 {
                    for y in 0..45 {
                        if state[x][y] {
                            root.set_char_background(x as i32, y as i32, live_cell_color, BackgroundFlag::Set)
                        }
                    }
                }
                if tabbing.is_none() {
                    root.set_char_background(cursor.0 as i32, cursor.1 as i32, cursor_color, BackgroundFlag::Set)
                }
            }
            else {
                // Paint life iteration.
                for x in 0..79 {
                    for y in 0..45 {
                        match state[x][y] {
                            true => root.set_char_background(x as i32, y as i32, live_cell_color, BackgroundFlag::Set),
                            false => root.set_char_background(x as i32, y as i32, bg_color, BackgroundFlag::Default)
                        }
                    }
                }
            }
            root.flush();
        }
    }
}

fn clear_stage(state: &mut [[bool; 45]; 79]) {
    for x in 0..79 {
        for y in 0..45 {
            state[x][y] = false;
        }
    }
}

fn get_live_neighbors(x: isize, y: isize, state: [[bool;45]; 79]) -> u8 {
    let mut live: u8 = 0;
    // start with top-left, going clockwise
    if in_bounds(x - 1, y - 1) {
        if state[(x - 1) as usize][(y - 1) as usize] {
            live += 1;
        }
    }
    if in_bounds(x - 1, y) {
        if state[(x - 1) as usize][(y) as usize] {
            live += 1;
        }
    }
    if in_bounds(x - 1, y + 1) {
        if state[(x - 1) as usize][(y + 1) as usize] {
            live += 1;
        }
    }
    if in_bounds(x, y - 1) {
        if state[(x) as usize][(y - 1) as usize] {
            live += 1;
        }
    }
    if in_bounds(x, y + 1) {
        if state[(x) as usize][(y + 1) as usize] {
            live += 1;
        }
    }
    if in_bounds(x + 1, y - 1) {
        if state[(x + 1) as usize][(y - 1) as usize] {
            live += 1;
        }
    }
    if in_bounds(x + 1, y) {
        if state[(x + 1) as usize][(y) as usize] {
            live += 1;
        }
    }
    if in_bounds(x + 1, y + 1) {
        if state[(x + 1) as usize][(y + 1) as usize] {
            live += 1;
        }
    }
    live
}

fn in_bounds(x: isize, y: isize) -> bool {
    if (x <= 78 && x >=0) && (y <= 44 && y >= 0) {      // Something major got mixed up here, x's and y's are swapped?
        true
    } else {
        false
    }
}