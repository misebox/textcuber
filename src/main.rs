mod cube;

extern crate termbox;



use termbox::{BLACK, Event, KEY_ENTER, KEY_ESC, KEY_SPACE, KEY_BACKSPACE, KEY_BACKSPACE2, Termbox, WHITE};

use cube::views::net::draw_cube;
// use cube::views::perspective::draw_cube;


fn main() {
    let mut state = cube::BASE.clone();

    let mut tb = Termbox::open().unwrap();

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
            } else {

            }
            match event.ch {
                Some(ch) if "udlrfbxyz'w".contains(&ch.to_string()) => {
                    let c = format!("pressed {} key", &ch);
                    tb.put_str(0, 2, &c, WHITE, BLACK);
                    tb.present();
                }
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
