mod face;
mod cube_move;
mod corners;
mod edges;

use move_::Move;
use cube::face::Face;
use cube::cube_move::CubeMove;
use cube::corners::Corners;
use cube::edges::Edges;

#[derive(Eq, PartialEq)]
pub struct Cube {
    corners: Corners,
    edges: Edges,
}

impl Cube {
    pub fn from_shuffle_sequence<I>(shuffle_sequence: I) -> Self
        where I: IntoIterator<Item=Move>
    {
        let mut new  = Self {
            corners: Corners::new(),
            edges: Edges::new(),
            };

        for m in shuffle_sequence.into_iter() {
            new.apply_move(m);
        }
        new
    }

    pub fn is_solved(&self) -> bool {
        *self == Self::default()
    }

    pub fn apply_move(&mut self, m: Move) {
        match m {
            Move::Front => self.apply_cube_move(CubeMove::Front),
            Move::FrontPrime => self.apply_cube_move(CubeMove::FrontPrime),
            Move::Front2 => {
                self.apply_cube_move(CubeMove::Front);
                self.apply_cube_move(CubeMove::Front);
            },
            Move::Right => self.apply_cube_move(CubeMove::Right),
            Move::RightPrime => self.apply_cube_move(CubeMove::RightPrime),
            Move::Right2 => {
                self.apply_cube_move(CubeMove::Right);
                self.apply_cube_move(CubeMove::Right);
            },
            Move::Up => self.apply_cube_move(CubeMove::Up),
            Move::UpPrime => self.apply_cube_move(CubeMove::UpPrime),
            Move::Up2 => {
                self.apply_cube_move(CubeMove::Up);
                self.apply_cube_move(CubeMove::Up);
            },
            Move::Back => self.apply_cube_move(CubeMove::Back),
            Move::BackPrime => self.apply_cube_move(CubeMove::BackPrime),
            Move::Back2 => {
                self.apply_cube_move(CubeMove::Back);
                self.apply_cube_move(CubeMove::Back);
            },
            Move::Left => self.apply_cube_move(CubeMove::Left),
            Move::LeftPrime => self.apply_cube_move(CubeMove::LeftPrime),
            Move::Left2 => {
                self.apply_cube_move(CubeMove::Left);
                self.apply_cube_move(CubeMove::Left);
            },
            Move::Down => self.apply_cube_move(CubeMove::Down),
            Move::DownPrime => self.apply_cube_move(CubeMove::DownPrime),
            Move::Down2 => {
                self.apply_cube_move(CubeMove::Down);
                self.apply_cube_move(CubeMove::Down);
            },
        }
    }

    fn apply_cube_move(&mut self, m: CubeMove) {
        self.corners.apply_move(m);
        self.edges.apply_move(m);
    }

    fn face(&self, face: Face) -> [Face; 9] {
        use self::corners::Corner::*;
        let corners = match face {
            Face::F => [UFL, URF, DFR, DLF],
            Face::B => [UBR, ULB, DBL, DRB],
            Face::U => [ULB, UBR, URF, UFL],
            Face::D => [DLF, DFR, DRB, DBL],
            Face::L => [ULB, UFL, DLF, DBL],
            Face::R => [URF, UBR, DRB, DFR],
        };

        let mut corner_faces: [self::Face; 4] = [self::Face::F; 4];

        for (i, c) in (&corners).iter().enumerate() {
            let corner_cubie: corners::Corner = self.corners.permutations[usize::from(*c)];

            corner_faces[i] = corner_cubie.face(*c, self.corners.orientations[usize::from(*c)], face);
        }

        use self::edges::Edge::*;
        let edges = match face {
            Face::F => [UF, RF, DF, LF],
            Face::B => [UB, LB, DB, RB],
            Face::U => [UB, UR, UF, UL],
            Face::D => [DF, DR, DB, DL],
            Face::L => [UL, LF, DL, LB],
            Face::R => [UR, RB, DR, RF],
        };

        let mut edge_faces: [self::Face; 4] = [self::Face::F; 4];

        for (i, e) in (&edges).iter().enumerate() {
            let edge_cubie: edges::Edge = self.edges.permutations[usize::from(*e)];

            edge_faces[i] = edge_cubie.face(*e, self.edges.orientations[usize::from(*e)], face);
        }

        [corner_faces[0], edge_faces[0], corner_faces[1],
        edge_faces[3], face, edge_faces[1],
        corner_faces[3], edge_faces[2], corner_faces[2]]
    }

    pub fn print(&self) {
        let faces = [
            self.face(self::Face::U),
            self.face(self::Face::L),
            self.face(self::Face::F),
            self.face(self::Face::R),
            self.face(self::Face::B),
            self.face(self::Face::D),
        ];

        print!("\n          ");
        for i in 0..9 {
            print!("{} {} \x1b[0m", faces[0][i].color(), faces[0][i].to_string());
            if i > 0 && (i+1) % 3 == 0 {
                print!("\n          ");
            }
        }
        print!("\r\n");
        for y in 0..3 {
            for &face in &faces {
                if face[4] != self::Face::U && face[4] != self::Face::D {
                    for x in 0..3 {
                        print!("{} {} \x1b[0m", face[x+y*3].color(), face[x+y*3].to_string());
                    }
                    print!(" ");
                }
            }
            print!("\n");
        }
        print!("\n          ");
        for i in 0..9 {
            print!("{} {} \x1b[0m", faces[5][i].color(), faces[5][i].to_string());

            if i > 0 && (i+1) % 3 == 0 {
                print!("\n          ");
            }
        }
        print!("\r");
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            corners: corners::Corners::default(),
            edges: edges::Edges::default(),
        }
    }
}
