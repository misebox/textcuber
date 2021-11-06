use std::collections::HashMap;
use std::ops;
use lazy_static::lazy_static;
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

pub enum Rotation {
    None=0,
    Right=1,
    UpsideDown=2,
    Left=3,
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

#[derive(Debug, Clone)]
pub struct CubeState {
    cp: [usize; 8],
    co: [usize; 8],
    ep: [usize; 12],
    eo: [usize; 12],
}

pub static BASE: CubeState = CubeState {
    cp: [0, 1, 2, 3, 4, 5, 6, 7],
    co: [0; 8],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
    eo: [0; 12],
};


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


impl ops::Add<CubeState> for CubeState {
    type Output = CubeState;

    fn add(self, mv: CubeState) -> CubeState {
        //println!("Added! ");
        //println!("{:?}", mv);
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
impl ops::Sub<CubeState> for CubeState {
    type Output = CubeState;

    fn sub(self, mv: CubeState) -> CubeState {
        self + mv.clone() + mv.clone() + mv
    }

}

impl CubeState {
    pub fn new(cp: [usize; 8], co: [usize; 8], ep: [usize; 12], eo: [usize; 12]) -> CubeState {
       CubeState { cp: cp.clone(), co: co.clone(), ep: ep.clone(), eo: eo.clone() } 
    }

    fn get_corner(&self, ci: usize, oi: usize) -> Attribute {
        let cp = self.cp[ci];
        let co = (self.co[ci] + oi) % 3usize;
        CORNERS[cp][co]
    }
    fn get_edge(&self, ei: usize, oi: usize) -> Attribute {
        let ep = self.ep[ei];
        let eo = (self.eo[ei] + oi) % 2;
        EDGES[ep][eo]
    }
    pub fn get_face_cells(&self, face: &Face) -> [Attribute; 9] {
        let center = CENTERS[face.clone() as usize];
        let cells: [Attribute; 9] = match face {
            Face::Up => [
                self.get_corner(0, 0), self.get_edge(4, 0), self.get_corner(1, 0),
                self.get_edge(7, 0),                       center,   self.get_edge(5, 0),
                self.get_corner(3, 0), self.get_edge(6, 0), self.get_corner(2, 0),
            ],
            Face::Down => [
                self.get_corner(7, 0), self.get_edge(10, 0), self.get_corner(6, 0),
                self.get_edge(11, 0),                       center,   self.get_edge(9, 0),
                self.get_corner(4, 0),  self.get_edge(8, 0), self.get_corner(5, 0),
            ],
            Face::Front => [
                self.get_corner(3, 2),  self.get_edge(6, 1), self.get_corner(2, 1),
                self.get_edge(3, 0),                        center,   self.get_edge(2, 0),
                self.get_corner(7, 1), self.get_edge(10, 1), self.get_corner(6, 2),
            ],
            Face::Back => [
                self.get_corner(1, 2), self.get_edge(4, 1), self.get_corner(0, 1),
                  self.get_edge(1, 0),                     center,   self.get_edge(0, 0),
                self.get_corner(5, 1), self.get_edge(8, 1), self.get_corner(4, 2),
            ],
            Face::Left => [
                self.get_corner(0, 2),  self.get_edge(7, 1), self.get_corner(3, 1),
                  self.get_edge(0, 1),                      center,   self.get_edge(3, 1),
                self.get_corner(4, 1), self.get_edge(11, 1), self.get_corner(7, 2),
            ],
            Face::Right => [
                self.get_corner(2, 2), self.get_edge(5, 1), self.get_corner(1, 1),
                  self.get_edge(2, 1),                     center,   self.get_edge(1, 1),
                self.get_corner(6, 1), self.get_edge(9, 1), self.get_corner(5, 2),
            ],
        };
        cells
    }
    pub fn get_color_from_face_pos(&self, face: &Face, x: i32, y: i32, r: &Rotation) -> Attribute {
        let mut cells = self.get_face_cells(face);
        match r {
            Rotation::None => {},
            Rotation::UpsideDown => {
                cells.reverse();
            },
            Rotation::Right => {
                cells = cells.map(|n| n as i32)
                    .map(|n| cells[((2 - n % 3) * 3 + n / 3) as usize]);
            },
            Rotation::Left => {
                cells = cells.map(|n| n as i32)
                    .map(|n| cells[(2 - n / 3 + (n % 3) * 3) as usize]);
            },
        }
        cells[(y * 3 + x) as usize]
    }

    pub fn apply_move(self, face: Face, reverse: bool) -> CubeState {
        // let moves = if reverse {REVERSED_MOVES} else {MOVES};
        let mv = MOVES[face as usize].clone();
        if reverse {
            self - mv
        } else {
            self + mv 
        }
    }
}
lazy_static! {
    static ref HASHMAP: HashMap<u32, &'static str> = {
        let mut m = HashMap::new();
        m.insert(0, "foo");
        m.insert(1, "bar");
        m.insert(2, "baz");
        m
    };
}
// MOVES
pub static MOVE_U: &'static CubeState = &CubeState {
    cp: [3, 0, 1, 2, 4, 5, 6, 7],
    co: [0; 8],
    ep: [0, 1, 2, 3, 7, 4, 5, 6, 8, 9, 10, 11],
    eo: [0; 12],
};
pub static MOVE_D: &'static CubeState = &CubeState {
    cp: [0, 1, 2, 3, 5, 6, 7, 4],
    co: [0; 8],
    ep: [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 8],
    eo: [0; 12],
};
pub static MOVE_F: &'static CubeState = &CubeState {
    cp: [0, 1, 3, 7, 4, 5, 2, 6],
    co: [0, 0, 1, 2, 0, 0, 2, 1],
    ep: [0, 1, 6, 10, 4, 5, 3, 7, 8, 9, 2, 11],
    eo: [0, 0, 1,  1, 0, 0, 1, 0, 0, 0, 1, 0],
};
pub static MOVE_B: &'static CubeState = &CubeState {
    cp: [1, 5, 2, 3, 0, 4, 6, 7],
    co: [1, 2, 0, 0, 2, 1, 0, 0],
    ep: [4, 8, 2, 3, 1, 5, 6, 7, 0, 9, 10, 11],
    eo: [1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0],
};
pub static MOVE_L: &'static CubeState = &CubeState {
    cp: [4, 1, 2, 0, 7, 5, 6, 3],
    co: [2, 0, 0, 1, 1, 0, 0, 2],
    ep: [11, 1, 2, 7, 4, 5, 6, 0, 8, 9, 10, 3],
    eo: [0; 12],
};
pub static MOVE_R: &'static CubeState = &CubeState {
    cp: [0, 2, 6, 3, 4, 1, 5, 7],
    co: [0, 1, 2, 0, 0, 2, 1, 0],
    ep: [0, 5, 9, 3, 4, 2, 6, 7, 8, 1, 10, 11],
    eo: [0; 12],
};
pub static MOVES: [&'static CubeState; 6] = [
    &*MOVE_U,
    &*MOVE_D,
    &*MOVE_F,
    &*MOVE_B,
    &*MOVE_L,
    &*MOVE_R,
];