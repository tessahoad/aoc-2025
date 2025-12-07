use std::{fmt::{self, Display}, str::FromStr};

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
}

impl fmt::Display for ManifoldState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coords.is_empty() {
            return Ok(());
        }


        let max_x = self.coords.iter().map(|c| c.x).max().unwrap_or(0);
        let max_y = self.coords.iter().map(|c| c.y).max().unwrap_or(0);
        
        for x in 0..=max_x {
            for y in 0..=max_y {
                let coord = self.coords.iter()
                    .find(|c| c.x == x && c.y == y);
                
                match coord {
                    Some(c) => write!(f, "{}", c.manifold_object.to_string())?,
                    _ => return Err(fmt::Error),
                }
            }
            if x < max_x {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, PartialEq)]
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

impl Display for ManifoldObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ManifoldObject::Origin => write!(f, "S"),
            ManifoldObject::Space => write!(f, "."),
            ManifoldObject::Splitter { .. } => write!(f, "^"),
            ManifoldObject::TachyonBeam => write!(f, "|"),
        }
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
    .inspect(
        |state| {
        println!("Manifold state:\n{} \n", "Analyzed");
    })
    .take_while(|state| !state.fully_progressed())
    .collect();

    let final_state = manifold_states.iter().next_back().unwrap();
    let activated_splitter_count = final_state.coords.iter().filter(|c| matches!(c.manifold_object, ManifoldObject::Splitter { triggered: true })).count();

    Ok(activated_splitter_count)
}

fn part_two(input: Vec<&str>) -> Result<usize, String> {
    todo!()
}

fn main() {
    let result = part_one(INPUT.lines().collect::<Vec<&str>>());
    println!("Part one: {:?}", result);
    // let result = part_two(INPUT.lines().collect::<Vec<&str>>());
    // println!("Part two: {:?}", result);
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
        assert_eq!(1, 1);
    }
}