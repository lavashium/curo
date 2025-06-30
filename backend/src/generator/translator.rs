use crate::asm::*;
use language::*;

pub struct Translator {
    source_ast: AstProgram,
    result_program: AsmProgram,
}

impl Translator {
    pub fn new(source_ast: AstProgram) -> Self {
        Self {
            source_ast: source_ast,
            result_program: AsmProgram {
                function_definition: AsmFunction {
                    name: String::new(),
                    instructions: Vec::new(),
                },
            },
        }
    }

    pub fn parse(&mut self) -> AsmProgram {
        let source_ast = self.source_ast.clone();
        self.visit_program(&source_ast);
        self.result_program.clone()
    }
}

impl Visitor for Translator {
    fn visit_program(&mut self, program: &AstProgram) {
        program.function_definition.accept(self);
    }

    fn visit_function(&mut self, function: &AstFunction) {
        let mut instructions = Vec::new();

        self.result_program.function_definition.name = function.name.clone();

        let mut body_translator = FunctionBodyTranslator {
            instructions: &mut instructions,
        };
        function.body.accept(&mut body_translator);

        self.result_program.function_definition.instructions = instructions;
    }
}

struct FunctionBodyTranslator<'a> {
    instructions: &'a mut Vec<AsmInstruction>,
}

impl<'a> Visitor for FunctionBodyTranslator<'a> {
    fn visit_statement(&mut self, statement: &AstStatement) {
        match statement {
            AstStatement::Return { expression } => {
                expression.accept(self);
                self.instructions.push(AsmInstruction::Ret);
            }
        }
    }

    fn visit_expression(&mut self, _expression: &AstExpression) {
        // match expression {
        //     AstExpression::Constant { constant } => {
        //         if let Ok(value) = constant.parse::<i32>() {
        //             self.instructions.push(AsmInstruction::Mov {
        //                 source: AsmOperand::Imm(value),
        //                 dest: AsmOperand::Register,
        //             });
        //         } else {
        //             panic!("Invalid constant: {}", constant);
        //         }
        //     }
        // }
    }
}
