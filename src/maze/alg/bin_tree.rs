use super::super::grid::*;
use rand::*;

pub fn apply(grid: &mut Grid) -> Result<&str, &str> {
  let rng = rand::thread_rng();
  for cursor in 0..grid.area() {
    let open_east = grid.look(cursor, &Direction::East).is_some();
    let open_south = grid.look(cursor, &Direction::South).is_some();

    if open_south && open_east {
      if random() {
        grid.carve(cursor, &Direction::South).unwrap();
      } else {
        grid.carve(cursor, &Direction::East).unwrap();
      }
    } else if open_east {
      grid.carve(cursor, &Direction::East).unwrap();
    } else if open_south {
      grid.carve(cursor, &Direction::South).unwrap();
    }
  }
  Ok("Operation succeeded")
}
