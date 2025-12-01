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
pub enum Instruction {
    RotateLeft { degree: i32 },
    RotateRight { degree: i32 },
}

impl Instruction {
    fn from_str(s: &str) -> Result<Instruction, &str> {
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


fn main() -> Result<(), String> {
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
    
    // for instr in &instructions {
    //     dial.apply(instr);
    // }

    println!("Final dial position: {}", dial.position);
    Ok(())
}