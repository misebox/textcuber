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

pub static centers: &'static [Attribute] = &[
    WHITE,
    YELLOW,
    GREEN,
    BLUE,
    MAGENTA,
    RED,
];

pub static corners: &'static [[Attribute; 3]; 8] = &[
    [ WHITE,    BLUE, MAGENTA],
    [ WHITE,     RED,    BLUE],
    [ WHITE,   GREEN,     RED],
    [ WHITE, MAGENTA,   GREEN],
    [YELLOW,    BLUE, MAGENTA],
    [YELLOW,    BLUE,     RED],
    [YELLOW,     RED,   GREEN],
    [YELLOW,   GREEN, MAGENTA],
];

pub static edges: &'static [[Attribute; 2]; 12] = &[
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

pub static base: &'static CubeState = &CubeState {
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
        WHITE => &"W",
        YELLOW => &"Y",
        GREEN => &"G",
        BLUE => &"B",
        MAGENTA => &"O",
        RED => &"R",
        _ => &" ",
    }
}

impl CubeState {
    pub fn get_face_cells(&self, face: &Face) -> [Attribute; 9] {
        let center = centers[face.clone() as usize];
        let cells: [Attribute; 9] = match face {
            Face::Up => [
                corners[self.cp[0]][0],  edges[self.ep[4]][0], corners[self.cp[1]][0],
                  edges[self.ep[7]][0],                center,   edges[self.ep[5]][0],
                corners[self.cp[3]][0],  edges[self.ep[6]][0], corners[self.cp[2]][0],
            ],
            Face::Down => [
                corners[self.cp[7]][0], edges[self.ep[10]][0], corners[self.cp[6]][0],
                 edges[self.ep[11]][0],                center,   edges[self.ep[9]][0],
                corners[self.cp[4]][0], edges[self.ep[10]][1], corners[self.cp[5]][0],
            ],
            Face::Front => [
                corners[self.cp[3]][2],  edges[self.ep[6]][1], corners[self.cp[2]][1],
                  edges[self.ep[3]][0],                center,   edges[self.ep[2]][0],
                corners[self.cp[7]][1], edges[self.ep[10]][1], corners[self.cp[6]][2],
            ],
            Face::Right => [
                corners[self.cp[2]][2],  edges[self.ep[5]][1], corners[self.cp[1]][1],
                  edges[self.ep[2]][1],                center,   edges[self.ep[1]][1],
                corners[self.cp[6]][1],  edges[self.ep[9]][1], corners[self.cp[5]][2],
            ],
            _ => [
                corners[self.cp[0]][0],
                edges[self.ep[4]][0],
                corners[self.cp[1]][0],
                edges[self.ep[7]][0],
                center,
                edges[self.ep[5]][0],
                corners[self.cp[3]][0],
                edges[self.ep[6]][0],
                corners[self.cp[2]][0],
            ],
        };
        //cells[(yi * 3 + xi) as usize]
        cells
    }
}
