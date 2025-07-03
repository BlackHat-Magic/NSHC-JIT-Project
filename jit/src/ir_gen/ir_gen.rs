use std::collections::HashMap;

use crate::parser::ast::*;
use crate::ir::ir::*;

fn map_data_type(data_type: &DataType) -> IRType {
    match data_type {
        DataType::U8 => IRType::U8,
        DataType::I32 => IRType::I32,
        // ... other data types
        _ => panic!("Unimplemented data type mapping"),
    }
}

pub struct IrGenerator {
    // per-function
    variables: HashMap<String, IRType>,     // track variable names
    instructions: Vec<IRInstruction>,       // Instructions inside the current function
    function_name: String,
    parameters: Vec<(IRType, String)>,
    return_type: IRType,
    functions: Vec<IRFunction>,
    label_count: usize,
}

impl IrGenerator {
    pub fn new() -> Self {
        IrGenerator {
            variables: HashMap::new(),
            instructions: Vec::new(),
            function_name: String::new(),
            parameters: Vec::new(),
            return_type: IRType::Void,
            functions: Vec::new(),
            label_count: 0,
        }
    }

    fn next_label(&mut self) -> String {
        self.label_count += 1;
        format!("L{}", self.label_count)
    }

    pub fn generate_ir(&mut self, top_levels: &Vec<TopLevel>) -> Result<IRProgram, String> {
        for top_level in top_levels {
            match top_level {
                TopLevel::Function(func) => {
                self.function_name = func.name.clone();
                self.parameters = func.parameters.clone().into_iter().map(|(dt, s)| (map_data_type(&dt), s)).collect();
                self.return_type = func.return_type.clone().map(|dt| map_data_type(&dt)).unwrap_or(IRType::Void);
                self.variables.clear();
                self.instructions.clear();

                //Add paramters to variable
                for (dt, s) in &self.parameters {
                    self.variables.insert(s.clone(), dt.clone());
                }

                for statement in &func.body {
                    self.generate_statement(statement)?;
                }

                let ir_function = IRFunction {
                    name: self.function_name.clone(),
                    parameters: self.parameters.clone(),
                    return_type: self.return_type.clone(),
                    body: self.instructions.clone(),
                };

                self.functions.push(ir_function);
                }
            }
        }

        Ok(IRProgram {
        functions: self.functions.clone()
        })
    }

    fn generate_statement(&mut self, statement: &Statement) -> Result<(), String> {
        match statement {
            Statement::Declaration { data_type, variable_name, expression } => {
                let ir_type = map_data_type(data_type);
                self.variables.insert(variable_name.clone(), ir_type.clone());
                let value = self.generate_expression(expression)?;

                //Store the value to the global variable
                let store_instruction = IRInstruction {
                    opcode: IROpcode::Store,
                    result_type: IRType::Void, // Store doesn't return a value
                    operands: vec![IRValue::GlobalVariable(variable_name.clone()), value],
                };
                self.instructions.push(store_instruction);
                Ok(())
            }
            Statement::Assignment { variable_name, expression } => {
                let value = self.generate_expression(expression)?;

                let store_instruction = IRInstruction {
                    opcode: IROpcode::Store,
                    result_type: IRType::Void,
                    operands: vec![IRValue::GlobalVariable(variable_name.clone()), value],
                };
                self.instructions.push(store_instruction);
                Ok(())
            }
            Statement::Increment { variable_name } => {
                // Implement increment by loading the variable, adding 1, and storing back
                let load_instruction = IRInstruction {
                    opcode: IROpcode::Load,
                    result_type: self.variables.get(variable_name).ok_or(format!("Variable not found: {}", variable_name))?.clone(),
                    operands: vec![IRValue::GlobalVariable(variable_name.clone())],
                };
                self.instructions.push(load_instruction.clone());
                let value_index = self.instructions.len() - 1;

                let add_instruction = IRInstruction {
                    opcode: IROpcode::Add,
                    result_type: self.variables.get(variable_name).ok_or(format!("Variable not found: {}", variable_name))?.clone(),
                    operands: vec![IRValue::Instruction(value_index), IRValue::Constant(IRConstant::I32(1))],
                };
                self.instructions.push(add_instruction);

                let store_instruction = IRInstruction {
                    opcode: IROpcode::Store,
                    result_type: IRType::Void,
                    operands: vec![IRValue::GlobalVariable(variable_name.clone()), IRValue::Instruction(self.instructions.len() - 1)],
                };
                self.instructions.push(store_instruction);
                Ok(())
            }
            Statement::Decrement { variable_name } => {
                 // Implement decrement by loading the variable, subtracting 1, and storing back
                let load_instruction = IRInstruction {
                    opcode: IROpcode::Load,
                    result_type: self.variables.get(variable_name).ok_or(format!("Variable not found: {}", variable_name))?.clone(),
                    operands: vec![IRValue::GlobalVariable(variable_name.clone())],
                };
                self.instructions.push(load_instruction.clone());
                let value_index = self.instructions.len() - 1;

                let sub_instruction = IRInstruction {
                    opcode: IROpcode::Sub,
                    result_type: self.variables.get(variable_name).ok_or(format!("Variable not found: {}", variable_name))?.clone(),
                    operands: vec![IRValue::Instruction(value_index), IRValue::Constant(IRConstant::I32(1))],
                };
                self.instructions.push(sub_instruction);

                let store_instruction = IRInstruction {
                    opcode: IROpcode::Store,
                    result_type: IRType::Void,
                    operands: vec![IRValue::GlobalVariable(variable_name.clone()), IRValue::Instruction(self.instructions.len() - 1)],
                };
                self.instructions.push(store_instruction);
                Ok(())
            }
            Statement::If { condition, then_branch, elif_branches, else_branch } => {
               let condition_value = self.generate_expression(condition)?;

                let then_label = self.next_label();
                let else_label = self.next_label();
                let end_if_label = self.next_label();

                let jump_if_false = IRInstruction {
                    opcode: IROpcode::JumpIfFalse,
                    result_type: IRType::Void,
                    operands: vec![condition_value, IRValue::GlobalVariable(else_label.clone())], //Placeholder
                };
                self.instructions.push(jump_if_false);

                //Generate then branch
                for statement in then_branch {
                    self.generate_statement(statement)?;
                }
                self.instructions.push(IRInstruction {
                    opcode: IROpcode::Jump,
                    result_type: IRType::Void,
                    operands: vec![IRValue::GlobalVariable(end_if_label.clone())], //Placeholder
                });

                //Else label
                //Elif Branches
                //Else branch
                //End if label
                Ok(())
            }
            Statement::For { initialization, condition, increment, body } => {
                Ok(())
            }
            Statement::While { condition, body } => {
                Ok(())
            }
            Statement::FunctionCall { name, arguments } => {
                Ok(())
            }
            Statement::Return { expression } => {
                let value = self.generate_expression(expression)?;
                let return_instruction = IRInstruction {
                    opcode: IROpcode::Return,
                    result_type: IRType::Void,
                    operands: vec![value],
                };
                self.instructions.push(return_instruction);
                Ok(())
            }
        }
    }

    fn generate_expression(&mut self, expression: &Expression) -> Result<IRValue, String> {
        match expression {
            Expression::LiteralU8(value) => Ok(IRValue::Constant(IRConstant::U8(*value))),
            Expression::LiteralI32(value) => Ok(IRValue::Constant(IRConstant::I32(*value))),
            Expression::Variable(name) => Ok(IRValue::GlobalVariable(name.clone())),
            Expression::BinaryOp { op, left, right } => {
                // Generate IR for the left and right expressions
                let left_value = self.generate_expression(left)?;
                let right_value = self.generate_expression(right)?;

                // Determine the opcode based on the operator
                let opcode = match op.as_str() {
                    "+" => IROpcode::Add,
                    "-" => IROpcode::Sub,
                    "*" => IROpcode::Mul,
                    "/" => IROpcode::Div,
                    _ => return Err(format!("Unsupported binary operator: {}", op)),
                };

                // Create a new IR instruction
                let instruction = IRInstruction {
                    opcode,
                    result_type: IRType::I32, //FIXME: infer type
                    operands: vec![left_value, right_value],
                };

                // Add the instruction to the current function body
                self.instructions.push(instruction);

                // Return the IR value representing the result of the instruction
                Ok(IRValue::Instruction(self.instructions.len() - 1))
            }
            // Implement other expression types (UnaryOp, FunctionCall, etc.)
            _ => Err("Unimplemented expression type".to_string()),
        }
    }
}