use super::super::grid::*;
use rand::seq::SliceRandom;

pub fn apply(grid: &mut Grid, direction: &Direction) -> Result<String, String> {
  let (corner, traversal_order, primary_dir, secondary_dir) = match direction {
    Direction::North => (
      Corner::SouthEast,
      TraversalOrder::RowWise,
      Direction::North,
      Direction::West,
    ),
    Direction::East => (
      Corner::SouthWest,
      TraversalOrder::ColumnWise,
      Direction::East,
      Direction::North,
    ),
    Direction::South => (
      Corner::NorthWest,
      TraversalOrder::RowWise,
      Direction::South,
      Direction::East,
    ),
    Direction::West => (
      Corner::NorthEast,
      TraversalOrder::ColumnWise,
      Direction::West,
      Direction::South,
    ),
  };

  let mut current_run: Vec<usize> = vec![];

  for cursor in grid.traverse(&traversal_order, &corner) {
    current_run.push(cursor);

    let open_primary = grid.look(cursor, &primary_dir);
    let open_secondary = grid.look(cursor, &secondary_dir);

    let result = match (open_primary, open_secondary) {
      (Some(_), Some(_)) => {
        if rand::random() {
          grid.carve(cursor, &secondary_dir)
        } else {
          let to_carve = current_run.choose(&mut rand::thread_rng());
          let result = match to_carve {
            Some(cursor) => grid.carve(*cursor, &primary_dir),
            None => Err(CarveError::missing_neighbor()),
          };
          current_run.clear();
          result
        }
      }
      (Some(_), None) => {
        let to_carve = current_run.choose(&mut rand::thread_rng());
        let result = match to_carve {
          Some(cursor) => grid.carve(*cursor, &primary_dir),
          None => Err(CarveError::missing_neighbor()),
        };
        current_run.clear();
        result
      }
      (None, Some(_)) => grid.carve(cursor, &secondary_dir),
      (None, None) => Ok(0),
    };

    if result.is_err() {
      let msg = "Unexpected error carving maze: ".to_string() + result.unwrap_err().msg();
      return Err(msg);
    }
  }
  Ok("Operation succeeded".to_string())
}
