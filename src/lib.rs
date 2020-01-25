mod maze;

use grid::Grid;
use maze::{alg, grid};

pub use grid::{Corner, Dimensions, Direction};

pub enum Algorithm {
    AlduousBroder,
    SideWinder { traversal_direction: Direction },
    BinTree { start_corner: Corner },
}

impl Algorithm {
    fn apply(&self, grid: &mut Grid) -> Result<String, String> {
        match &self {
            Algorithm::AlduousBroder => alg::aldous_broder::apply(grid),
            Algorithm::SideWinder {
                traversal_direction,
            } => alg::side_winder::apply(grid, traversal_direction),
            Algorithm::BinTree { start_corner } => alg::bin_tree::apply(grid, &start_corner),
        }
    }
}

pub enum OutputType {
    ASCII,
    BIN,
}

impl OutputType {
    fn generate(&self, grid: &mut Grid) -> Output {
        match &self {
            OutputType::ASCII => Output::ASCII(grid.to_ascii()),
            OutputType::BIN => Output::BIN(grid.to_bytes()),
        }
    }
}

pub enum Output {
    ASCII(String),
    BIN(Vec<u8>),
}

pub struct Args {
    output_type: OutputType,
    dimensions: Dimensions,
    algorigthm: Algorithm,
}

pub fn generate(args: Args) -> Result<Output, String> {
    let mut grid = Grid::with_dim(&args.dimensions)?;
    args.algorigthm.apply(&mut grid)?;
    Ok(args.output_type.generate(&mut grid))
}
