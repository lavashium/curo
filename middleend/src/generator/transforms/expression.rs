use super::*;
use language::*;

pub trait ExpressionTransform {
    fn transform_expression(&mut self, expression: &AstExpression)
        -> (Vec<TacInstruction>, TacVal);
}

impl<'a> ExpressionTransform for GeneratorTransforms<'a> {
    fn transform_expression(&mut self, expression: &AstExpression) -> (Vec<TacInstruction>, TacVal) {
        match expression {
            AstExpression::Constant { constant, .. } => {
                let val = TacVal::new_constant(constant.clone());
                (vec![], val)
            }
            AstExpression::Unary { operator, operand, .. } => {
                let (mut instructions, source) = self.transform_expression(operand);

                let destination = TacVal::new_var(self.generator.tempgen.temp());

                instructions.push(TacInstruction::new_unary(
                    operator.to_tac().unwrap(),
                    source,
                    destination.clone(),
                ));

                (instructions, destination)
            }
            AstExpression::Binary { operator, left, right, .. } => {
                if *operator == AstBinaryKind::And || *operator == AstBinaryKind::Or {
                    let mut instructions = vec![];

                    let result = TacVal::new_var(self.generator.tempgen.temp());

                    let short_circuit_label = self.generator.tempgen.label(match operator {
                        AstBinaryKind::And => "and_false",
                        AstBinaryKind::Or => "or_true",
                        _ => unreachable!(),
                    });

                    let end_label = self.generator.tempgen.label("end");

                    let (mut instr_left, val_left) = self.transform_expression(left);
                    instructions.append(&mut instr_left);

                    match operator {
                        AstBinaryKind::And => {
                            instructions.push(TacInstruction::JumpIfZero {
                                condition: val_left.clone(),
                                target: short_circuit_label.clone(),
                            });
                        }
                        AstBinaryKind::Or => {
                            instructions.push(TacInstruction::JumpIfNotZero {
                                condition: val_left.clone(),
                                target: short_circuit_label.clone(),
                            });
                        }
                        _ => unreachable!(),
                    }

                    let (mut instr_right, val_right) = self.transform_expression(right);
                    instructions.append(&mut instr_right);

                    match operator {
                        AstBinaryKind::And => {
                            instructions.push(TacInstruction::JumpIfZero {
                                condition: val_right.clone(),
                                target: short_circuit_label.clone(),
                            });

                            instructions.push(TacInstruction::Copy {
                                src: TacVal::Constant("1".to_string()),
                                dst: result.clone(),
                            });
                        }
                        AstBinaryKind::Or => {
                            instructions.push(TacInstruction::JumpIfNotZero {
                                condition: val_right.clone(),
                                target: short_circuit_label.clone(),
                            });

                            instructions.push(TacInstruction::Copy {
                                src: TacVal::Constant("0".to_string()),
                                dst: result.clone(),
                            });
                        }
                        _ => unreachable!(),
                    }

                    instructions.push(TacInstruction::Jump {
                        target: end_label.clone(),
                    });

                    instructions.push(TacInstruction::Label(short_circuit_label.clone()));

                    match operator {
                        AstBinaryKind::And => {
                            instructions.push(TacInstruction::Copy {
                                src: TacVal::Constant("0".to_string()),
                                dst: result.clone(),
                            });
                        }
                        AstBinaryKind::Or => {
                            instructions.push(TacInstruction::Copy {
                                src: TacVal::Constant("1".to_string()),
                                dst: result.clone(),
                            });
                        }
                        _ => unreachable!(),
                    }

                    instructions.push(TacInstruction::Label(end_label));

                    (instructions, result)
                } else {
                    let mut instructions = Vec::new();

                    let (mut instructions_left, dest_left) = self.transform_expression(left);
                    let (mut instructions_right, dest_right) = self.transform_expression(right);

                    let destination = TacVal::new_var(self.generator.tempgen.temp());

                    instructions.append(&mut instructions_left);
                    instructions.append(&mut instructions_right);

                    instructions.push(TacInstruction::Binary {
                        operator: operator.to_tac().unwrap(),
                        source1: dest_left,
                        source2: dest_right,
                        destination: destination.clone(),
                    });

                    (instructions, destination)
                }
            }
            AstExpression::Var { identifier, .. } => {
                let val = TacVal::new_var(identifier.clone());
                (vec![], val)
            }

            AstExpression::Assignment { left, right, .. } => {
                if let AstExpression::Var { identifier, .. } = &**left {
                    let (mut rhs_instrs, rhs_val) = self.transform_expression(right);
                    let lhs = TacVal::new_var(identifier.clone());

                    rhs_instrs.push(TacInstruction::Copy {
                        src: rhs_val.clone(),
                        dst: lhs.clone(),
                    });

                    (rhs_instrs, lhs)
                } else {
                    panic!("Invalid assignment target; only simple variables supported");
                }
            }
        }
    }
}
