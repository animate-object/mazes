use ::rand::{self, Rng};
use ::std::convert::TryFrom;

#[derive(Debug)]
pub struct Dimensions {
  width: u32,
  height: u32,
}

#[derive(Debug)]
pub struct MazeGen {
  unvisited: Vec<u32>,
  cells: Vec<u8>,
  dim: Dimensions,
}

impl MazeGen {
  pub fn with_dim(dim: Dimensions) -> Result<MazeGen, &'static str> {
    let maybe_area = usize::try_from(dim.width * dim.height);
    match maybe_area {
      Ok(area) => Ok(MazeGen {
        unvisited: (0..(dim.width * dim.height)).collect(),
        cells: vec![0b00000000; area],
        dim,
      }),
      Err(_) => Err("Error allocating MazeGenerator. Maximum area of a maze is 2^32."),
    }
  }

  pub fn with_side_len(len: u32) -> Result<MazeGen, &'static str> {
    let dim = Dimensions {
      width: len,
      height: len,
    };
    MazeGen::with_dim(dim)
  }

  pub fn area(&self) -> u32 {
    return self.dim.height * self.dim.width;
  }

  pub fn random_cursor(&self) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, self.area())
  }

  // look (given a cursor and a direction, inspect cell in direction)

  // walk (given a cursor and a direction, modify the mazegen to mark the cell as visited)
}
