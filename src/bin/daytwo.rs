use std::{fmt::Display, str::FromStr};

static INPUT: &str = include_str!("../input/daytwo.txt");

trait ProductIdValidator {
    fn is_valid_product_id_by_one_repetition(&self) -> bool;
    fn is_valid_product_id_by_many_repetitions(&self) -> bool;
}

impl ProductIdValidator for String {
    fn is_valid_product_id_by_one_repetition(&self) -> bool {
        let len = self.len();
        if len % 2 == 1 {
            return true;
        }
        let midpoint = len / 2;
        let (left, right) = self.split_at(midpoint);
        left.ne(right)
    }
    
    fn is_valid_product_id_by_many_repetitions(&self) -> bool {
        let len = self.len();
        let midpoint = len / 2;
        for i in 1..=midpoint {
            let (head, tail) = self.split_at(i);
            if tail == head {
                return false;
            }
            let mut acc = tail;
            while !acc.is_empty() {
                if acc.starts_with(head) {
                    acc = acc.strip_prefix(head).unwrap();
                } else {
                    break;
                }
            }
            if acc.is_empty() {
                return false
            }
        }
        true
    }
}

#[derive(Debug, PartialEq)]
struct ProductIdRange {
    lower_bound: String,
    upper_bound: String
}

impl Display for ProductIdRange {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.lower_bound, self.upper_bound)
    }
}

impl FromStr for ProductIdRange {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_bound = s.split('-').next(); 
        let upper_bound = s.split('-').next_back(); 


        match (lower_bound, upper_bound) {
            (Some(l_value), Some(r_value)) => 
                Ok(ProductIdRange{ lower_bound: l_value.to_string(), upper_bound: r_value.to_string() }),
            _ =>
                Err("Could nae parse")
        }
    }
}

fn invalid_ids_by<F>(ids: Vec<ProductIdRange>, op: F) -> Result<Vec<u128>, String>
where F: Fn(String) -> bool
{
    Ok(ids.iter().flat_map(|range| {
        let lower = range.lower_bound.parse::<u128>()
            .map_err(|_| "Not valid numbers in bounds".to_string()).ok()?;
        let upper = range.upper_bound.parse::<u128>()
            .map_err(|_| "Not valid numbers in bounds".to_string()).ok()?;
        

            let invalids: Vec<u128> = (lower..=upper).filter(|num| {
                !op(num.to_string())
            }).collect();
            Some(invalids)
        
    }).flatten().collect())
}

fn part_one() -> Result<(), String> {
    let lines: Vec<&str> = INPUT.lines().collect();
    let unparsed_input = lines.first().unwrap();
    let parsed_ranges: Vec<ProductIdRange> = unparsed_input.split(",").map(ProductIdRange::from_str).collect::<Result<_, _>>()?;

    let invalids = invalid_ids_by(parsed_ranges, |s| s.is_valid_product_id_by_one_repetition())?;

    let sum_of_invalids: u128 = invalids
        .iter()
        .sum();

    println!("Sum of invalids: {}", sum_of_invalids);
    
    Ok(())
}

fn part_two() -> Result<(), String> {
    let lines: Vec<&str> = INPUT.lines().collect();
    let unparsed_input = lines.first().unwrap();
    let parsed_ranges: Vec<ProductIdRange> = unparsed_input.split(",").map(ProductIdRange::from_str).collect::<Result<_, _>>()?;

    let invalids = invalid_ids_by(parsed_ranges, |s| s.is_valid_product_id_by_many_repetitions())?;

    let sum_of_invalids: u128 = invalids
        .iter()
        .sum();

    println!("Sum of invalids: {}", sum_of_invalids);
    
    Ok(())
}

fn main(){
    let _ = part_one();
    let _ = part_two();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_parses_range() {
        // Given
        let input: &str = "1188511880-1188511890";
        let expected =  Ok(ProductIdRange{ lower_bound: "1188511880".to_owned(), upper_bound: "1188511890".to_owned() });
        // When
        let actual = ProductIdRange::from_str(input);
        // Then
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_product_id_is_valid() {
        // Given
        let test_cases = vec![
            ("11", false),
            ("1212", false),
            ("123", true),
            ("1234", true),
            ("123123", false),
        ];

        for (input, expected) in test_cases {
            // When
            let actual = input.to_string().is_valid_product_id_by_one_repetition();
            // Then 
            assert_eq!(actual, expected, "Failed for input: {}", input);
        }
    }

    #[test]
    fn test_product_id_is_valid_part_two() {
        // Given
        let test_cases = vec![
            ("11", false),
            ("12341234", false),
            ("123", true),
            ("123412", true),
            ("123123123", false),
            ("1212121212", false),
            ("1111111", false),
            ("1231231", true)
        ];

        for (input, expected) in test_cases {
            // When
            let actual = input.to_string().is_valid_product_id_by_many_repetitions();
            // Then 
            assert_eq!(actual, expected, "Failed for input: {}", input);
        }
    }
}