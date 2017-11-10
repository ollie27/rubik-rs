mod move_;
mod cube;
mod solver;

use std::env;
use move_::Move;
use cube::Cube;
use solver::Solver;

fn main() {
    let first_arg = env::args().nth(1);
    match first_arg {
        None => {
            println!("Usage")
        }
        Some(arg) => {
            if let Ok(shuffle_sequence) = Move::sequence_from_str(&arg) {
                let cube = Cube::from_shuffle_sequence(shuffle_sequence);
                let solver = Solver::new(cube);

                solver.solve();
            }
        }
    }
}
