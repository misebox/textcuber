use std::collections::HashMap;

use termbox::{
    Attribute,
    BLACK,
    BLUE,
    RED,
    GREEN,
    MAGENTA,
    YELLOW,
    WHITE,
};


#[derive(Debug)]
pub enum CubePart {
    Center(Attribute),
    Edge([Attribute; 2]),
    Corner([Attribute; 3]),
}

pub static CENTERS: &'static [Attribute] = &[
    WHITE,
    YELLOW,
    GREEN,
    BLUE,
    MAGENTA,
    RED,
];

pub static CORNERS: &'static [[Attribute; 3]; 8] = &[
    [ WHITE,    BLUE, MAGENTA],
    [ WHITE,     RED,    BLUE],
    [ WHITE,   GREEN,     RED],
    [ WHITE, MAGENTA,   GREEN],
    [YELLOW,    BLUE, MAGENTA],
    [YELLOW,    BLUE,     RED],
    [YELLOW,     RED,   GREEN],
    [YELLOW,   GREEN, MAGENTA],
];

pub static EDGES: &'static [[Attribute; 2]; 12] = &[
    [  BLUE, MAGENTA],
    [  BLUE,     RED],
    [ GREEN,     RED],
    [ GREEN, MAGENTA],
    [ WHITE,    BLUE],
    [ WHITE,     RED],
    [ WHITE,   GREEN],
    [ WHITE, MAGENTA],
    [YELLOW,    BLUE],
    [YELLOW,     RED],
    [YELLOW,   GREEN],
    [YELLOW, MAGENTA],
];

#[derive(Debug, Clone)]
pub struct CubeState {
    cp: [usize; 8],
    co: [usize; 8],
    ep: [usize; 12],
    eo: [usize; 12],
}

pub static BASE: &'static CubeState = &CubeState {
    cp: [0, 1, 2, 3, 4, 5, 6, 7],
    co: [0; 8],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    eo: [0; 12],
};


#[derive(Clone)]
pub enum Face {
    Up=0,
    Down,
    Front,
    Back,
    Left,
    Right,
}

pub fn get_color_char(color: Attribute) -> &'static str {
    match (color) {
        WHITE   => &"W",
        YELLOW  => &"Y",
        GREEN   => &"G",
        BLUE    => &"B",
        MAGENTA => &"O",
        RED     => &"R",
        _       => &" ",
    }
}

impl CubeState {
    pub fn get_face_cells(&self, face: &Face) -> [Attribute; 9] {
        let center = CENTERS[face.clone() as usize];
        let cells: [Attribute; 9] = match face {
            Face::Up => [
                CORNERS[self.cp[0]][0],  EDGES[self.ep[4]][0], CORNERS[self.cp[1]][0],
                  EDGES[self.ep[7]][0],                center,   EDGES[self.ep[5]][0],
                CORNERS[self.cp[3]][0],  EDGES[self.ep[6]][0], CORNERS[self.cp[2]][0],
            ],
            Face::Down => [
                CORNERS[self.cp[7]][0], EDGES[self.ep[10]][0], CORNERS[self.cp[6]][0],
                 EDGES[self.ep[11]][0],                center,   EDGES[self.ep[9]][0],
                CORNERS[self.cp[4]][0], EDGES[self.ep[10]][1], CORNERS[self.cp[5]][0],
            ],
            Face::Front => [
                CORNERS[self.cp[3]][2],  EDGES[self.ep[6]][1], CORNERS[self.cp[2]][1],
                  EDGES[self.ep[3]][0],                center,   EDGES[self.ep[2]][0],
                CORNERS[self.cp[7]][1], EDGES[self.ep[10]][1], CORNERS[self.cp[6]][2],
            ],
            Face::Back => [
                CORNERS[self.cp[1]][2],  EDGES[self.ep[4]][1], CORNERS[self.cp[0]][1],
                  EDGES[self.ep[1]][0],                center,   EDGES[self.ep[0]][0],
                CORNERS[self.cp[5]][1],  EDGES[self.ep[8]][1], CORNERS[self.cp[4]][2],
            ],
            Face::Right => [
                CORNERS[self.cp[2]][2],  EDGES[self.ep[5]][1], CORNERS[self.cp[1]][1],
                  EDGES[self.ep[2]][1],                center,   EDGES[self.ep[1]][1],
                CORNERS[self.cp[6]][1],  EDGES[self.ep[9]][1], CORNERS[self.cp[5]][2],
            ],
            Face::Left => [
                CORNERS[self.cp[0]][1],  EDGES[self.ep[7]][1], CORNERS[self.cp[3]][1],
                  EDGES[self.ep[0]][1],                center,   EDGES[self.ep[3]][1],
                CORNERS[self.cp[4]][1], EDGES[self.ep[11]][1], CORNERS[self.cp[7]][2],
            ],
        };
        //cells[(yi * 3 + xi) as usize]
        cells
    }
}
