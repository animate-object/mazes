pub mod maze;
use maze::*;

fn main() {
    let mut maze_gen = MazeGen::with_side_len(5).unwrap();
    println!("{:?}", maze_gen);
    println!("{:?}", maze_gen.carve(0, &Direction::South));
    println!("{:?}", maze_gen.carve(5, &Direction::South));
    println!("{:?}", maze_gen.carve(10, &Direction::South));
    println!("{:?}", maze_gen.carve(15, &Direction::South));
    println!("{:?}", maze_gen.carve(20, &Direction::South));
    println!("{:?}", maze_gen);
}
