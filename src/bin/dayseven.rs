use std::{collections::HashMap, io::{self, Write}, str::FromStr};

use itertools::Itertools;

static INPUT: &str = include_str!("../input/dayseven.txt");

#[derive(Clone)]
struct ManifoldState{ coords: Vec<Coordinate>, last_visited_row_index: Option<isize> }

impl ManifoldState {
    fn from_input(input: Vec<&str>) -> Result<Self, &'static str> {
        let coords: Vec<Coordinate> = input.iter().enumerate().flat_map(|(row_index, row)| {
            row.chars().enumerate().map(move |(char_index, char)| {
                let manifold_object = ManifoldObject::from_str(&char.to_string()).unwrap();
                Coordinate{ x: row_index as isize, y: char_index as isize, manifold_object, visited: false }
            })
        }).collect();
        Ok(ManifoldState{ coords, last_visited_row_index: None })
    }

    fn progress_beam(&self) -> ManifoldState {
        match self.last_visited_row_index {
            None => {
                let updated_coords: Vec<Coordinate> = self.coords.iter()
                    .map(|coord| {
                        if coord.x == 0 { 
                            coord.mark_visited()
                        } else {
                            coord.clone()
                        }
                    })
                    .collect();

                ManifoldState { coords: updated_coords, last_visited_row_index: Some(0) }
            },
            Some(last_visited_row_index) => {
                let row_index = last_visited_row_index + 1;
                let current_row = self.get_row(row_index);
                if current_row.is_empty() {
                    return self.clone();
                }

                let coords_with_transformations_from_row_above: Vec<Coordinate> = self.coords
                    .iter()
                    .map(|coord|{
                        if coord.x == row_index {
                            match (self.get_object_above(coord), &coord.manifold_object) {
                                (ManifoldObject::Origin, _) | (ManifoldObject::TachyonBeam, ManifoldObject::Space) => 
                                    coord.activate_tachyon_beam().mark_visited(),
                                (ManifoldObject::TachyonBeam, ManifoldObject::Splitter { .. }) => 
                                    coord.activate_splitter().mark_visited(),
                                _ => 
                                    coord.mark_visited(),
                            }
                        }
                        else {
                            coord.clone()
                        } 
                    })
                    .collect();

                let updated_coords: Vec<Coordinate> = coords_with_transformations_from_row_above.iter()
                    .map(|coord| {
                        if coord.x == row_index && Self::has_triggered_splitter_adjacent(&coords_with_transformations_from_row_above, coord) {
                            coord.activate_tachyon_beam()
                        } else {
                            coord.clone()
                        }
                    })
                    .collect();

                ManifoldState { coords: updated_coords, last_visited_row_index: Some(row_index) }
            },
        }
    }

    fn get_row(&self, index: isize) -> Vec<&Coordinate> {
        self.coords.iter().filter(|c: &&Coordinate| c.x == index).sorted_by_key(|c| c.y).collect()
    }

    fn get_object_above(&self, coord: &Coordinate) -> &ManifoldObject {
        self.coords
            .iter()
            .find(|c| c.x == coord.x - 1 && c.y == coord.y)
            .map(|c| &c.manifold_object)
            .unwrap()
    }

    fn has_triggered_splitter_adjacent(coords: &[Coordinate], coord: &Coordinate) -> bool {
        coords
            .iter()
            .any(|c| c.x == coord.x 
                && (c.y == coord.y - 1 || c.y == coord.y + 1)
                && matches!(c.manifold_object, ManifoldObject::Splitter { triggered: true }))
}

    fn number_rows(&self) -> isize {
        self.coords.iter().max_by_key(|c| c.x).unwrap().x
    }

    fn fully_progressed(&self) -> bool {
        match self.last_visited_row_index {
            Some(index) => index == self.number_rows(),
            None => false,
        }
    }

    fn display_animated(&self, window_size: isize) {
        use std::fmt::Write as FmtWrite;
        
        let current_row = self.last_visited_row_index.unwrap_or(0);
        let max_row = self.number_rows();
        let max_col = self.coords.iter().map(|c| c.y).max().unwrap_or(0);
        
        let start_row = (current_row - window_size).max(0);
        let end_row = (current_row + window_size).min(max_row);
        
        let mut frame = String::with_capacity(((end_row - start_row + 1) * (max_col + 10)) as usize);
        
        frame.push_str("\x1B[2J\x1B[1;1H"); // Clear display to simulate animation

        writeln!(frame, "┌─ Tachyon Manifold ─── Row {}/{} ───┐", current_row, max_row).unwrap();
        writeln!(frame).unwrap();
        
        for x in start_row..=end_row {
            let current_line_indicator = if x == current_row { "►" } else { " " };
            write!(frame, "{} ", current_line_indicator).unwrap();
            
            for y in 0..=max_col {
                if let Some(coord) = self.coords.iter().find(|c| c.x == x && c.y == y) {
                    let display = match &coord.manifold_object {
                        ManifoldObject::Origin => "\x1B[33mS\x1B[0m",      // Yellow
                        ManifoldObject::TachyonBeam => "\x1B[36m│\x1B[0m", // Cyan
                        ManifoldObject::Splitter { triggered: true } => "\x1B[32m^\x1B[0m",  // Green
                        ManifoldObject::Splitter { triggered: false } => "\x1B[31m^\x1B[0m", // Red
                        ManifoldObject::Space => "·",                      
                    };
                    frame.push_str(display);
                }
            }
            writeln!(frame).unwrap();
        }
        
        writeln!(frame).unwrap();
    
        print!("{}", frame);
        io::stdout().flush().unwrap();
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Coordinate { x: isize, y: isize, manifold_object: ManifoldObject, visited: bool }

impl Coordinate {
    fn activate_tachyon_beam(&self) -> Coordinate {
        Coordinate {
            manifold_object: ManifoldObject::TachyonBeam,
            ..self.clone()
        }
    }

    fn activate_splitter(&self) -> Coordinate {
        Coordinate {
            manifold_object: ManifoldObject::Splitter { triggered: true },
            ..self.clone()
        }
    }

    fn mark_visited(&self) -> Coordinate {
        Coordinate {
            visited: true,
            ..self.clone()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ManifoldObject {
    Origin,
    Space,
    Splitter { triggered: bool },
    TachyonBeam
}

impl FromStr for ManifoldObject {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "S" => Ok(ManifoldObject::Origin),
            "." => Ok(ManifoldObject::Space),
            "|" => Ok(ManifoldObject::TachyonBeam),
            "^" => Ok(ManifoldObject::Splitter { triggered: false }),
            _ => Err("Unrecognised object. Manifold collapsing")
        }
    }
}

struct Dag {
    edges: HashMap<Coordinate, Vec<Coordinate>>
}

impl Dag {
    fn from_manifold_state(mf: &ManifoldState) -> Dag {
        let coords_with_objects: Vec<Coordinate> = mf.coords.iter()
            .filter(|c| matches!(
                &c.manifold_object,
                ManifoldObject::Origin | ManifoldObject::TachyonBeam | ManifoldObject::Splitter { triggered: true }
            ))
            .cloned()
            .collect();

        let edges = coords_with_objects.iter().map(|coord| {
            let children: Vec<Coordinate> = coords_with_objects.iter().filter(|c| {
                let is_beam_child = c.x == coord.x + 1 && c.y == coord.y
                    && matches!(coord.manifold_object, ManifoldObject::Origin | ManifoldObject::TachyonBeam);

                let is_splitter_child = c.x == coord.x && (c.y == coord.y - 1 || c.y == coord.y + 1)
                    && matches!(coord.manifold_object, ManifoldObject::Splitter { triggered: true })
                    && matches!(c.manifold_object, ManifoldObject::TachyonBeam);

                is_beam_child || is_splitter_child
            })
            .cloned()
            .collect();
            (coord.clone(), children)
        }).collect();

        Dag { edges }
    }
    fn count_paths_from_node(&self, node: Coordinate, cache: &mut HashMap<Coordinate, usize>) -> usize {
        if cache.contains_key(&node) {
            return cache[&node]
        } else if self.edges.get(&node).unwrap().is_empty() {
            return 1
        }
        let total = self.edges.get(&node).unwrap().iter().map(|child| self.count_paths_from_node(child.clone(), cache)).sum();
        cache.insert(node, total);
        total
    }

    fn count_all_paths(&self) -> usize {
        let mut cache: HashMap<Coordinate, usize> = HashMap::new();
        let starting_node = self.edges.iter()
            .find(|(c, _)| matches!(c.manifold_object, ManifoldObject::Origin))
            .unwrap().0
            .clone();
        self.count_paths_from_node(starting_node, &mut cache)
    }
}

fn part_one(input: Vec<&str>) -> Result<usize, String> {
    let initial_state = ManifoldState::from_input(input)?;
    let manifold_states: Vec<ManifoldState> = std::iter::successors(
        Some(initial_state.clone()),
        |current| {
            let next_state = current.progress_beam();
            Some(next_state)
        }
    )
    .inspect(|state| state.display_animated(20))
    .take_while(|state| !state.fully_progressed())
    .collect();

    let final_state = manifold_states.iter().next_back().unwrap();
    let activated_splitter_count = final_state.coords.iter().filter(|c| matches!(c.manifold_object, ManifoldObject::Splitter { triggered: true })).count();

    Ok(activated_splitter_count)
}

fn part_two(input: Vec<&str>) -> Result<usize, String> {
    let initial_state = ManifoldState::from_input(input)?;
    let manifold_states: Vec<ManifoldState> = std::iter::successors(
        Some(initial_state.clone()),
        |current| {
            let next_state = current.progress_beam();
            Some(next_state)
        }
    )
    .take_while(|state| !state.fully_progressed())
    .collect();

    let final_state = manifold_states.iter().next_back().unwrap().progress_beam();
    final_state.display_animated(20);

    let dag = Dag::from_manifold_state(&final_state);
    let total_paths = dag.count_all_paths();
    Ok(total_paths)
}

fn main() {
    let result = part_one(INPUT.lines().collect::<Vec<&str>>());
    println!("\nPart one: {:?}", result);
    let result = part_two(INPUT.lines().collect::<Vec<&str>>());
    println!("Part two: {:?}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        // Given
        let input = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];
        
        // When
        let result = part_one(input).unwrap();
        
        // Then
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part_two() {
        // Given
        let input = vec![
            ".......S.......",
            "...............",
            ".......^.......",
            "...............",
            "......^.^......",
            "...............",
            ".....^.^.^.....",
            "...............",
            "....^.^...^....",
            "...............",
            "...^.^...^.^...",
            "...............",
            "..^...^.....^..",
            "...............",
            ".^.^.^.^.^...^.",
            "...............",
        ];
        
        // When
        let result = part_two(input).unwrap();
        
        // Then
        assert_eq!(result, 40);
    }
}