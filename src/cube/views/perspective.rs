use super::super::state;

use termbox::{
    Termbox, BLACK, WHITE,
};


pub fn draw_cube(tb: &mut Termbox, state: &state::CubeState) {
    //.______.______.______.
    let edge_len = 6;
    let bar = "_".repeat(edge_len);
    let cube_f_border = vec!["."; 4].join(&bar);
    let cell_spaces = " ".repeat(edge_len);
    let cube_f_cells = vec!["|"; 4].join(&cell_spaces);
    let cell_spaces = " ".repeat(edge_len);
    let cube_u_cells = vec!["/"; 4].join(&cell_spaces);

    let sx = 2;
    let sy = 5;
    let fx = sx;
    let fy = sy + 2 * 3;
    let rx = fx + (edge_len as i32 + 1) * 3;
    let ry = fy;
    let mut fg = WHITE;
    // U - F border
    tb.put_str(fx, fy, &cube_f_border, fg, BLACK);

    // F face
    for i in 0..3 {
        let y = fy + i * 3;
        tb.put_str(fx, y + 1, &cube_f_cells, fg, BLACK);
        tb.put_str(fx, y + 2, &cube_f_cells, fg, BLACK);
        tb.put_str(fx, y + 3, &cube_f_border, fg, BLACK);
    }

    let tilt = 4;
    let half_tilt = tilt / 2;
    // U face
    for i in 0..3 {
        let x = sx + half_tilt + i * tilt;
        let y = sy + 2 * (3 - i);
        tb.put_str(x + half_tilt, y - 2, &cube_f_border, fg, BLACK);
        tb.put_str(x, y - 1, &cube_u_cells, fg, BLACK);
    }

    // R face
    for yi in 0..3 {
        for xi in 0..3 {
            let x = fx + (edge_len as i32 + 1) * 3 + xi * tilt;
            let y = sy + 2 * (3 - xi) + 3 * yi;
            tb.put_str(x + tilt, y - 1, "|", fg, BLACK);
            tb.put_str(x + tilt, y, "|", fg, BLACK);
            tb.put_str(x + tilt, y + 1, ".", fg, BLACK);
            tb.put_str(x + half_tilt, y + 2, "/", fg, BLACK);
        }
    }

    fg = BLACK;
    // U face cells
    let u_cells = state.get_face_cells(&state::Face::Up);
    for yi in 0..3 {
        for xi in 0..3 {
            let x = sx + half_tilt + (edge_len as i32 + 1) * xi + tilt * (2 - yi);
            let y = sy + 1 + 2 * yi;
            let bg = u_cells[(yi * 3 + xi) as usize];
            let ch = state::get_color_char(bg);
            tb.put_str(x + half_tilt, y, &ch.repeat(4), fg, bg);
        }
    }

    // F face cells
    let f_cells = state.get_face_cells(&state::Face::Front);
    for yi in 0..3 {
        for xi in 0..3 {
            let x = fx + 2 + (edge_len as i32 + 1) * xi;
            let y = fy + yi * 3;
            let bg = f_cells[(yi * 3 + xi) as usize];
            let ch = state::get_color_char(bg);
            tb.put_str(x, y + 1, &ch.repeat(4), fg, bg);
            tb.put_str(x, y + 2, &ch.repeat(4), fg, bg);
        }
    }

    // R face cells
    let r_cells = state.get_face_cells(&state::Face::Right);
    for yi in 0..3 {
        for xi in 0..3 {
            let x = rx + tilt * xi;
            let y = ry - 2 * xi + 3 * yi;
            let bg = r_cells[(yi * 3 + xi) as usize];
            let ch = state::get_color_char(bg);
            tb.put_str(x+2, y, &ch.repeat(2), fg, bg);
            tb.put_str(x+1, y + 1, &ch.repeat(2), fg, bg);
        }
    }

    tb.present();
}