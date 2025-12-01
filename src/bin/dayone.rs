use std::{str::FromStr};

static INPUT: &str = include_str!("../input/dayone.txt");

// struct for dial
#[derive(Debug, Copy, Clone)]
struct Dial {
    position: i32
}

// impl that applies an instruction
impl Dial {
    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::RotateLeft { degree } =>
                self.position = ((self.position - degree) % 100 + 100) % 100,
            Instruction::RotateRight { degree } =>
                self.position = (self.position + degree) % 100,
            
        }
    }
}

// enum for instruction
#[derive(PartialEq, Debug)]
pub enum Instruction {
    RotateLeft { degree: i32 },
    RotateRight { degree: i32 },
}

impl FromStr for Instruction {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let first = s.chars().next().ok_or("Empty string was passed")?;
        let tail = &s[first.len_utf8()..];

        let degree: i32 = tail
            .parse()
            .map_err(|_| "Failed to parse degree")?;

        match first {
            'L' => 
                Ok(Instruction::RotateLeft { degree }),
            'R' => 
                Ok(Instruction::RotateRight { degree }),
            _ => 
                Err("Unexpected instruction character"),
        }
    }
}


fn part_one() -> Result<(), String> {
    let instructions: Vec<Instruction> = INPUT.lines()
        .map(|line| Instruction::from_str(line))
        .collect::<Result<_, _>>()?;
    let dial = Dial{ position: 50 };


    let states: Vec<Dial> = instructions.iter().scan(dial, |state, instruction| {
        let mut next_state = *state;
        next_state.apply(instruction);
        *state = next_state;
        Some(next_state)
    }).collect();

    let zeros = states.iter().filter(|d| d.position == 0).count();
    println!("Position is 0 {} times", zeros);

    println!("Final dial position: {}", dial.position);
    Ok(())
}


fn main() -> Result<(), String> {
    return part_one();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_instruction_from_str_for_rotate_left() {
        // Given
        let input = "L32";
        let expected = Ok(Instruction::RotateLeft { degree: 32 });
        
        // When
        let actual = Instruction::from_str(input);
        
        // Then
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_instruction_from_str_for_rotate_right() {
        // Given
        let input = "R250";
        let expected = Ok(Instruction::RotateRight { degree: 250 });

        // When
        let actual = Instruction::from_str(input);

        // Then
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_instruction_from_str_fails_when_not_l_or_r() {
        // Given
        let input = "H123";
        
        // When
        // Then
        assert_eq!(Instruction::from_str(input), Err("Unexpected instruction character"))
    }

    #[test]
    fn test_instruction_from_str_fails_when_bad_number() {
        // Given
        let input = "L23P";
        
        // When
        // Then
        assert_eq!(Instruction::from_str(input), Err("Failed to parse degree"))
    }

}