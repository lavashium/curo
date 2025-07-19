use super::*;
use language::*;
use common::*;

impl<'scp, 'ctx> GeneratorTransforms<'scp, 'ctx> {
    pub fn transform_expression(&mut self, expression: &mut AstExpression) -> (Vec<TacInstruction>, TacVal) {
        <Self as Factory<(Vec<TacInstruction>, TacVal), Self, AstExpression>>::run(self, expression)
    }
}

impl<'scp, 'ctx> Factory<(Vec<TacInstruction>, TacVal), Self, AstExpression> for GeneratorTransforms<'scp, 'ctx> {
    fn run(driver: &mut Self, expression: &mut AstExpression) -> (Vec<TacInstruction>, TacVal) {
        match expression {
            AstExpression::Constant { constant, .. } => {
                let val = TacVal::new_constant(constant.clone());
                (vec![], val)
            }
            AstExpression::Unary { operator, operand, .. } => {
                let (mut instructions, source) = driver.transform_expression(operand);

                let destination = TacVal::new_var(driver.ctx.ctx.tempgen.temp());

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

                    let result = TacVal::new_var(driver.ctx.ctx.tempgen.temp());

                    let short_circuit_label = driver.ctx.ctx.tempgen.label(match operator {
                        AstBinaryKind::And => "and_false",
                        AstBinaryKind::Or => "or_true",
                        _ => unreachable!(),
                    });

                    let end_label = driver.ctx.ctx.tempgen.label("end");

                    let (mut instr_left, val_left) = driver.transform_expression(left);
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

                    let (mut instr_right, val_right) = driver.transform_expression(right);
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

                    let (mut instructions_left, dest_left) = driver.transform_expression(left);
                    let (mut instructions_right, dest_right) = driver.transform_expression(right);

                    let destination = TacVal::new_var(driver.ctx.ctx.tempgen.temp());

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
                    let (mut rhs_instrs, rhs_val) = driver.transform_expression(right);
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
            AstExpression::Conditional { condition, then_branch, else_branch, .. } => {
                let else_label = driver.ctx.ctx.tempgen.label("cond_else");
                let end_label = driver.ctx.ctx.tempgen.label("cond_end");

                let (mut cond_instrs, cond_val) = driver.transform_expression(condition);

                let result = TacVal::new_var(driver.ctx.ctx.tempgen.temp());

                let mut instructions = Vec::new();
                instructions.append(&mut cond_instrs);

                instructions.push(TacInstruction::JumpIfZero {
                    condition: cond_val.clone(),
                    target: else_label.clone(),
                });

                let (mut then_instrs, then_val) = driver.transform_expression(then_branch);
                instructions.append(&mut then_instrs);

                instructions.push(TacInstruction::Copy {
                    src: then_val,
                    dst: result.clone(),
                });

                instructions.push(TacInstruction::Jump {
                    target: end_label.clone(),
                });

                instructions.push(TacInstruction::Label(else_label));

                let (mut else_instrs, else_val) = driver.transform_expression(else_branch);
                instructions.append(&mut else_instrs);

                instructions.push(TacInstruction::Copy {
                    src: else_val,
                    dst: result.clone(),
                });

                instructions.push(TacInstruction::Label(end_label));

                (instructions, result)
            }
            AstExpression::FunctionCall { identifier, args, span } => {
                let mut instructions = vec![];
                let mut tac_args = Vec::new();

                for arg in args {
                    let (mut arg_instrs, arg_val) = driver.transform_expression(arg);
                    instructions.append(&mut arg_instrs);
                    tac_args.push(arg_val);
                }

                let result = TacVal::new_var(driver.ctx.ctx.tempgen.temp());
                instructions.push(TacInstruction::FunCall {
                    fun_name: identifier.clone(),
                    args: tac_args,
                    dst: result.clone(),
                });

                (instructions, result)
            }
        }
    }
}