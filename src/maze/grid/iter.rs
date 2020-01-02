use super::*;

pub enum Corner {
  NorthWest,
  NorthEast,
  SouthWest,
  SouthEast,
}

pub enum TraversalOrder {
  RowWise = 1,
  ColumnWise = 2,
}

pub struct IterGrid {
  pos: usize,
  max: usize,
  iterator: Box<dyn Fn(usize) -> usize>,
}

impl IterGrid {
  pub fn new(
    height: usize,
    width: usize,
    traversal_order: TraversalOrder,
    start_corner: Corner,
  ) -> IterGrid {
    let (row_wise, col_wise) = match traversal_order {
      TraversalOrder::RowWise => (1, 0),
      TraversalOrder::ColumnWise => (0, 1),
    };
    let (reverse_row, reverse_col) = match start_corner {
      Corner::NorthWest => (false, false),
      Corner::NorthEast => (false, true),
      Corner::SouthWest => (true, false),
      Corner::SouthEast => (true, true),
    };

    let row_fn = move |position: usize| {
      let row_idx = row_wise * (position / width) + col_wise * (position % height);
      if reverse_row {
        return width - 1 - row_idx;
      } else {
        return row_idx;
      }
    };

    let col_fn = move |position: usize| {
      let col_idx = row_wise * (position % width) + col_wise * (position / height);
      if reverse_col {
        return height - 1 - col_idx;
      } else {
        return col_idx;
      }
    };

    let iterator = move |position: usize| {
      return col_fn(position) + row_fn(position) * width;
    };
    return IterGrid {
      pos: 0,
      iterator: Box::new(iterator),
      max: width * height,
    };
  }

  fn iterator(&self) -> &Box<dyn Fn(usize) -> usize> {
    &self.iterator
  }
}

impl Iterator for IterGrid {
  type Item = usize;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos >= self.max {
      None
    } else {
      let next_val = self.iterator()(self.pos);
      self.pos = self.pos + 1;
      return Some(next_val);
    }
  }
}
