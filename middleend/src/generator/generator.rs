use language::*;

pub struct TacGenerator {
    tempgen: TempGen,
}

impl TacGenerator {
    pub fn new() -> Self {
        Self {
            tempgen: TempGen::new(),
        }
    }

    pub fn parse(&mut self, program: AstProgram) -> TacProgram {
        self.visit_program(program)
    }
}

impl TacGenerator {
    fn visit_program(&mut self, program: AstProgram) -> TacProgram {
        let function = program.function_definition;
        let function_definition = self.visit_function(function);
        TacProgram {
            function_definition,
        }
    }

    fn visit_function(&mut self, function: AstFunction) -> TacFunction {
        let identifier = function.name;
        let statement = function.body;
        let instructions = self.visit_statement(statement);
        TacFunction {
            identifier,
            instructions,
        }
    }

    fn visit_statement(&mut self, statement: AstStatement) -> Vec<TacInstruction> {
        let mut instructions: Vec<TacInstruction> = Vec::new();
        match statement {
            AstStatement::Return { expression } => {
                let (mut expression, value) = self.visit_expression(expression);
                instructions.append(&mut expression);
                instructions.push(TacInstruction::Return { val: value });
            }
        }
        return instructions;
    }

    fn visit_expression(&mut self, expression: AstExpression) -> (Vec<TacInstruction>, TacVal) {
        match expression {
            AstExpression::Constant { constant } => {
                let val = TacVal::Constant(constant);
                (vec![], val)
            }

            AstExpression::Unary { operator, operand } => {
                let (mut instructions, source) = self.visit_expression(*operand);

                let destination = self.tempgen.next();

                instructions.push(TacInstruction::Unary {
                    operator: operator,
                    source,
                    destination: destination.clone(),
                });

                (instructions, destination)
            }

            AstExpression::Binary {
                operator,
                left,
                right,
            } => {
                let mut instructions = Vec::new();

                let (mut instructions_left, dest_left) = self.visit_expression(*left);
                let (mut instructions_right, dest_right) = self.visit_expression(*right);

                let dest = self.tempgen.next();

                instructions.append(&mut instructions_left);
                instructions.append(&mut instructions_right);

                instructions.push(TacInstruction::Binary {
                    operator: operator,
                    source1: dest_left,
                    source2: dest_right,
                    destination: dest.clone(),
                });

                (instructions, dest)
            }
        }
    }
}
