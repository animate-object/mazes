use super::super::grid::*;

use std::collections::HashSet;

pub fn apply(grid: &mut Grid) -> Result<String, String> {
  let mut visited = HashSet::new();

  let mut cursor = grid.random_cursor();
  visited.insert(cursor);
  while visited.len() < grid.area() {
    let (next_direction, next_cursor) = grid.random_neighbor(cursor).expect("Nowhere to go!");

    if visited.contains(&next_cursor) {
      // retreading ground
      cursor = next_cursor;
    } else {
      // carving to an unvisited cell
      let carve_result = grid.carve(cursor, &next_direction);
      if carve_result.is_ok() {
        cursor = carve_result.ok().unwrap();
        visited.insert(cursor);
      } else {
        return Err("Error carving maze: ".to_string() + carve_result.err().unwrap().msg());
      }
    }
  }

  Ok("Operation succeeded".to_string())
}
