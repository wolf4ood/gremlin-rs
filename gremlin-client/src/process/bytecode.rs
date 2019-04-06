use crate::GValue;

#[derive(Debug, PartialEq)]
pub struct Bytecode {
    source_instructions: Vec<Instruction>,
    step_instructions: Vec<Instruction>,
}

impl Default for Bytecode {
    fn default() -> Bytecode {
        Bytecode {
            source_instructions: vec![],
            step_instructions: vec![],
        }
    }
}
impl Bytecode {
    pub fn new() -> Bytecode {
        Default::default()
    }

    pub fn add_source(&mut self, source_name: String, args: Vec<GValue>) {
        self.source_instructions
            .push(Instruction::new(source_name, args));
    }
    pub fn add_step(&mut self, step_name: String, args: Vec<GValue>) {
        self.step_instructions
            .push(Instruction::new(step_name, args));
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    operator: String,
    args: Vec<GValue>,
}

impl Instruction {
    pub fn new(operator: String, args: Vec<GValue>) -> Instruction {
        Instruction { operator, args }
    }
}
