mod cube;

extern crate termbox;

use termbox::{
    Event, Termbox, BLACK, KEY_BACKSPACE, KEY_BACKSPACE2, KEY_ENTER, KEY_ESC, KEY_SPACE, KEY_TAB,
    WHITE,
};

// use cube::views::net::draw_cube;
use cube::views;

fn notation_from_char(ch: char) -> &'static str {
    match ch {
        'u' => "U",
        'd' => "D",
        'f' => "F",
        'b' => "B",
        'l' => "L",
        'r' => "R",
        'U' => "U'",
        'D' => "D'",
        'F' => "F'",
        'B' => "B'",
        'L' => "L'",
        'R' => "R'",
        // TODO implement x,y,z,m,e,s
        _ => ".",
    }
}

fn print_seq(tb: &mut Termbox, seq: &Vec<&str>) {
    let joined = seq.join(" ");
    let s = format!("Sequence: {}", &joined);
    tb.put_str(2, 1, &s, WHITE, BLACK);
}
fn main() {
    let mut state = cube::BASE.clone();
    let mut tb = Termbox::open().unwrap();
    let views = [views::perspective::draw_cube, views::net::draw_cube];
    let mut view_idx: usize = 0usize;
    let mut fn_draw_cube = views[view_idx];
    let mut seq: Vec<&str> = Vec::new();

    // Clear the screen to black
    tb.set_clear_attributes(BLACK, BLACK);
    tb.clear();
    tb.put_str(0, 0, "Press Esc to exit", WHITE, BLACK);
    fn_draw_cube(&mut tb, &state);
    print_seq(&mut tb, &seq);
    tb.present();

    loop {
        if let Event::Key(event) = tb.poll_event() {
            // ESC
            if event.key == KEY_ESC {
                break;
            }
            #[allow(clippy::if_same_then_else)]
            if event.key == KEY_SPACE {
            } else if event.key == KEY_ENTER {
                seq.clear();
            } else if event.key == KEY_BACKSPACE || event.key == KEY_BACKSPACE2 {
                state = cube::BASE;
                seq.clear();
            } else if event.key == KEY_TAB {
                view_idx = ((view_idx as i32 + 1) % 2) as usize;
                fn_draw_cube = views[view_idx];
            } else {
            }
            match event.ch {
                Some(ch) if "UDFBLRMESXYZudfblrmesxyz".contains(&ch.to_string()) => {
                    tb.clear();
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
                        // TODO implement x,y,z,m,e,s
                        _ => state,
                    };
                    seq.push(notation_from_char(ch));
                }
                Some('u') => {}
                Some('q') => {
                    break;
                }
                // else
                Some(_) => {}
                None => {}
            }
            tb.clear();
            tb.put_str(0, 0, "Press Esc to exit", WHITE, BLACK);
            fn_draw_cube(&mut tb, &state);
            print_seq(&mut tb, &seq);
            tb.present();
        }
    }
}
