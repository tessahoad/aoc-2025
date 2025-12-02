use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct ProductIdRange {
    lower_bound: u128,
    upper_bound: u128
}

impl FromStr for ProductIdRange {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lower_bound_str = s.split('-').next(); 
        let upper_bound_str = s.split('-').last(); 

        let lower_bound = lower_bound_str
            .unwrap_or("")
            .parse()
            .map_err(|_| "Was not a number");

        let upper_bound = upper_bound_str
            .unwrap_or("")
            .parse()
            .map_err(|_| "Was not a number");

        match (lower_bound, upper_bound) {
            (Ok(l_value), Ok(r_value)) => 
                Ok(ProductIdRange{ lower_bound: l_value, upper_bound: r_value }),
            _ =>
                Err("Could nae parse")
        }
    }
}

fn part_one() -> Result<(), String> {
    let input = std::fs::read_to_string("input/daytwo.txt")
        .map_err(|e| format!("Failed to read input file: {}", e))?;
    
    let lines: Vec<&str> = input.lines().collect();
    
    for line in &lines {
        println!("Processing line for part one: {}", line);
    }
    
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
        let expected =  Ok(ProductIdRange{ lower_bound: 1188511880, upper_bound: 1188511890 });
        // When
        let actual = ProductIdRange::from_str(input);
        // Then
        assert_eq!(actual, expected);
    }
}