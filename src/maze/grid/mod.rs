use ::rand::{
  self,
  distributions::{Distribution, Standard},
  seq::SliceRandom,
  Rng,
};
use ::std::convert::TryFrom;
use std::slice::Iter;

pub mod iter;

#[derive(Debug, Copy, Clone)]
pub struct Dimensions {
  pub width: usize,
  pub height: usize,
}

#[derive(Debug)]
pub struct Grid {
  cells: Vec<u8>,
  dim: Dimensions,
}

impl Grid {
  pub fn with_dim(dim: &Dimensions) -> Result<Grid, &'static str> {
    let maybe_area = usize::try_from(dim.width * dim.height);
    match maybe_area {
      Ok(area) => Ok(Grid {
        cells: vec![0b00000000; area],
        dim: dim.clone(),
      }),
      Err(_) => Err("Error allocating Griderator."),
    }
  }

  #[allow(dead_code)]
  pub fn square(len: usize) -> Result<Grid, &'static str> {
    let dim = Dimensions {
      width: len,
      height: len,
    };
    Grid::with_dim(&dim)
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

  #[allow(dead_code)]
  pub fn random_cursor(&self) -> usize {
    let mut rng = rand::thread_rng();
    rng.gen_range(0, self.area())
  }

  pub fn find_valid_neighbor_idx<'cells>(&self, cursor: usize, dir: &Direction) -> Option<usize> {
    match dir {
      Direction::North => {
        if self.width() <= cursor {
          Some(cursor - self.width())
        } else {
          None
        }
      }
      Direction::South => {
        let idx = cursor + self.width();
        if idx < self.area() {
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
    }
  }

  pub fn look(&self, cursor: usize, dir: &Direction) -> Option<&u8> {
    self
      .find_valid_neighbor_idx(cursor, dir)
      .map(|idx| self.cells.get(idx))
      .flatten()
  }

  pub fn random_neighbor(&self, cursor: usize) -> Option<(Direction, usize)> {
    let mut open: Vec<(Direction, usize)> = vec![];
    for d in Direction::iterator() {
      let neighbor_idx = self.find_valid_neighbor_idx(cursor, &d);
      if neighbor_idx.is_some() {
        open.push((*d, neighbor_idx.unwrap()));
      }
    }
    open
      .choose(&mut rand::thread_rng())
      .map(|val| val.to_owned())
  }

  pub fn carve<'cells>(&mut self, cursor: usize, dir: &Direction) -> CarveResult {
    let maybe_neighbor_idx = self.find_valid_neighbor_idx(cursor, dir);
    match maybe_neighbor_idx {
      Some(idx) => {
        let (maybe_cell, maybe_neighbor) = get_distinct_mut(cursor, idx, &mut self.cells);
        match (maybe_cell, maybe_neighbor) {
          (Some(cell), Some(neighbor)) => {
            remove_wall(cell, neighbor, dir);
            Ok(idx)
          }
          (Some(_), None) => Err(CarveError::missing_neighbor()),
          (None, Some(_)) => Err(CarveError::cursor_not_found()),
          _ => Err(CarveError::unexpected_error()),
        }
      }
      None => Err(CarveError::hit_boundary()),
    }
  }

  pub fn to_ascii(&self) -> String {
    let mut builder: String = String::from("+");
    builder.push_str(&"---+".repeat(self.width()));
    for row in 0..self.height() {
      let row_start = row * self.width();
      let row_end = (row + 1) * self.width();
      let mut easts = String::from("\n|");
      let mut souths = String::from("\n+");
      for cell_idx in row_start..row_end {
        if Direction::East.is_open(&self.cells[cell_idx]) {
          easts.push_str("    ");
        } else {
          easts.push_str("   |");
        }
        if Direction::South.is_open(&self.cells[cell_idx]) {
          souths.push_str("   +");
        } else {
          souths.push_str("---+");
        }
      }
      builder.push_str(&easts);
      builder.push_str(&souths);
    }
    builder
  }

  pub fn to_bytes(&self) -> Vec<u8> {
    self.cells.clone()
  }

  pub fn traverse(
    &self,
    traversal_order: &TraversalOrder,
    start_corner: &Corner,
  ) -> iter::GridIter {
    return iter::GridIter::new(self.height(), self.width(), traversal_order, start_corner);
  }
}

// Static helpers -------------------------------------------------------------

// Carve a passage in dir from cell to neighbor
// nb. a northern passage from a to b implies a southern passage from b to a
fn remove_wall(_cell: &mut u8, _neighbor: &mut u8, dir: &Direction) {
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

// Grid Enums ------------------------------------------------------------------

#[derive(Debug, Copy, Clone)]
pub enum Direction {
  North = 0b00001000,
  South = 0b00000100,
  East = 0b00000010,
  West = 0b00000001,
}

impl Direction {
  fn iterator() -> Iter<'static, Direction> {
    static DIRECTIONS: [Direction; 4] = [
      Direction::North,
      Direction::South,
      Direction::East,
      Direction::West,
    ];
    DIRECTIONS.into_iter()
  }

  fn value(&self) -> u8 {
    match self {
      Direction::North => 0b00001000,
      Direction::South => 0b00000100,
      Direction::East => 0b00000010,
      Direction::West => 0b00000001,
    }
  }

  fn inverse(&self) -> Direction {
    match self {
      Direction::North => Direction::South,
      Direction::East => Direction::West,
      Direction::South => Direction::North,
      Direction::West => Direction::East,
    }
  }

  fn is_open(&self, cell: &u8) -> bool {
    self.value() & cell > 0
  }
}

impl Distribution<Direction> for Standard {
  fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Direction {
    match rng.gen_range(0, 4) {
      0 => Direction::North,
      1 => Direction::East,
      2 => Direction::South,
      _ => Direction::West,
    }
  }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum Corner {
  NorthWest,
  NorthEast,
  SouthWest,
  SouthEast,
}

impl Corner {
  pub fn to_directions(&self) -> (Direction, Direction) {
    match self {
      Corner::NorthWest => (Direction::North, Direction::West),
      Corner::NorthEast => (Direction::North, Direction::East),
      Corner::SouthWest => (Direction::South, Direction::West),
      Corner::SouthEast => (Direction::South, Direction::East),
    }
  }
}

#[derive(Debug)]
#[allow(dead_code)]
pub enum TraversalOrder {
  RowWise = 1,
  ColumnWise = 2,
}

// Status ---------------------------------------------------------------------

pub struct CarveError {
  code: u8,
}

impl CarveError {
  pub fn msg(&self) -> &'static str {
    match self.code {
      0 => "Cursor not found.",
      1 => "Hit maze boundary.",
      2 => "Did not find neighbor where one was expected.",
      8 => "Unexpected error",
      _ => "Unspecified error code",
    }
  }

  pub fn with_code(code: u8) -> CarveError {
    CarveError { code: code }
  }

  pub fn cursor_not_found() -> CarveError {
    CarveError::with_code(0)
  }

  pub fn hit_boundary() -> CarveError {
    CarveError::with_code(1)
  }

  pub fn missing_neighbor() -> CarveError {
    CarveError::with_code(2)
  }

  pub fn unexpected_error() -> CarveError {
    CarveError::with_code(8)
  }
}

type CarveResult = Result<usize, CarveError>;
