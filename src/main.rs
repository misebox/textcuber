extern crate termbox;

use termbox::{BLACK, BOLD, Event, KEY_CTRL_A, KEY_ESC, KEY_SPACE, Termbox, WHITE};

fn show(tb: &mut Termbox) {
  //.______.______.______.
  let edge_len= 6;
  let bar = "_".repeat(edge_len);
  let cube_f_border = vec!["."; 4].join(&bar);
  let cell_spaces = " ".repeat(edge_len);
  let cube_f_cells= vec!["|"; 4].join(&cell_spaces);
  let cell_spaces = " ".repeat(edge_len);
  let cube_u_cells= vec!["/"; 4].join(&cell_spaces);

  let sx = 2;
  let sy = 5;
  let fx = sx;
  let fy = sy + 2 * 3;
  // U - F border
  tb.put_str(fx, fy, &cube_f_border, WHITE, BLACK);

  // F face
  for i in 0..3 {
    let y = fy + i * 3; 
    tb.put_str(fx, y + 1, &cube_f_cells, WHITE, BLACK);
    tb.put_str(fx, y + 2, &cube_f_cells, WHITE, BLACK);
    tb.put_str(fx, y + 3, &cube_f_border, WHITE, BLACK);
  }

  let tilt = 4;
  let half_tilt = (tilt / 2);
  // U face
  for i in 0..3 {
    let x = sx + half_tilt + i * tilt;
    let y = sy + 2 * (3 - i); 
    tb.put_str(x + half_tilt, y - 2, &cube_f_border, WHITE, BLACK);
    tb.put_str(x, y - 1, &cube_u_cells, WHITE, BLACK);
  }

  // R face
  for yi in 0..3 {
    for xi in 0..3 {
      let x = fx + (edge_len as i32 + 1) * 3 + xi * tilt;
      let y = sy + 2 * (3 - xi) + 3 * yi; 
      tb.put_str(x + tilt, y - 1, "|", WHITE, BLACK);
      tb.put_str(x + tilt, y, "|", WHITE, BLACK);
      tb.put_str(x + tilt, y + 1, ".", WHITE, BLACK);
      tb.put_str(x + half_tilt, y + 2, "/", WHITE, BLACK);
    }
  }

  tb.present();

}
fn main () {
  let mut tb = Termbox::open().unwrap();

  // Clear the screen to black
  tb.set_clear_attributes(BLACK, BLACK);
  tb.clear();

  tb.put_str(0, 0, "Hello, world!", WHITE | BOLD, BLACK);
  tb.put_str(0, 1, "Press Esc to continue", WHITE, BLACK);
  show(&mut tb);
  tb.present();

  loop {
    if let Event::Key(event) = tb.poll_event() {
      // ESC
      if event.key == KEY_ESC {
        break;
      }
      if event.key == KEY_SPACE {
        show(&mut tb);
      }
      match event.ch {
        Some('j') => {
          tb.put_str(0, 2, "pressed A key", WHITE, BLACK);
          tb.present();
        },
        Some('q') => {
          break;
        },
        // else
        Some(_) => {},
        None => {},
      }
    }
  }
}