mod cube;

extern crate termbox;

use termbox::{
    Event, Termbox, BLACK, BLUE, BOLD, GREEN, KEY_ESC, KEY_SPACE, MAGENTA, RED, WHITE, YELLOW,
};

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
            if event.key == KEY_SPACE {
                draw_cube(&mut tb, &state);
            }
            match event.ch {
                Some('j') => {
                    tb.put_str(0, 2, "pressed j key", WHITE, BLACK);
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
