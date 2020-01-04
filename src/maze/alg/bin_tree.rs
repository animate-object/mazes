use super::super::grid::*;
use rand::*;

pub fn apply(grid: &mut Grid, corner: &'static Corner) -> Result<String, String> {
  for cursor in grid.traverse(&TraversalOrder::RowWise, corner) {
    let (d1, d2) = corner.to_directions();

    let open_one = grid.look(cursor, &d1).is_some();
    let open_two = grid.look(cursor, &d2).is_some();

    let result = match (open_one, open_two) {
      (true, true) => {
        if random() {
          grid.carve(cursor, &d1)
        } else {
          grid.carve(cursor, &d2)
        }
      }
      (true, false) => grid.carve(cursor, &d1),
      (false, true) => grid.carve(cursor, &d2),
      (false, false) => Ok(0),
    };

    if result.is_err() {
      let msg = "Unexpected error carving maze: ".to_string() + result.unwrap_err().msg();
      return Err(msg);
    }
  }
  Ok("Operation succeeded".to_string())
}
