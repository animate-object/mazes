pub mod maze;
use maze::*;

fn main() {
    let maze_gen = MazeGen::with_side_len(5u32).unwrap();
    println!("{:?}", maze_gen);
    println!("{}", maze_gen.random_cursor());
}
