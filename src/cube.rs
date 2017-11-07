use std::collections::HashMap;

use move_::Move;

#[derive(Clone, Copy, Eq, PartialEq)]
enum Face {
    F, // Front
    B, // Back
    U, // Up
    D, // Down
    L, // Left
    R, // Right
}

impl Face {
    fn color(&self) -> &str {
        match *self {
            Face::F => "\x1b[7;33m", // Yellow
            Face::B => "\x1b[7;31m", // Red
            Face::U => "\x1b[7;37m", // White
            Face::D => "\x1b[7;47;30m", // Black
            Face::L => "\x1b[7;32m", // Green
            Face::R => "\x1b[7;34m", // Blue
        }

    }
}

impl ToString for Face {
    fn to_string(&self) -> String {
        match *self {
            Face::F => "F",
            Face::B => "B",
            Face::U => "U",
            Face::D => "D",
            Face::L => "L",
            Face::R => "R",
        }.to_string()
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Corner {
    UFR,
    UFL,
    UBL,
    UBR,
    DFR,
    DFL,
    DBL,
    DBR,
}

impl Corner {
    fn decompose(&self) -> (Face, Face, Face) {
        use self::Corner::*;
        match *self {
            UFR => (Face::U, Face::F, Face::R),
            UFL => (Face::U, Face::F, Face::L),
            UBL => (Face::U, Face::B, Face::L),
            UBR => (Face::U, Face::B, Face::R),
            DFR => (Face::D, Face::F, Face::R),
            DFL => (Face::D, Face::F, Face::L),
            DBL => (Face::D, Face::B, Face::L),
            DBR => (Face::D, Face::B, Face::R),
        }
    }

    fn orient(&self, orientation: u8) -> (Face, Face, Face) {
        let (a, b, c) = (*self).decompose();

        match orientation {
            0 => (a, b, c),
            1 => (b, c, a),
            _ => (c, a, b),
        }
    }

    fn get_face(&self, cubicle: Corner, orientation: u8, face: Face) -> Face {
        let (oriented_a, oriented_b, oriented_c) = (*self).orient(orientation);
        let (a, b, _c) = cubicle.decompose();

        match face {
            x if x == a => oriented_a,
            x if x == b => oriented_b,
            _ => oriented_c,
        }
    }
}

impl From<Corner> for usize {
    fn from(c: Corner) -> Self {
        use self::Corner::*;

        match c {
            UFR => 0,
            UFL => 1,
            UBL => 2,
            UBR => 3,
            DFR => 4,
            DFL => 5,
            DBL => 6,
            DBR => 7,
        }
    }
}

struct Corners {
    permutations: [Corner; 8],
    orientations: [u8; 8],
}

impl Corners {
    fn new() -> Self {
        Corners::default()
    }
}

impl Default for Corners {
    fn default() -> Self {
        let mut corners: [Corner; 8] = [UFR; 8];
        use self::Corner::*;

        corners[usize::from(UFR)] = UFR;
        corners[usize::from(UFL)] = UFL;
        corners[usize::from(UBL)] = UBL;
        corners[usize::from(UBR)] = UBR;
        corners[usize::from(DFR)] = DFR;
        corners[usize::from(DFL)] = DFL;
        corners[usize::from(DBL)] = DBL;
        corners[usize::from(DBR)] = DBR;

        Self {
            permutations: corners,
            orientations: [0; 8],
        }
    }
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
enum Edge {
    UR,
    UF,
    UL,
    UB,
    DR,
    DF,
    DL,
    DB,
    FR,
    FL,
    BR,
    BL,
}

impl Edge {
    fn decompose(&self) -> (Face, Face) {
        use self::Edge::*;
        match *self {
            UR => (Face::U, Face::R),
            UF => (Face::U, Face::F),
            UL => (Face::U, Face::L),
            UB => (Face::U, Face::B),
            DR => (Face::D, Face::R),
            DF => (Face::D, Face::F),
            DL => (Face::D, Face::L),
            DB => (Face::D, Face::B),
            FR => (Face::F, Face::R),
            FL => (Face::F, Face::L),
            BR => (Face::B, Face::R),
            BL => (Face::B, Face::L),
        }
    }

    fn orient(&self, orientation: u8) -> (Face, Face) {
        let (a, b) = (*self).decompose();

        match orientation {
            0 => (a, b),
            _ => (b, a),
        }
    }

    fn get_face(&self, cubicle: Edge, orientation: u8, face: Face) -> Face {
        let (oriented_a, oriented_b) = (*self).orient(orientation);
        let (a, _b) = cubicle.decompose();

        match face {
            x if x == a => oriented_a,
            _ => oriented_b,
        }
    }
}

impl From<Edge> for usize {
    fn from(e: Edge) -> Self {
        use self::Edge::*;

        match e {
            UR => 0,
            UF => 1,
            UL => 2,
            UB => 3,
            DR => 4,
            DF => 5,
            DL => 6,
            DB => 7,
            FR => 8,
            FL => 9,
            BR => 10,
            BL => 11,
        }
    }
}

struct Edges {
    map: HashMap<Edge, Edge>,
    orientations: [u8; 12],
}

impl Edges {
    fn new() -> Self {
        Edges::default()
    }
}

impl Default for Edges {
    fn default() -> Self {
        let mut edges = HashMap::new();
        use self::Edge::*;

        edges.insert(UR, UR);
        edges.insert(UF, UF);
        edges.insert(UL, UL);
        edges.insert(UB, UB);
        edges.insert(DR, DR);
        edges.insert(DF, DF);
        edges.insert(DL, DL);
        edges.insert(DB, DB);
        edges.insert(FR, FR);
        edges.insert(FL, FL);
        edges.insert(BR, BR);
        edges.insert(BL, BL);

        Self {
            map: edges,
            orientations: [0; 12],
        }
    }
}

pub struct Cube {
    shuffle_sequence: Vec<Move>,
    corners: Corners,
    edges: Edges,
}

impl Cube {
    pub fn new() -> Self {
        Self {
            shuffle_sequence: Vec::new(),
            corners: Corners::new(),
            edges: Edges::new(),
        }
    }

    pub fn from_shuffle_sequence(shuffle_sequence: &Vec<Move>) -> Self {
        Self {
            shuffle_sequence: (*shuffle_sequence).clone(),
            corners: Corners::new(),
            edges: Edges::new(),
        }
    }

    fn get_face(&self, face: Face) -> [Face; 9] {
        use self::Corner::*;
        let corners = match face {
            Face::F => [UFL, UFR, DFR, DFL],
            Face::B => [UBL, UBR, DBL, DBR],
            Face::U => [UBL, UBR, UFR, UFL],
            Face::D => [DFL, DFR, DBR, DBL],
            Face::L => [UBL, UFL, DFL, DBL],
            Face::R => [UFR, UBR, DBR, DFR],
        };

        let mut corner_faces: [self::Face; 4] = [self::Face::F; 4];

        for (i, c) in (&corners).iter().enumerate() {
            let corner_cubie: Corner = self.corners.permutations[usize::from(*c)];
            let corner_index: usize = corner_cubie.into();

            corner_faces[i] = corner_cubie.get_face(*c, self.corners.orientations[corner_index], face);
        }

        use self::Edge::*;
        let edges = match face {
            Face::F => [UF, FR, DF, FL],
            Face::B => [UB, BL, DB, BR],
            Face::U => [UB, UR, UF, UL],
            Face::D => [DF, DR, DB, DL],
            Face::L => [UL, FL, DL, BL],
            Face::R => [UR, BR, DR, FR],
        };

        let mut edge_faces: [self::Face; 4] = [self::Face::F; 4];

        for (i, e) in (&edges).iter().enumerate() {
            let edge_cubie: Edge = *self.edges.map.get(&e).unwrap();
            let edge_index: usize = edge_cubie.into();

            edge_faces[i] = edge_cubie.get_face(*e, self.edges.orientations[edge_index], face);
        }

        [corner_faces[0], edge_faces[0], corner_faces[1],
        edge_faces[3], face, edge_faces[1],
        corner_faces[3], edge_faces[2], corner_faces[2]]
    }

    pub fn print(&self) {
        let faces = [
            self.get_face(self::Face::U),
            self.get_face(self::Face::L),
            self.get_face(self::Face::F),
            self.get_face(self::Face::R),
            self.get_face(self::Face::B),
            self.get_face(self::Face::D),
        ];
        print!("\n         ");
        for i in 0..9 {
            print!("{} {} \x1b[0m", faces[0][i].color(), faces[0][i].to_string());
            if i > 0 && (i+1) % 3 == 0 {
                print!("\n         ");
            }
        }
        print!("\r");
        for y in 0..3 {
            for &face in &faces {
                for x in 0..3 {
                    if face[x+y*3] != self::Face::U && face[x+y*3] != self::Face::D {
                        print!("{} {} \x1b[0m", face[x+y*3].color(), face[x+y*3].to_string());
                    }
                }
            }
            print!("\n");
        }

        print!("         ");
        for i in 0..9 {
            print!("{} {} \x1b[0m", faces[5][i].color(), faces[5][i].to_string());

            if i > 0 && (i+1) % 3 == 0 {
                print!("\n         ");
            }
        }
        print!("\r");
    }
}
