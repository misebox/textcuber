mod cube;

extern crate termbox;

use termbox::{
    Event, Termbox, BLACK, KEY_BACKSPACE, KEY_BACKSPACE2, KEY_ENTER, KEY_ESC, KEY_SPACE, KEY_TAB,
    WHITE,
};

// use cube::views::net::draw_cube;
use cube::views;

fn main() {
    let mut state = cube::BASE.clone();
    let mut tb = Termbox::open().unwrap();
    let views = [views::perspective::draw_cube, views::net::draw_cube];
    let mut view_idx: usize = 0usize;
    let mut draw_cube = views[view_idx];

    // Clear the screen to black
    tb.set_clear_attributes(BLACK, BLACK);
    tb.clear();

    tb.put_str(0, 1, "Press Esc to exit", WHITE, BLACK);
    draw_cube(&mut tb, &state);
    tb.present();

    loop {
        if let Event::Key(event) = tb.poll_event() {
            // ESC
            if event.key == KEY_ESC {
                break;
            }
            #[allow(clippy::if_same_then_else)]
            if event.key == KEY_SPACE {
                draw_cube(&mut tb, &state);
            } else if event.key == KEY_ENTER {
                draw_cube(&mut tb, &state);
            } else if event.key == KEY_BACKSPACE || event.key == KEY_BACKSPACE2 {
                draw_cube(&mut tb, &state);
            } else if event.key == KEY_TAB {
                view_idx = ((view_idx + 1) % 2);
                draw_cube = views[view_idx];
                tb.clear();
                draw_cube(&mut tb, &state);
            } else {
            }
            match event.ch {
                Some(ch) if "udlrfbxyz'w".contains(&ch.to_string()) => {
                    let c = format!("pressed {} key", &ch);
                    tb.put_str(0, 2, &c, WHITE, BLACK);
                    if ch == 'u' {
                        state = state + cube::MOVE_U.clone();
                    }
                    tb.clear();
                    draw_cube(&mut tb, &state);
                    tb.present();
                }
                Some('u') => {}
                Some('q') => {
                    break;
                }
                // else
                Some(_) => {}
                None => {}
            }
        }
    }
}
