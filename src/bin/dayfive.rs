use std::cmp::{min, max};
use std::str::FromStr;

static INPUT: &str = include_str!("../input/dayfive.txt");

struct IngredientId(usize);

#[derive(PartialEq)]
enum IngedientState {
    Fresh,
    Spoiled,
}

impl FromStr for IngredientId {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(IngredientId(s.parse::<usize>().map_err(|_| "Could nae parse")?))
    }
}

#[derive(Clone)]
struct FreshIdRange {
    lower_bound: usize,
    upper_bound: usize
}

impl FreshIdRange {
    fn total_fresh_ids(&self) -> usize {
        self.upper_bound - self.lower_bound + 1
    }
}

impl FromStr for FreshIdRange {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_bound = s.split('-').next(); 
        let upper_bound = s.split('-').next_back(); 


        match (lower_bound, upper_bound) {
            (Some(l_value), Some(r_value)) => 
                Ok(FreshIdRange{ 
                    lower_bound: l_value.parse::<usize>().map_err(|_| "Could nae parse")?, 
                    upper_bound: r_value.parse::<usize>().map_err(|_| "Could nae parse")? 
                }),
            _ =>
                Err("Could nae parse")
        }
    }
}

fn part_one(input: Vec<&str>) -> Result<usize, String> {
    let fresh_id_ranges: Vec<FreshIdRange> = input.iter()
        .filter(|line| line.contains("-"))
        .flat_map(|s: &&str| FreshIdRange::from_str(s))
        .collect();

    let ingredient_ids: Vec<IngredientId> = input.iter()
        .filter(|line| !line.contains("-") && !line.is_empty())
        .flat_map(|s| s.parse::<IngredientId>())
        .collect();

    let ingredient_states: Vec<(IngredientId, IngedientState)> = ingredient_ids.into_iter().map(|id| {
        let id_in_any_range = fresh_id_ranges.iter().any(|range| range.lower_bound <= id.0 && id.0 <= range.upper_bound);
        match id_in_any_range {
            true => (id, IngedientState::Fresh),
            false => (id, IngedientState::Spoiled),
        }
        
    }).collect();
    let fresh_ingredients: Vec<(IngredientId, IngedientState)> = ingredient_states.into_iter()
    .filter(|(_, state)| *state == IngedientState::Fresh)
    .collect();
    Ok(fresh_ingredients.len())
}

fn add_new_range(mut acc: Vec<FreshIdRange>, range_to_add: FreshIdRange) -> Vec<FreshIdRange> {
    let overlap_index = acc.iter()
        .enumerate()
        .find(|(_, existing_range)| {
            existing_range.lower_bound <= range_to_add.upper_bound && 
            range_to_add.lower_bound <= existing_range.upper_bound
        })
        .map(|(index, _)| index);
    
    match overlap_index {
        None => {
            acc.push(range_to_add);
            acc
        },
        Some(index) => {
            let overlapping_range = acc.remove(index);
            let new_range = FreshIdRange {
                lower_bound: min(overlapping_range.lower_bound, range_to_add.lower_bound),
                upper_bound: max(overlapping_range.upper_bound, range_to_add.upper_bound)
            };
            add_new_range(acc, new_range)
        }
    }
}

fn part_two(input: Vec<&str>) -> Result<usize, String> {
    let fresh_id_ranges: Vec<FreshIdRange> = input.iter()
        .filter(|line| line.contains("-"))
        .flat_map(|s: &&str| FreshIdRange::from_str(s))
        .collect();

    let ranges_without_intersections = fresh_id_ranges
        .into_iter()
        .fold(vec![], add_new_range);

    let total_fresh_ids = ranges_without_intersections
        .iter()
        .map(|range| range.total_fresh_ids())
        .sum();

    Ok(total_fresh_ids)
}

fn main() {
    let result = part_one(INPUT.lines().collect::<Vec<&str>>());
    println!("Part one: {:?}", result);
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
            "3-5",
            "10-14",
            "16-20",
            "12-18",
            "",
            "1",
            "5",
            "8",
            "11",
            "17",
            "32",
        ];
        
        // When
        let result = part_one(input).unwrap();
        
        // Then
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part_two() {
        // Given
        let input = vec![
            "3-5",
            "10-14",
            "16-20",
            "12-18",
            "",
            "1",
            "5",
            "8",
            "11",
            "17",
            "32",
        ];
        
        // When
        let result = part_two(input).unwrap();
        
        // Then
        assert_eq!(result, 14);
    }
}