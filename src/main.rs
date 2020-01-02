mod maze;
use maze::alg::*;
use maze::grid::*;

fn main() {
    let dim = Dimensions {
        width: 100,
        height: 1000,
    };
    let mut grid = Grid::with_dim(dim).unwrap();

    bin_tree::apply(&mut grid).err().map(|e| println!("{}", e));

    println!("{}", grid.to_ascii());
}
