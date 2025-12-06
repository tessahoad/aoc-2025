use std::fmt;

static INPUT: &str = include_str!("../input/dayfour.txt");

#[derive(Clone)]
struct Grid{ coords: Vec<Coordinate> }

#[derive(Debug, Clone)]
struct Coordinate { x: u32, y: u32, has_paper_roll: bool }

// Written by Claude
impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coords.is_empty() {
            return Ok(());
        }
        
        let max_x = self.coords.iter().map(|c| c.x).max().unwrap_or(0);
        let max_y = self.coords.iter().map(|c| c.y).max().unwrap_or(0);
        
        for y in 0..=max_y {
            for x in 0..=max_x {
                let coord = self.coords.iter()
                    .find(|c| c.x == x && c.y == y);
                
                match coord {
                    Some(c) if c.has_paper_roll => write!(f, "@")?,
                    _ => write!(f, ".")?,
                }
            }
            if y < max_y {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Grid {
    

    fn neighbours(&self, coord: &Coordinate) -> Vec<&Coordinate> {
        self.coords.iter().filter(|maybe_neighbour| {
            if maybe_neighbour.x == coord.x && maybe_neighbour.y == coord.y {
                return false;
            }
            let x_dist = maybe_neighbour.x.abs_diff(coord.x);
            let y_dist = maybe_neighbour.y.abs_diff(coord.y);
            x_dist <= 1 && y_dist <= 1
        }).collect()
    }

    fn neighbours_with_rolls(&self, coord: &Coordinate) -> Vec<&Coordinate> {
        self.neighbours(coord).into_iter().filter(|c| c.has_paper_roll).collect()
    }

    fn removable_rolls(&self) -> Vec<Coordinate> {
        self.coords.iter()
            .filter(|coord| coord.has_paper_roll)
            .filter(|coord| self.neighbours_with_rolls(coord).len() < 4)
            .cloned()
            .collect()
    }

    fn remove_rolls(&self, manouverable_coords: &[Coordinate]) -> Grid {
        let removed_rolls = self.coords.iter().map(|coord| {
            if manouverable_coords.iter().any(|c| c.x == coord.x && c.y == coord.y) {
                Coordinate{ x: coord.x, y: coord.y, has_paper_roll: false }
            } else {
                coord.clone()
            }
        }).collect();
        Grid{ coords: removed_rolls }
    }
}

// Written by Claude
fn parse_grid(raw_grid: Vec<&str>) -> Grid {
    let coords: Vec<Coordinate> = raw_grid.iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, char)| Coordinate {
                    x: x as u32,
                    y: y as u32,
                    has_paper_roll: char == '@',
                })
        })
        .collect();
    Grid{ coords }
}

fn part_one(input: &str) -> Result<usize, String> {
    let lines: Vec<&str> = input.lines().collect();
    let grid = parse_grid(lines);
    let manouverable_coords = &grid.removable_rolls();
    Ok(manouverable_coords.len())
}

fn part_two(input: &str) -> Result<usize, String> {
    let lines: Vec<&str> = input.lines().collect();
    let initial_grid = parse_grid(lines);

    let grids_with_counts: Vec<usize> = std::iter::successors(
        Some(initial_grid.clone()),
        |current| {
            let removable = current.removable_rolls();
            if removable.is_empty() {
                None
            } else {
                Some(current.remove_rolls(&removable))
            }
        }
    )
    .inspect(
        |grid| {
        println!("Grid state:\n {} \n", grid);
    })
    .map(|grid| {
        let removable = grid.removable_rolls();
        
        removable.len()
    })
    .collect();

    let removed_roll_count = grids_with_counts.iter().sum();
    
    Ok(removed_roll_count)
}

fn main() {
    let result = part_one(INPUT);
    println!("Part one: {:?}", result);
    let result = part_two(INPUT);
    println!("Part two: {:?}", result);
}

// Written by Claude
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // Given
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ].join("\n");
        
        // When
        let result = part_one(&input).unwrap();
        
        // Then
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two() {
        // Given
        let input = vec![
            "..@@.@@@@.",
            "@@@.@.@.@@",
            "@@@@@.@.@@",
            "@.@@@@..@.",
            "@@.@@@@.@@",
            ".@@@@@@@.@",
            ".@.@.@.@@@",
            "@.@@@.@@@@",
            ".@@@@@@@@.",
            "@.@.@@@.@.",
        ].join("\n");
        
        // When
        let result = part_two(&input).unwrap();
        
        // Then
        assert_eq!(result, 43);
    }

    #[test]
    fn test_manouverable_coords_dense_random_grid() {
        // Given
        // @.@@.
        // @@.@.
        // .@@@@
        // @.@.@
        // .@@@.
        let mut coords = Vec::new();
        for y in 0..5 {
            for x in 0..5 {
                let has_roll = match (x, y) {
                    (0, 0) | (2, 0) | (3, 0) => true,
                    (0, 1) | (1, 1) | (3, 1) => true,
                    (1, 2) | (2, 2) | (3, 2) | (4, 2) => true,
                    (0, 3) | (2, 3) | (4, 3) => true,
                    (1, 4) | (2, 4) | (3, 4) => true,
                    _ => false,
                };
                coords.push(Coordinate {
                    x,
                    y,
                    has_paper_roll: has_roll,
                });
            }
        }
        let grid = Grid { coords };
        
        // When
        let result = grid.removable_rolls();
        
        // Then - check corner coord (0,0) has 3 neighbours
        let corner = grid.coords.iter().find(|c| c.x == 0 && c.y == 0).unwrap();
        let corner_neighbours = grid.neighbours(corner);
        assert_eq!(corner_neighbours.len(), 3); // (1,0), (0,1), (1,1)
        
        // Then - check corner coord (0,0) has 2 neighbours with rolls: (0,1) and (1,1)
        let corner_neighbours_with_rolls = grid.neighbours_with_rolls(corner);
        assert_eq!(corner_neighbours_with_rolls.len(), 2); // (0,1) and (1,1)
        assert!(corner_neighbours_with_rolls.iter().all(|c| c.has_paper_roll));
        
        // Then - check manouverable coords count
        assert_eq!(result.len(), 10);
    }

    #[test]
    fn test_manouverable_coords_mixed_density() {
        // Given
        // @@...
        // @@...
        // .....
        // ..@@@
        // ..@@@
        let mut coords = Vec::new();
        for y in 0..5 {
            for x in 0..5 {
                let has_roll = (x <= 1 && y <= 1) || (x >= 2 && y >= 3);
                coords.push(Coordinate {
                    x,
                    y,
                    has_paper_roll: has_roll,
                });
            }
        }
        let grid = Grid { coords };
        
        // When
        let result = grid.removable_rolls();
        
        // Then - check coord (1,1) has 8 neighbours (it's not on edge)
        let center_top = grid.coords.iter().find(|c| c.x == 1 && c.y == 1).unwrap();
        let neighbours = grid.neighbours(center_top);
        assert_eq!(neighbours.len(), 8);
        
        // Then - check coord (1,1) has 3 neighbours with rolls: (0,0), (1,0), (0,1)
        let neighbours_with_rolls = grid.neighbours_with_rolls(center_top);
        assert_eq!(neighbours_with_rolls.len(), 3);
        assert!(neighbours_with_rolls.iter().any(|c| c.x == 0 && c.y == 0));
        assert!(neighbours_with_rolls.iter().any(|c| c.x == 1 && c.y == 0));
        assert!(neighbours_with_rolls.iter().any(|c| c.x == 0 && c.y == 1));
        
        // Then - check manouverable coords count
        assert_eq!(result.len(), 8);
    }

    #[test]
    fn test_manouverable_coords_center_cluster() {
        // Given
        // .....
        // ..@..
        // .@@@.
        // ..@..
        // .....
        let mut coords = Vec::new();
        for y in 0..5 {
            for x in 0..5 {
                let has_roll = (x == 2 && y == 1) || 
                               (x == 1 && y == 2) || (x == 2 && y == 2) || (x == 3 && y == 2) ||
                               (x == 2 && y == 3);
                coords.push(Coordinate {
                    x,
                    y,
                    has_paper_roll: has_roll,
                });
            }
        }
        let grid = Grid { coords };
        
        // When
        let result = grid.removable_rolls();
        
        // Then - check center coord (2,2) has 8 neighbours
        let center = grid.coords.iter().find(|c| c.x == 2 && c.y == 2).unwrap();
        let centre_neighbours = grid.neighbours(center);
        assert_eq!(centre_neighbours.len(), 8);
        
        // Then - check center coord (2,2) has 4 neighbours with rolls
        let centre_neighbours_with_rolls = grid.neighbours_with_rolls(center);
        assert_eq!(centre_neighbours_with_rolls.len(), 4);
        assert!(centre_neighbours_with_rolls.iter().any(|c| c.x == 2 && c.y == 1));
        assert!(centre_neighbours_with_rolls.iter().any(|c| c.x == 1 && c.y == 2));
        assert!(centre_neighbours_with_rolls.iter().any(|c| c.x == 3 && c.y == 2));
        assert!(centre_neighbours_with_rolls.iter().any(|c| c.x == 2 && c.y == 3));
        
        // Then - check manouverable coords count (all edge coords have < 4 neighbours with rolls)
        assert_eq!(result.len(), 4);
    }
}
