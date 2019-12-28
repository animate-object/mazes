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
  North,
  South,
  East,
  West,
  Up,
  Down,
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
    let (idx, offgrid) = match dir {
      Direction::North => {
        let idx = cursor - self.width();
        (idx, self.width() > cursor)
      }
      Direction::South => {
        let idx = cursor + self.width();
        (idx, idx > self.area())
      }
      Direction::East => (cursor + 1, cursor % self.width() == self.width() - 1),
      Direction::West => (cursor - 1, cursor % self.width() == 0),
      _ => (0, false),
    };

    if !offgrid {
      Some(idx)
    } else {
      None
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

pub fn remove_wall(_cell: &mut u8, _neighbor: &mut u8, dir: &Direction) {
  println!("Carving a {:?} wall ", dir);
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
