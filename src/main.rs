mod maze;
use maze::alg::*;
use maze::grid::*;

fn main() {
    println!("Binary Tree Algorithm");
    static CORNERS: [Corner; 4] = [
        Corner::NorthEast,
        Corner::NorthWest,
        Corner::SouthEast,
        Corner::SouthWest,
    ];
    for c in CORNERS.iter() {
        let dim = Dimensions {
            width: 10,
            height: 10,
        };
        let mut grid = Grid::with_dim(dim).unwrap();
        bin_tree::apply(&mut grid, c)
            .err()
            .map(|e| println!("{}", e));
        println!("Starting corner {:?}", c);
        println!("{}", grid.to_ascii());
    }

    println!("Sidewinder algorithm.");
    static DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    for d in DIRECTIONS.iter() {
        let dim = Dimensions {
            width: 10,
            height: 10,
        };
        let mut grid = Grid::with_dim(dim).unwrap();
        side_winder::apply(&mut grid, &d)
            .err()
            .map(|e| println!("{}", e));

        println!("Toward direction {:?}", d);
        println!("{}", grid.to_ascii());
    }
    println!("Aldous Broder algorithm.");

    let dim = Dimensions {
        width: 10,
        height: 10,
    };
    let mut grid = Grid::with_dim(dim).unwrap();
    aldous_broder::apply(&mut grid)
        .err()
        .map(|e| println!("{}", e));
    println!("{}", grid.to_ascii());
}
