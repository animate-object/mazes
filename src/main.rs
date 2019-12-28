pub mod maze;
use maze::*;

fn main() {
    let maze_gen = MazeGen::with_side_len(5).unwrap();
    println!("{:?}", maze_gen);
    println!("{:?}", maze_gen.look(0, &Direction::East));
    println!("{:?}", maze_gen.look(0, &Direction::West));
    println!("{}", maze_gen.random_cursor());
}
