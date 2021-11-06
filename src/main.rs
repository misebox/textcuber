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
    let mut fn_draw_cube = views[view_idx];

    // Clear the screen to black
    tb.set_clear_attributes(BLACK, BLACK);
    tb.clear();

    tb.put_str(0, 1, "Press Esc to exit", WHITE, BLACK);
    fn_draw_cube(&mut tb, &state);
    tb.present();

    loop {
        if let Event::Key(event) = tb.poll_event() {
            // ESC
            if event.key == KEY_ESC {
                break;
            }
            #[allow(clippy::if_same_then_else)]
            if event.key == KEY_SPACE {
                fn_draw_cube(&mut tb, &state);
            } else if event.key == KEY_ENTER {
                fn_draw_cube(&mut tb, &state);
            } else if event.key == KEY_BACKSPACE || event.key == KEY_BACKSPACE2 {
                fn_draw_cube(&mut tb, &state);
            } else if event.key == KEY_TAB {
                view_idx = ((view_idx as i32 + 1) % 2) as usize;
                fn_draw_cube = views[view_idx];
                tb.clear();
                fn_draw_cube(&mut tb, &state);
            } else {
            }
            match event.ch {
                Some(ch) if "UDFBLRudfblrxyz'w".contains(&ch.to_string()) => {
                    tb.clear();
                    let c = format!("pressed {} key", &ch);
                    tb.put_str(0, 2, &c, WHITE, BLACK);
                    state = match ch {
                        'u' => state + cube::MOVE_U.clone(),
                        'r' => state + cube::MOVE_R.clone(),
                        'd' => state + cube::MOVE_D.clone(),
                        'l' => state + cube::MOVE_L.clone(),
                        'f' => state + cube::MOVE_F.clone(),
                        'b' => state + cube::MOVE_B.clone(),
                        'U' => state - cube::MOVE_U.clone(),
                        'R' => state - cube::MOVE_R.clone(),
                        'D' => state - cube::MOVE_D.clone(),
                        'L' => state - cube::MOVE_L.clone(),
                        'F' => state - cube::MOVE_F.clone(),
                        'B' => state - cube::MOVE_B.clone(),
                        _ => state,
                    };
                    fn_draw_cube(&mut tb, &state);
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
