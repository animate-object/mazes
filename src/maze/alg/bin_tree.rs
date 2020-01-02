use super::super::grid::*;
use rand::*;

pub fn apply(grid: &mut Grid) -> Result<String, String> {
  for cursor in grid.traverse(iter::TraversalOrder::RowWise, iter::Corner::NorthWest) {
    let open_east = grid.look(cursor, &Direction::East).is_some();
    let open_south = grid.look(cursor, &Direction::South).is_some();

    let result = match (open_south, open_east) {
      (true, true) => {
        if random() {
          grid.carve(cursor, &Direction::South)
        } else {
          grid.carve(cursor, &Direction::East)
        }
      }
      (true, false) => grid.carve(cursor, &Direction::South),
      (false, true) => grid.carve(cursor, &Direction::East),
      (false, false) => Ok("Reached opposite corner"),
    };

    if result.is_err() {
      let msg = "Unexpected error carving maze: ".to_string() + result.unwrap_err().msg();
      return Err(msg);
    }
  }
  Ok("Operation succeeded".to_string())
}
