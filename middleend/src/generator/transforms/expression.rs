use super::*;
use language::*;
use common::*;

impl Factory<(Vec<TacInstruction>, TacVal), TypedExpression, TacGenContext<'_, '_>> for GeneratorTransforms{
    fn run(expression: &mut TypedExpression, ctx: &mut TacGenContext) -> (Vec<TacInstruction>, TacVal) {
        match expression {
            TypedExpression::Constant { constant, .. } => {
                let val = TacVal::new_constant(constant.clone());
                (vec![], val)
            }
            TypedExpression::Unary { operator, operand, .. } => {
                let (mut instructions, source) = Self::run_box(operand, ctx);

                let destination = TacVal::new_var(ctx.ctx.tempgen.temp());

                instructions.push(TacInstruction::new_unary(
                    operator.to_tac().unwrap(),
                    source,
                    destination.clone(),
                ));

                (instructions, destination)
            }
            TypedExpression::Binary { operator, left, right, .. } => {
                if *operator == AstBinaryKind::And || *operator == AstBinaryKind::Or {
                    let mut instructions = vec![];

                    let result = TacVal::new_var(ctx.ctx.tempgen.temp());

                    let short_circuit_label = ctx.ctx.tempgen.label(match operator {
                        AstBinaryKind::And => "and_false",
                        AstBinaryKind::Or => "or_true",
                        _ => unreachable!(),
                    });

                    let end_label = ctx.ctx.tempgen.label("end");

                    let (mut instr_left, val_left) = Self::run_box(left, ctx);
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

                    let (mut instr_right, val_right) = Self::run_box(right, ctx);
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

                    let (mut instructions_left, dest_left) = Self::run_box(left, ctx);
                    let (mut instructions_right, dest_right) = Self::run_box(right, ctx);

                    let destination = TacVal::new_var(ctx.ctx.tempgen.temp());

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
            TypedExpression::Var { identifier, .. } => {
                let val = TacVal::new_var(identifier.clone());
                (vec![], val)
            }

            TypedExpression::Assignment { left, right, .. } => {
                if let TypedExpression::Var { identifier, .. } = &**left {
                    let (mut rhs_instrs, rhs_val) = Self::run_box(right, ctx);
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
            TypedExpression::Conditional { condition, then_branch, else_branch, .. } => {
                let else_label = ctx.ctx.tempgen.label("cond_else");
                let end_label = ctx.ctx.tempgen.label("cond_end");

                let (mut cond_instrs, cond_val) = Self::run_box(condition, ctx);

                let result = TacVal::new_var(ctx.ctx.tempgen.temp());

                let mut instructions = Vec::new();
                instructions.append(&mut cond_instrs);

                instructions.push(TacInstruction::JumpIfZero {
                    condition: cond_val.clone(),
                    target: else_label.clone(),
                });

                let (mut then_instrs, then_val) = Self::run_box(then_branch, ctx);
                instructions.append(&mut then_instrs);

                instructions.push(TacInstruction::Copy {
                    src: then_val,
                    dst: result.clone(),
                });

                instructions.push(TacInstruction::Jump {
                    target: end_label.clone(),
                });

                instructions.push(TacInstruction::Label(else_label));

                let (mut else_instrs, else_val) = Self::run_box(else_branch, ctx);
                instructions.append(&mut else_instrs);

                instructions.push(TacInstruction::Copy {
                    src: else_val,
                    dst: result.clone(),
                });

                instructions.push(TacInstruction::Label(end_label));

                (instructions, result)
            }
            TypedExpression::FunctionCall { identifier, args, .. } => {
                let mut instructions = vec![];
                let mut tac_args = Vec::new();

                for arg in args {
                    let (mut arg_instrs, arg_val) = Self::run_box(arg, ctx);
                    instructions.append(&mut arg_instrs);
                    tac_args.push(arg_val);
                }

                let result = TacVal::new_var(ctx.ctx.tempgen.temp());
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