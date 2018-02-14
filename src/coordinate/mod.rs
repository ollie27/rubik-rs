use cube::Cube;
use move_::Move;

use std::fs;
use std::fs::File;
use std::io::Write;

use bincode;
use serde;

const NB_MOVES: usize = 18;
const NB_TWIST: usize = 2187;
const NB_FLIP: usize = 2048;
const NB_FR_TO_BR: usize = 11880;
const NB_URF_TO_DLF: usize = 20160;
const NB_UR_TO_UL: usize = 1320;
const NB_UB_TO_DF: usize = 1320;
const NB_UR_TO_DF: usize = 20160;
const NB_SLICE: usize = 24;
const NB_PARITY: usize = 2;

pub struct Coordinate {
    cache_folder_name: String,
    twist: u32,
    flip: u32,
    parity: u32,
    fr_to_br: u32,
    urf_to_dlf: u32,
    ur_to_ul: u32,
    ub_to_df: u32,
    ur_to_df: u32,
    twist_move: Box<[[u32; NB_MOVES]; NB_TWIST]>,
    flip_move: Box<[[u32; NB_MOVES]; NB_FLIP]>,
    parity_move: Box<[[i8; NB_MOVES]; 2]>,
    fr_to_br_move: Box<[[u32; NB_MOVES]; NB_FR_TO_BR]>,
    urf_to_dlf_move: Box<[[u32; NB_MOVES]; NB_URF_TO_DLF]>,
    ur_to_ul_move: Box<[[u32; NB_MOVES]; NB_UR_TO_UL]>,
    ub_to_df_move: Box<[[u32; NB_MOVES]; NB_UB_TO_DF]>,
    ur_to_df_move: Box<[[u32; NB_MOVES]; NB_UR_TO_DF]>,
    merge_ur_to_ul_and_ub_to_df: Box<[[i16; 336]; 336]>,
    urf_to_dlf_parity_prun: Box<[i8; NB_SLICE * NB_URF_TO_DLF * NB_PARITY / 2]>,
}

impl Coordinate {
    pub fn from_cube(cube: &Cube) -> Self {
        Self {
            cache_folder_name: String::from("pruning_tables"),
            twist: cube.twist(),
            flip: cube.flip(),
            parity: cube.corner_parity(),
            fr_to_br: cube.fr_to_br(),
            urf_to_dlf: cube.urf_to_dlf(),
            ur_to_ul: cube.ur_to_ul(),
            ub_to_df: cube.ub_to_df(),
            ur_to_df: cube.ur_to_df(),
            twist_move: Box::new([[0; NB_MOVES]; NB_TWIST]),
            flip_move: Box::new([[0; NB_MOVES]; NB_FLIP]),
            parity_move: Box::new([
                [1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 0, 1],
                [0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0]]),
            fr_to_br_move: Box::new([[0; NB_MOVES]; NB_FR_TO_BR]),
            urf_to_dlf_move: Box::new([[0; NB_MOVES]; NB_URF_TO_DLF]),
            ur_to_ul_move: Box::new([[0; NB_MOVES]; NB_UR_TO_UL]),
            ub_to_df_move: Box::new([[0; NB_MOVES]; NB_UB_TO_DF]),
            ur_to_df_move: Box::new([[0; NB_MOVES]; NB_UR_TO_DF]),
            merge_ur_to_ul_and_ub_to_df: Box::new([[0; 336]; 336]),
            urf_to_dlf_parity_prun: box [0; NB_SLICE * NB_URF_TO_DLF * NB_PARITY / 2],
        }
    }

    fn create_cache_dir(&self) {
        match fs::create_dir(&self.cache_folder_name) {
            Ok(_) => println!("cache \"folder pruning_tables\" created"),
            Err(e) => println!("{:?}", e),
        }
    }

    fn dump_to_file<T>(&self, arr: T, name: &str)
        where T: serde::ser::Serialize {
        let mut path = self.cache_folder_name.to_owned();
        path.push_str("/");
        path.push_str(name);

        let file = File::create(&path);
        match file {
            Ok(mut file) => {
                let encoded: Vec<u8> = bincode::serialize(&arr, bincode::Infinite).unwrap();
                match file.write(&encoded[..]) {
                    Ok(_) => { },
                    Err (e) => println!("{:?}", e),
                }
                match file.metadata() {
                    Ok(metadata) => {
                        let mut perm = metadata.permissions();
                        perm.set_readonly(true);
                        fs::set_permissions(&path, perm).unwrap();
                    },
                    Err(e) => println!("{:?}", e),
                }
            },
            Err(e) => println!("{:?}", e),
        };
    }

    pub fn init_pruning(&mut self) {
        self.create_cache_dir();

        self.init_twist_move();
        // self.dump_to_file(&self.twist_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "twist_move");

        self.init_flip_move();
        // self.dump_to_file(&self.flip_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "flip_move");

        self.init_fr_to_br_move();
        // self.dump_to_file(&self.fr_to_br_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "fr_to_br_move");

        self.init_urf_to_dlf_move();
        // self.dump_to_file(&self.urf_to_dlf_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "urf_to_dlf_move");

        self.init_ur_to_ul_move();
        // self.dump_to_file(&self.ur_to_ul_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "ur_to_ul_move");

        self.init_ub_to_df_move();
        // self.dump_to_file(&self.ub_to_df_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "ub_to_df_move");

        self.init_ur_to_df_move();
        // self.dump_to_file(&self.ur_to_df_move.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "ur_to_df_move");

        self.init_merge_ur_to_ul_and_ub_to_df();
        // self.dump_to_file(&self.merge_ur_to_ul_and_ub_to_df.iter().map(|x| &x[..]).collect::<Vec<&[u32]>>(), "merge_ur_to_ul_and_ub_to_df");

        self.init_urf_to_dlf_parity_prun();
        // self.dump_to_file(&self.urf_to_dlf_parity_prun.iter().map(|x| &x[..]).collect::<Vec<&[i8]>>(), "urf_to_dlf_parity_prun");
    }

    fn init_twist_move(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_TWIST {
            solved.set_twist(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.corners_multiply(Move::from_u(y));
                    self.twist_move[x][3 * y + z] = solved.twist();
                }
                solved.corners_multiply(Move::from_u(y));
            }
        }
    }

    fn init_flip_move(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_FLIP {
            solved.set_flip(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.flip_move[x][3 * y + z] = solved.flip();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
    }

    fn init_fr_to_br_move(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_FR_TO_BR {
            solved.set_fr_to_br(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.fr_to_br_move[x][3 * y + z] = solved.fr_to_br();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
    }

    fn init_urf_to_dlf_move(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_URF_TO_DLF {
            solved.set_urf_to_dlf(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.corners_multiply(Move::from_u(y));
                    self.urf_to_dlf_move[x][3 * y + z] = solved.urf_to_dlf();
                }
                solved.corners_multiply(Move::from_u(y));
            }
        }
    }

    fn init_ur_to_ul_move(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_UR_TO_UL {
            solved.set_ur_to_ul(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.ur_to_ul_move[x][3 * y + z] = solved.ur_to_ul();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
    }

    fn init_ub_to_df_move(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_UR_TO_UL {
            solved.set_ub_to_df(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.ub_to_df_move[x][3 * y + z] = solved.ub_to_df();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
    }

    fn init_ur_to_df_move(&mut self) {
        let mut solved = Cube::new_default();

        for x in 0..NB_UR_TO_DF {
            solved.set_ur_to_df(x as i16);
            for y in 0..6 {
                for z in 0..3 {
                    solved.edges_multiply(Move::from_u(y));
                    self.ur_to_df_move[x][3 * y + z] = solved.ur_to_df();
                }
                solved.edges_multiply(Move::from_u(y));
            }
        }
    }

    fn init_merge_ur_to_ul_and_ub_to_df(&mut self) {
        for ur_to_ul in 0..336 {
            for ub_to_df in 0..336 {
                self.merge_ur_to_ul_and_ub_to_df[ur_to_ul][ub_to_df] = Cube::ur_to_uf_standalone(ur_to_ul as i16, ub_to_df as i16);
            }
        }
    }

    fn init_urf_to_dlf_parity_prun(&mut self) {
        self.urf_to_dlf_parity_prun = Box::new([-1; NB_SLICE * NB_URF_TO_DLF * NB_PARITY / 2]);
        let mut depth = 0;
        let mut done = 1;

        Self::set_prunning(&mut self.urf_to_dlf_parity_prun[..], 0, 0);

        loop {
            if done == NB_SLICE * NB_URF_TO_DLF * NB_PARITY { break; }

            for x in 0..NB_SLICE * NB_URF_TO_DLF * NB_PARITY {
                let parity = x % 2;
                let urf_to_dlf = (x / 2) / NB_SLICE;
                let slice = (x / 2) % NB_SLICE;
                if Self::prunning(&self.urf_to_dlf_parity_prun[..], x) == depth {
                    for y in 0..NB_MOVES {
                        match y {
                            3 | 5 | 6 | 8 | 12 | 14 | 15 | 17 => continue,
                            _ => {
                                let n_slice = self.fr_to_br_move[slice][y];
                                let n_urf_to_dlf = self.urf_to_dlf_move[urf_to_dlf][y];
                                let n_parity = self.parity_move[parity][y];
                                let index = ((NB_SLICE as i32 * n_urf_to_dlf as i32 + n_slice as i32) * 2 + n_parity as i32) as usize;
                                if Self::prunning(&self.urf_to_dlf_parity_prun[..], index) == 0x0f {
                                    Self::set_prunning(&mut self.urf_to_dlf_parity_prun[..], index, depth + 1);
                                    println!("{:x}", Self::prunning(&self.urf_to_dlf_parity_prun[..], index));
                                    done += 1;
                                }
                            },
                        }
                    }
                }
            }
            depth += 1;
        }
    }



    fn set_prunning(arr: &mut [i8], i: usize, value: i8) {
        if i & 1 == 0 {
            arr[i / 2] &= 0xf0 | value;
        } else {
            arr[i / 2] &= 0x0f | (value << 4);
        }
    }

    fn prunning(arr: &[i8], i: usize) -> i8{
        let ret: i8;

        if i & 1 == 0 {
            ret = arr[i / 2] & 0x0f;
        } else {
            ret = (arr[i / 2] >> 4) & 0x0f;
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_init_twist_move(b: &mut Bencher) {
        let cube = Cube::new_default();
        let mut c = Coordinate::from_cube(&cube);

        b.iter(|| c.init_twist_move());
    }

    #[bench]
    fn bench_init_flip_move(b: &mut Bencher) {
        let cube = Cube::new_default();
        let mut c = Coordinate::from_cube(&cube);

        b.iter(|| c.init_flip_move());
    }

    #[bench]
    fn bench_init_fr_to_br_move(b: &mut Bencher) {
        let cube = Cube::new_default();
        let mut c = Coordinate::from_cube(&cube);

        b.iter(|| c.init_fr_to_br_move());
    }

    #[bench]
    fn bench_init_urf_to_dlf_move(b: &mut Bencher) {
        let cube = Cube::new_default();
        let mut c = Coordinate::from_cube(&cube);

        b.iter(|| c.init_urf_to_dlf_move());
    }

    #[bench]
    fn bench_init_ur_to_ul_move(b: &mut Bencher) {
        let cube = Cube::new_default();
        let mut c = Coordinate::from_cube(&cube);

        b.iter(|| c.init_ur_to_ul_move());
    }

    #[bench]
    fn bench_init_ur_to_ul_move(b: &mut Bencher) {
        let cube = Cube::new_default();
        let mut c = Coordinate::from_cube(&cube);

        b.iter(|| c.init_ub_to_df_move());
    }
}
