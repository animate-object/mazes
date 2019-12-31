use ::rand::{self, Rng};
use ::std::convert::TryFrom;

#[derive(Debug)]
pub struct Dimensions {
  width: usize,
  height: usize,
}

#[derive(Debug)]
pub struct MazeGen {
  unvisited: Vec<usize>,
  cells: Vec<u8>,
  dim: Dimensions,
}

#[derive(Debug)]
pub enum Direction {
  North = 0b00001000,
  South = 0b00000100,
  East = 0b00000010,
  West = 0b00000001,
  Up = 0b00010000,
  Down = 0b00100000,
}

impl Direction {
  fn value(&self) -> u8 {
    match self {
      Direction::North => 0b00001000,
      Direction::South => 0b00000100,
      Direction::East => 0b00000010,
      Direction::West => 0b00000001,
      Direction::Up => 0b00010000,
      Direction::Down => 0b00100000,
    }
  }

  fn inverse(&self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::East => Direction::West,
      Direction::South => Direction::North,
      Direction::West => Direction::East,
      Direction::Up => Direction::Down,
      Direction::Down => Direction::Up,
    }
  }
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

  pub fn with_side_len(len: usize) -> Result<MazeGen, &'static str> {
    let dim = Dimensions {
      width: len,
      height: len,
    };
    MazeGen::with_dim(dim)
  }

  pub fn width(&self) -> usize {
    self.dim.width
  }

  pub fn height(&self) -> usize {
    self.dim.height
  }

  pub fn area(&self) -> usize {
    return self.dim.height * self.dim.width;
  }

  pub fn random_cursor(&self) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, self.area())
  }

  fn find_valid_neighbor_idx<'cells>(&self, cursor: usize, dir: &Direction) -> Option<usize> {
    match dir {
      Direction::North => {
        if self.width() >= cursor {
          Some(cursor - self.width())
        } else {
          None
        }
      }
      Direction::South => {
        let idx = cursor + self.width();
        if idx <= self.area() {
          Some(idx)
        } else {
          None
        }
      }
      Direction::East => {
        if self.width() < 1 || cursor % self.width() == self.width() - 1 {
          None
        } else {
          Some(cursor + 1)
        }
      }
      Direction::West => {
        if cursor % self.width() != 0 {
          Some(cursor - 1)
        } else {
          None
        }
      }
      _ => None,
    }
  }

  pub fn look(&self, cursor: usize, dir: &Direction) -> Option<&u8> {
    self
      .find_valid_neighbor_idx(cursor, dir)
      .map(|idx| self.cells.get(idx))
      .flatten()
  }

  pub fn carve<'cells>(
    &mut self,
    cursor: usize,
    dir: &Direction,
  ) -> Result<&'static str, &'static str> {
    let maybe_neighbor_idx = self.find_valid_neighbor_idx(cursor, dir);
    match maybe_neighbor_idx {
      Some(idx) => {
        let (maybe_cell, maybe_neighbor) = get_distinct_mut(cursor, idx, &mut self.cells);
        match (maybe_cell, maybe_neighbor) {
          (Some(cell), Some(neighbor)) => {
            remove_wall(cell, neighbor, dir);
            Ok("Carved new passage")
          }
          _ => Err("One of cell, neighbor, were not found"),
        }
      }
      None => Err("Invalid Neighbor"),
    }
  }
}

// Carve a passage in dir from cell to neighbor
// nb. a northern passage from a to b implies a southern passage from b to a
pub fn remove_wall(_cell: &mut u8, _neighbor: &mut u8, dir: &Direction) {
  *_cell = *_cell | dir.value();
  *_neighbor = *_neighbor | dir.inverse().value();
}

fn get_distinct_mut<T>(
  first: usize,
  second: usize,
  items: &mut [T],
) -> (Option<&mut T>, Option<&mut T>) {
  assert!(first != second);
  let split_at_index = if first < second { second } else { first };
  let (first_slice, second_slice) = items.split_at_mut(split_at_index);
  if first < second {
    (first_slice.get_mut(first), second_slice.get_mut(0))
  } else {
    (second_slice.get_mut(0), first_slice.get_mut(second))
  }
}
