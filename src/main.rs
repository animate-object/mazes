mod maze;
use maze::alg::*;
use maze::grid::*;

fn main() {
    let corners = vec![
        Corner::NorthEast,
        Corner::NorthWest,
        Corner::SouthEast,
        Corner::SouthWest,
    ];

    for c in corners.iter() {
        let dim = Dimensions {
            width: 10,
            height: 10,
        };
        let mut grid = Grid::with_dim(dim).unwrap();
        bin_tree::apply(&mut grid, c)
            .err()
            .map(|e| println!("{}", e));
        println!("{}", grid.to_ascii());
    }
}
