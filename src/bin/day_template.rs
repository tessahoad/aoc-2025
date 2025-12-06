static INPUT: &str = include_str!("../input/dayfour.txt");

fn part_one(_input: Vec<&str>) -> Result<usize, String> {
    todo!()
}

fn part_two(_input: Vec<&str>) -> Result<usize, String> {
    todo!()
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
        let input = vec!["a string"];
        
        // When
        let result = part_one(input).unwrap();
        
        // Then
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part_two() {
        // Given
        let input = vec!["a string"];

        
        // When
        let result = part_two(input).unwrap();
        
        // Then
        assert_eq!(1, 1);
    }
}