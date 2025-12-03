use std::{fmt::Display, str::FromStr};

use itertools::Itertools;

static INPUT: &str = include_str!("../input/daythree.txt");

#[derive(Debug, PartialEq)]
struct Bank {
    batteries: String,
}

impl Bank {
    fn largest_joltage(&self) -> u32 {
        let len = self.batteries.len();
        let (head, _) = self.batteries.split_at(len - 1);

        let max_first_digit = head.chars().filter_map(|c| c.to_digit(10)).max();
        let position_max_first_char = head.chars()
            .position(|c| c.to_digit(10) == max_first_digit)
            .unwrap();

        let (_, remaining) = self.batteries.split_at(position_max_first_char + 1);

        let max_second_digit = remaining.chars().filter_map(|c| c.to_digit(10)).max();
        let position_max_second_char = remaining.chars()
            .position(|c| c.to_digit(10) == max_second_digit)
            .unwrap();


        let largest_joltage_string = head.chars().nth(position_max_first_char).unwrap().to_string() + &remaining.chars().nth(position_max_second_char).unwrap().to_string();
        return largest_joltage_string.parse::<u32>().unwrap();
    }
}

impl Display for Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.batteries)
    }
}


fn part_one() -> Result<(), String> {
    let lines: Vec<&str> = INPUT.lines().collect();
    let banks: Vec<Bank> = lines.iter().map(|line| Bank{ batteries: line.to_string() }).collect();
    let joltage_sum: u32 = banks.iter().map(Bank::largest_joltage).sum();
    println!("Sum of joltage: {}", joltage_sum);
    Ok(())
}

fn part_two() -> Result<(), String> {
    todo!()
}

fn main(){
    let _ = part_one();
    // let _ = part_two();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_bank_largest_joltage() {
        // Given
        let test_cases = vec![
            (Bank{ batteries: "987654321111111".to_string() }, 98),
            (Bank{ batteries: "811111111111119".to_string() }, 89),
            (Bank{ batteries: "234234234234278".to_string() }, 78),
            (Bank{ batteries: "818181911112111".to_string() }, 92),
        ];

        for (input, expected) in test_cases {
            // When
            let actual = input.largest_joltage();
            // Then
            assert_eq!(actual, expected, "Failed for input: {}", input)
        }
    }
}