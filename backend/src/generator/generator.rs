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
                operator,
                source,
                destination,
            } => {
                instructions.push(AsmInstruction::Mov {
                    src: self.convert_operand(&source),
                    dst: self.convert_operand(&destination),
                });
                instructions.push(AsmInstruction::Unary {
                    unary_operator: self.convert_unary_operator(&operator),
                    operand: self.convert_operand(&destination),
                });
            }
            TacInstruction::Binary { operator, source1, source2, destination } => {
                if operator == BinaryKind::Divide {
                    instructions.push(AsmInstruction::Mov {
                        src: self.convert_operand(&source1),
                        dst: AsmOperand::Reg(AsmReg::AX),
                    });
                    instructions.push(AsmInstruction::Cdq);
                    instructions.push(AsmInstruction::Idiv { 
                        operand: self.convert_operand(&source2),
                    });
                    instructions.push(AsmInstruction::Mov { 
                        src: AsmOperand::Reg(AsmReg::AX), 
                        dst: self.convert_operand(&destination),
                    });
                } else if operator == BinaryKind::Remainder {
                    instructions.push(AsmInstruction::Mov {
                        src: self.convert_operand(&source1),
                        dst: AsmOperand::Reg(AsmReg::AX),
                    });
                    instructions.push(AsmInstruction::Cdq);
                    instructions.push(AsmInstruction::Idiv { 
                        operand: self.convert_operand(&source2),
                    });
                    instructions.push(AsmInstruction::Mov { 
                        src: AsmOperand::Reg(AsmReg::DX), 
                        dst: self.convert_operand(&destination),
                    });
                } else {
                    instructions.push(AsmInstruction::Mov {
                        src: self.convert_operand(&source1),
                        dst: self.convert_operand(&destination),
                    });
                    instructions.push(AsmInstruction::Binary { 
                        binary_operator: self.convert_binary_operator(&operator), 
                        operand1: self.convert_operand(&source2), 
                        operand2: self.convert_operand(&destination) 
                    });
                }
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
            UnaryKind::Negate => AsmUnaryOperator::Neg,
            UnaryKind::Complement => AsmUnaryOperator::Not,
        }
    }

    fn convert_binary_operator(&self, op: &BinaryKind) -> AsmBinaryOperator {
        match op {
            BinaryKind::Add => AsmBinaryOperator::Add,
            BinaryKind::Subtract => AsmBinaryOperator::Sub,
            BinaryKind::Multiply => AsmBinaryOperator::Mult,
            _ => unreachable!()
        }
    }
}
