use std::{str::FromStr};

static INPUT: &str = include_str!("../input/dayone.txt");

// struct for dial
#[derive(Debug, Copy, Clone)]
struct Dial {
    current_position: i32,
    zero_clicks: i32
}

// impl that applies an instruction
impl Dial {
    fn apply(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::RotateLeft { degree } => {
                let old_position = self.current_position;
                self.current_position = (self.current_position - degree).rem_euclid(100);

                if old_position == 0 && *degree >= 100  {
                    self.zero_clicks += degree.div_euclid(100);
                } else if old_position != 0 && *degree >= old_position {
                    let remaining = degree - old_position;
                    self.zero_clicks += remaining.div_euclid(100) + 1
                }           
            }
            Instruction::RotateRight { degree } => {
                let new_position = self.current_position + degree;
                self.current_position = new_position.rem_euclid(100);

                let zero_passes = new_position.div_euclid(100);
                self.zero_clicks += zero_passes.abs();
            }
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
    let dial = Dial{ current_position: 50, zero_clicks: 0 };


    let dial_states: Vec<Dial> = instructions.iter().scan(dial, |state, instruction| {
        let mut next_state = *state;
        next_state.apply(instruction);
        *state = next_state;
        Some(next_state)
    }).collect();

    let zeros = dial_states.iter().filter(|d| d.current_position == 0).count();
    println!("Position is 0 {} times", zeros);

    println!("Final dial position: {}", dial.current_position);
    Ok(())
}

fn part_two() -> Result<(), String> {
    let instructions: Vec<Instruction> = INPUT.lines()
        .map(|line| Instruction::from_str(line))
        .collect::<Result<_, _>>()?;
    let dial = Dial{ current_position: 50, zero_clicks: 0 };
    
    let dial_states: Vec<Dial> = instructions.iter().scan(dial, |state, instruction| {
        let mut next_state = *state;
        next_state.apply(instruction);
        *state = next_state;
        Some(next_state)
    }).collect();


    let final_state = dial_states.last().unwrap();

    println!("Final zero clicks: {}", final_state.zero_clicks);
    Ok(())
}

fn main() -> Result<(), String> {
    let _ = part_one();
    return part_two();
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

    #[test]
    fn test_zero_clicks_added_for_left_rotations_when_no_zero_pass() {
        // Given
        let mut dial = Dial { current_position: 50, zero_clicks: 0 }; 
        let instruction = Instruction::RotateLeft { degree: 45 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 5);
        assert_eq!(dial.zero_clicks, 0);
    }

    #[test]
    fn test_zero_clicks_added_for_left_rotations_when_no_zero_pass_and_zero_start() {
        // Given
        let mut dial = Dial { current_position: 0, zero_clicks: 0 }; 
        let instruction = Instruction::RotateLeft { degree: 45 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 55);
        assert_eq!(dial.zero_clicks, 0);
    }

    #[test]
    fn test_zero_clicks_added_for_left_rotations_when_land_on_zero() {
        // Given
        let mut dial = Dial { current_position: 50, zero_clicks: 0 }; 
        let instruction = Instruction::RotateLeft { degree: 350 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 0);
        assert_eq!(dial.zero_clicks, 4);
    }

    #[test]
    fn test_zero_clicks_added_for_left_rotations_when_land_on_nonzero_with_diff_greater_than_100() {
        // Given
        let mut dial = Dial { current_position: 50, zero_clicks: 0 }; 
        let instruction = Instruction::RotateLeft { degree: 355 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 95);
        assert_eq!(dial.zero_clicks, 4);
    }

    #[test]
    fn test_zero_clicks_added_for_left_rotations_when_start_nonzero_and_land_on_nonzero_with_diff_less_than_100() {
        // Given
        let mut dial = Dial { current_position: 50, zero_clicks: 0 }; 
        let instruction = Instruction::RotateLeft { degree: 55 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 95);
        assert_eq!(dial.zero_clicks, 1);
    }

    #[test]
    fn test_zero_clicks_added_for_left_rotations_when_start_nonzero_and_land_on_zero() {
        // Given
        let mut dial = Dial { current_position: 50, zero_clicks: 0 }; 
        let instruction = Instruction::RotateLeft { degree: 150 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 0);
        assert_eq!(dial.zero_clicks, 2);
    }

    #[test]
    fn test_zero_clicks_added_for_left_rotations_when_start_zero_and_land_on_zero() {
        // Given
        let mut dial = Dial { current_position: 0, zero_clicks: 0 }; 
        let instruction = Instruction::RotateLeft { degree: 200 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 0);
        assert_eq!(dial.zero_clicks, 2);
    }

    #[test]
    fn test_zero_clicks_added_for_right_rotations_when_start_nonzero_and_land_on_zero() {
        // Given
        let mut dial = Dial { current_position: 50, zero_clicks: 0 }; 
        let instruction = Instruction::RotateRight{ degree: 355 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 5);
        assert_eq!(dial.zero_clicks, 4);
    }

    #[test]
    fn test_zero_clicks_added_for_right_rotations_when_land_on_nonzero() {
        // Given
        let mut dial = Dial { current_position: 50, zero_clicks: 0 }; 
        let instruction = Instruction::RotateRight{ degree: 350 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 0);
        assert_eq!(dial.zero_clicks, 4);
    }

    #[test]
    fn test_zero_clicks_added_for_right_rotations_when_start_on_zero_and_land_nonzero() {
        // Given
        let mut dial = Dial { current_position: 0, zero_clicks: 0 }; 
        let instruction = Instruction::RotateRight{ degree: 350 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 50);
        assert_eq!(dial.zero_clicks, 3);
    }

    #[test]
    fn test_zero_clicks_added_for_right_rotations_when_start_on_zero_and_land_zero() {
        // Given
        let mut dial = Dial { current_position: 0, zero_clicks: 0 }; 
        let instruction = Instruction::RotateRight{ degree: 400 };

        // When
        dial.apply(&instruction);
        
        // Then
        assert_eq!(dial.current_position, 0);
        assert_eq!(dial.zero_clicks, 4);
    }

}