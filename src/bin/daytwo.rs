use std::{fmt::Display, str::FromStr};

static INPUT: &str = include_str!("../input/daytwo.txt");

trait ProductIdValidator {
    fn is_valid_product_id(&self) -> bool;
}

impl ProductIdValidator for String {
    fn is_valid_product_id(&self) -> bool {
        let len = &self.len();
        if len % 2 == 1 {
            return true;
        }
        let midpoint = len / 2;
        let (left, right) = self.split_at(midpoint);
        return left.ne(right);
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
        let upper_bound = s.split('-').last(); 


        match (lower_bound, upper_bound) {
            (Some(l_value), Some(r_value)) => 
                Ok(ProductIdRange{ lower_bound: l_value.to_string(), upper_bound: r_value.to_string() }),
            _ =>
                Err("Could nae parse")
        }
    }
}

fn part_one() -> Result<(), String> {
    let lines: Vec<&str> = INPUT.lines().collect();
    let unparsed_input = lines.first().unwrap();
    let parsed_ranges: Vec<ProductIdRange> = unparsed_input.split(",").map(ProductIdRange::from_str).collect::<Result<_, _>>()?;

    let invalids = parsed_ranges.iter().map(|range| {
        let l = range.lower_bound.parse::<u128>();
        let u = range.upper_bound.parse::<u128>();
        
        match (l, u) {
            (Ok(lower), Ok(upper)) => {
                let invalids: Vec<u128> = (lower..=upper).filter(|num| {
                    !num.to_string().is_valid_product_id()
                }).collect();
                return Ok(invalids);
            }
            _ => Err("Not valid numbers in bounds")
        }
    });

    // let all_invalids: Vec<String> = invalids
    //     .filter_map(Result::ok)
    //     .flatten()
    //     .map(|n| n.to_string())
    //     .collect();

    let sum_of_invalids: u128 = invalids
        .filter_map(Result::ok)
        .flatten()
        .sum();

    // println!("All invalids: {}", all_invalids.join(","));
    println!("Sum of invalids: {}", sum_of_invalids);
    
    Ok(())
}

fn main(){
    let _ = part_one();
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
            let actual = input.to_string().is_valid_product_id();
            // Then 
            assert_eq!(actual, expected, "Failed for input: {}", input);
        }
    }
}