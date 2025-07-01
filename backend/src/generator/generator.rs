use crate::asm::*;
use language::*;

pub struct AsmGenerator;

impl AsmGenerator {
    pub fn new() -> Self {
        Self
    }

    pub fn generate(&self, program: TacProgram) -> AsmProgram {
        self.visit_program(program)
    }
}

impl AsmGenerator {
    fn visit_program(&self, program: TacProgram) -> AsmProgram {
        let function = program.function_definition;
        let function_definition = self.visit_function(function);
        AsmProgram {
            function_definition,
        }
    }

    fn visit_function(&self, function: TacFunction) -> AsmFunction {
        let identifier = function.identifier;
        let mut instructions = Vec::new();
        for instruction in function.instructions {
            let mut instruction = self.visit_instruction(instruction);
            instructions.append(&mut instruction);
        }
        AsmFunction {
            identifier,
            instructions,
        }
    }

    fn visit_instruction(&self, instruction: TacInstruction) -> Vec<AsmInstruction> {
        let mut instructions = Vec::new();
        match instruction {
            TacInstruction::Return { val } => {
                instructions.push(AsmInstruction::Mov {
                    src: self.convert_operand(&val),
                    dst: AsmOperand::Reg(AsmReg::AX),
                });
                instructions.push(AsmInstruction::Ret);
            }
            TacInstruction::Unary {
                unary_operator,
                source,
                destination,
            } => {
                instructions.push(AsmInstruction::Mov {
                    src: self.convert_operand(&source),
                    dst: self.convert_operand(&destination),
                });
                instructions.push(AsmInstruction::Unary {
                    unary_operator: self.convert_unary_operator(&unary_operator),
                    operand: self.convert_operand(&destination),
                });
            }
        }
        instructions
    }

    fn convert_operand(&self, val: &TacVal) -> AsmOperand {
        match val {
            TacVal::Constant(s) => {
                let parsed = s.parse::<i32>().expect("invalid constant integer");
                AsmOperand::Imm(parsed)
            }
            TacVal::Var(ident) => AsmOperand::Pseudo(ident.clone()),
        }
    }

    fn convert_unary_operator(&self, op: &UnaryKind) -> AsmUnaryOperator {
        match op {
            UnaryKind::Negation => AsmUnaryOperator::Neg,
            UnaryKind::Complement => AsmUnaryOperator::Not,
        }
    }
}
