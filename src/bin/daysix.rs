use std::str::FromStr;


static INPUT: &str = include_str!("../input/daysix.txt");

#[derive(Copy, Clone)]
enum Operator {
    Add,
    Mulitply
}

impl FromStr for Operator {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Mulitply),
            _ => Err("Invalid operator"),
        }
    }
}

struct MathsProblem {
    operands: Vec<usize>,
    operator: Operator
}

impl MathsProblem {
    fn resolve(&self) -> usize {
        match self.operator {
            Operator::Add => {
                self.operands.iter().sum::<usize>()
            },
            Operator::Mulitply => {
                self.operands.iter().product::<usize>()
            },
        }
    }
}

fn part_one(input: Vec<&str>) -> Result<usize, String> {
    let whitespace_split_input: Vec<Vec<&str>> = input.iter()
        .map(|line| line.split_whitespace().collect())
        .collect();

    let problems: Vec<MathsProblem> = (0..whitespace_split_input.first().unwrap().len())
        .map(|i| {
            let raw_values: Vec<&str> = whitespace_split_input.iter()
                .map(|entries| *entries.get(i).unwrap())
                .collect();
            let raw_operator = raw_values.last().unwrap();
            let operator = Operator::from_str(raw_operator).unwrap();
            let operands: Vec<usize> = raw_values[..raw_values.len() - 1]
                .iter()
                .map(|operand| operand.parse::<usize>().unwrap())
                .collect();
            MathsProblem { operands, operator }
        })
        .collect();
    let resolved: Vec<usize> = problems.iter().map(|problem| problem.resolve()).collect();
    Ok(resolved.iter().sum())
}

fn part_two(input: Vec<&str>) -> Result<usize, String> {
    
    let operators: Vec<Operator> = input.last()
        .unwrap()
        .split_whitespace()
        .map(|s| Operator::from_str(s).unwrap())
        .collect();

    let number_rows: Vec<&str> = input[..input.len() - 1].to_vec();
    
    let columns: Vec<String> = (0..number_rows.first().unwrap().len())
        .map(|i| {
            number_rows.iter()
                .map(|row| row.chars().nth(i).unwrap_or_default())
                .collect()
        })
        .collect();

    let grouped_operands : Vec<Vec<String>>= columns
        .split(|col| col.trim().is_empty())
        .map(|slice| slice.to_vec())    
        .collect();

    let zipped: Vec<(Vec<String>, Operator)> = grouped_operands.into_iter().zip(operators).collect();

    let problems: Vec<MathsProblem> = zipped.iter().map(|(raw_operands, operator)| {
        let operands: Vec<usize> = raw_operands.iter().map(|o| o.trim().parse().unwrap()).collect();
        MathsProblem{ operands, operator: *operator }
    }).collect();
    let resolved: Vec<usize> = problems.iter().map(|problem| problem.resolve()).collect();
    Ok(resolved.iter().sum())
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
            "123 328  51 64 ",
            "45  64  387 23 ",
            "6   98  215 314",
            "*   +   *   +"
        ];
        
        // When
        let result = part_one(input).unwrap();
        
        // Then
        assert_eq!(result, 4277556);
    }

   #[test]
    fn test_part_two() {
        // Given
        let input = vec![
            "123 328  51 64 ",
            " 45 64  387 23 ",
            "  6 98  215 314",
            "*   +   *   +"
        ];
        
        // When
        let result = part_two(input).unwrap();
        
        // Then
        assert_eq!(result, 3263827);
    }
}