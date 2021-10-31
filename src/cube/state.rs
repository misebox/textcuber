use std::collections::HashMap;
use std::ops;

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
    [YELLOW, MAGENTA,    BLUE],
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


pub static MOVES: [&'static CubeState; 6] = [
    // U
    &CubeState {
        cp: [0usize, 1, 2, 3, 4, 5, 6, 7],
        co: [0usize; 8],
        ep: [0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        eo: [0usize; 12],
    },
    // D
    &CubeState {
        cp: [0usize, 1, 2, 3, 4, 5, 6, 7],
        co: [0usize; 8],
        ep: [0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        eo: [0usize; 12],
    },
    // F
    &CubeState {
        cp: [0usize, 1, 2, 3, 4, 5, 6, 7],
        co: [0usize; 8],
        ep: [0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        eo: [0usize; 12],
    },
    // B
    &CubeState {
        cp: [0usize, 1, 2, 3, 4, 5, 6, 7],
        co: [0usize; 8],
        ep: [0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        eo: [0usize; 12],
    },
    // L
    &CubeState {
        cp: [0usize, 1, 2, 3, 4, 5, 6, 7],
        co: [0usize; 8],
        ep: [0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        eo: [0usize; 12],
    },
    // R
    &CubeState {
        cp: [0usize, 1, 2, 3, 4, 5, 6, 7],
        co: [0usize; 8],
        ep: [0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        eo: [0usize; 12],
    },
];

//                   C0-0  E4-0  C1-0
//                   E7-0   U    E5-0
//                   C3-0  E6-0  C2-0
//
// C0-2  E7-1 C3-1   C3-2  E4-1  C2-1   C2-2  E5-1  C1-1   C1-2  E4-1  C0-1
// E0-1   L   E3-1   E3-0   F    E2-0   E2-1   R    E1-1   E1-0   B    E0-0
// C4-1 E11-1 C7-2   C7-1 E10-1  C6-2   C6-1  E9-1  C5-2   C5-1  E8-1  C4-2
//
//                   C7-0 E10-0  C6-0
//                  E11-0   D    E9-0
//                   C4-0  E8-0  C5-0

#[derive(Debug, Clone)]
pub struct CubeState {
    cp: [usize; 8],
    co: [usize; 8],
    ep: [usize; 12],
    eo: [usize; 12],
}

pub static BASE: &'static CubeState = &CubeState {
    cp: [0usize, 1, 2, 3, 4, 5, 6, 7],
    co: [0usize; 8],
    ep: [0usize, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    eo: [0usize; 12],
};

impl ops::Add<CubeState> for CubeState {
    type Output = CubeState;

    fn add(self, mv: CubeState) -> CubeState {
        println!("Added! ");
        println!("{:?}", mv);
        let cp: [usize; 8] = mv.cp.iter().map(|&n| self.cp[n] as usize)
            .collect::<Vec<usize>>().try_into()
            .unwrap_or_else(|_| panic!("Expected length 8"));

        let co: [usize; 8] = mv.cp.into_iter().enumerate()
            .map(|(i, n)| ((self.co[n] + mv.co[i]) % 3usize) as usize)
            .collect::<Vec<usize>>().try_into()
            .unwrap_or_else(|_| panic!("Expected length 8"));

        let ep: [usize; 12] = mv.ep.iter().map(|&n| self.ep[n])
            .collect::<Vec<usize>>().try_into()
            .unwrap_or_else(|_| panic!("Expected length 12"));

        let eo: [usize; 12] = mv.ep.into_iter().enumerate()
            .map(|(i, n)| (self.eo[n] + mv.eo[i] % 2usize))
            .collect::<Vec<usize>>().try_into()
            .unwrap_or_else(|_| panic!("Expected length 12"));

        CubeState {
            cp,
            co,
            ep,
            eo,
        }
    }

}

impl CubeState {
    pub fn new(cp: [usize; 8], co: [usize; 8], ep: [usize; 12], eo: [usize; 12]) -> CubeState {
       CubeState { cp: cp.clone(), co: co.clone(), ep: ep.clone(), eo: eo.clone() } 
    }

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
                CORNERS[self.cp[4]][0], EDGES[self.ep[10]][0], CORNERS[self.cp[5]][0],
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
            Face::Left => [
                CORNERS[self.cp[0]][2],  EDGES[self.ep[7]][1], CORNERS[self.cp[3]][1],
                  EDGES[self.ep[0]][1],                center,   EDGES[self.ep[3]][1],
                CORNERS[self.cp[4]][1], EDGES[self.ep[11]][1], CORNERS[self.cp[7]][2],
            ],
            Face::Right => [
                CORNERS[self.cp[2]][2],  EDGES[self.ep[5]][1], CORNERS[self.cp[1]][1],
                  EDGES[self.ep[2]][1],                center,   EDGES[self.ep[1]][1],
                CORNERS[self.cp[6]][1],  EDGES[self.ep[9]][1], CORNERS[self.cp[5]][2],
            ],
        };
        cells
    }

    pub fn apply_move(self, face: Face, reverse: bool) -> CubeState {
        let mv = MOVES[face as usize].clone();
        self + mv
    }
}
