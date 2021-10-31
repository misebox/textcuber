use super::super::state::*;

use termbox::{
    Termbox, BLACK, BLUE, BOLD, GREEN, MAGENTA, RED, WHITE, YELLOW,
};


struct FaceView {
    face: Face,
    x: i32,
    y: i32,
    // times of rotation to right
    rotation: i32,
}

pub fn draw_cube(tb: &mut Termbox, state: &CubeState) {
    // cell width
    let cell_wx = 2;
    let cell_wy = 1;
    // face margin
    let face_mx = 2;
    let face_my = 1;

    // screen position
    let sx = 2;
    let sy = 5;
    let face_width = cell_wx * 3 + face_mx;
    let face_height = cell_wy * 3 + face_my;
    let fg = BLACK;

    let face_views = [
        FaceView {face: Face::Back , x: 2, y: 0, rotation: 2},
        FaceView {face: Face::Up   , x: 2, y: 1, rotation: 0},
        FaceView {face: Face::Back , x: 0, y: 2, rotation: 0},
        FaceView {face: Face::Left , x: 1, y: 2, rotation: 0},
        FaceView {face: Face::Front, x: 2, y: 2, rotation: 0},
        FaceView {face: Face::Right, x: 3, y: 2, rotation: 0},
        FaceView {face: Face::Back , x: 4, y: 2, rotation: 0},
        FaceView {face: Face::Down , x: 2, y: 3, rotation: 0},
        FaceView {face: Face::Back , x: 2, y: 4, rotation: 0},
    ];
    for fv in face_views {
        let cells = state.get_face_cells(&fv.face);
        for yi in 0..3 {
            for xi in 0..3 {
                let x = sx + face_width * fv.x + cell_wx * xi;
                let y = sx + face_height * fv.y + cell_wy * yi;
                let bg = cells[(yi * 3 + xi) as usize];
                let ch = get_color_char(bg);
                tb.put_str(x, y, ch, fg, bg);
                tb.put_str(x+1, y, " ", fg, bg);
            }
        }
    }

    tb.present();
}