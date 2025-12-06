use std::fmt::Display;


static INPUT: &str = include_str!("../input/daythree.txt");

#[derive(Debug, PartialEq)]
struct Bank {
    batteries: String,
}

fn find_largest_first_place_of_n_digit_number_in_string(n: usize, string: String) -> usize {
    let len = string.len();
    let (head, _) = string.split_at(len - n + 1);

    let first_battery_value = head.chars().filter_map(|c| c.to_digit(10)).max();
    

    head.chars()
        .position(|c| c.to_digit(10) == first_battery_value)
        .unwrap()
}

impl Bank {
    fn two_digit_joltage(&self) -> u64 {
        let len = self.batteries.len();
        let (head, _) = self.batteries.split_at(len - 1);

        let first_battery_value = head.chars().filter_map(|c| c.to_digit(10)).max();
        let first_battery_position = head.chars()
            .position(|c| c.to_digit(10) == first_battery_value)
            .unwrap();

        let (_, remaining) = self.batteries.split_at(first_battery_position + 1);

        let second_battery_value = remaining.chars().filter_map(|c| c.to_digit(10)).max();
        let second_batter_position = remaining.chars()
            .position(|c| c.to_digit(10) == second_battery_value)
            .unwrap();

        let largest_joltage_string = head.chars().nth(first_battery_position).unwrap().to_string() + &remaining.chars().nth(second_batter_position).unwrap().to_string();
        largest_joltage_string.parse::<u64>().unwrap()
    }

    fn twelve_digit_joltage(&self) -> u64 { 
        let mut remaining = self.batteries.as_str();
        let joltage: String = (0..12).map( |i| {
            let index_next_battery = find_largest_first_place_of_n_digit_number_in_string(12 - i, remaining.to_string());
            let next_battery_value = remaining.chars().nth(index_next_battery).unwrap();
            remaining = &remaining[index_next_battery + 1..];
            next_battery_value
        }).collect();

        joltage.parse::<u64>().unwrap()
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
    let joltage_sum: u64 = banks.iter().map(Bank::two_digit_joltage).sum();
    println!("Sum of joltage: {}", joltage_sum);
    Ok(())
}

fn part_two() -> Result<(), String> {
    let lines: Vec<&str> = INPUT.lines().collect();
    let banks: Vec<Bank> = lines.iter().map(|line| Bank{ batteries: line.to_string() }).collect();
    let joltage_sum: u64 = banks.iter().map(Bank::twelve_digit_joltage).sum();
    println!("Sum of joltage: {}", joltage_sum);
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
    fn test_bank_two_digit_joltage() {
        // Given
        let test_cases = vec![
            (Bank{ batteries: "987654321111111".to_string() }, 98),
            (Bank{ batteries: "811111111111119".to_string() }, 89),
            (Bank{ batteries: "234234234234278".to_string() }, 78),
            (Bank{ batteries: "818181911112111".to_string() }, 92),
        ];

        for (input, expected) in test_cases {
            // When
            let actual = input.two_digit_joltage();
            // Then
            assert_eq!(actual, expected, "Failed for input: {}", input)
        }
    }

    #[test]
    fn test_bank_twelve_digit_joltage() {
        // Given
        let test_cases = vec![
            (Bank{ batteries: "987654321111111".to_string() }, 987654321111),
            (Bank{ batteries: "811111111111119".to_string() }, 811111111119),
            (Bank{ batteries: "234234234234278".to_string() }, 434234234278),
            (Bank{ batteries: "818181911112111".to_string() }, 888911112111),
        ];

        for (input, expected) in test_cases {
            // When
            let actual = input.twelve_digit_joltage();
            // Then
            assert_eq!(actual, expected, "Failed for input: {}", input)
        }
    }
}