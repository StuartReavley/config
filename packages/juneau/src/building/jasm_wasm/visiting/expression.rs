use crate::{
    core::Id,
    semantic::{
        jasm::{
            Block, Jasm, JasmExpression, JasmExpressionVisitor, JasmStatement,
            JasmStatementVisitor, JasmType, JasmValue, Struct,
        },
        Function, FunctionType, Name, Parameter, Variable, BinaryOperator
    },
};
use crate::{
    core::{Visitor, VisitorWith},
    semantic::{jasm::JasmPrimitiveImplementation, Implementation},
};
use crate::building::jasm_wasm::visitor::*;
use std::{any::Any, rc::Rc, str::FromStr};
use walrus::ir::*;
use walrus::{FunctionBuilder, InstrSeqBuilder, LocalId, Module, ModuleConfig, ValType};

impl JasmExpressionVisitor<()> for WasmBuilderVisitor {
    fn visit_constant(&mut self, value: &JasmValue) -> () {
        todo!()
    }

    fn visit_invocation(
        &mut self,
        id: Id,
        name: &Name,
        arguments: &Vec<JasmExpression>,
        return_typ: &JasmType,
    ) -> () {
        let return_type = ValType::from(return_typ);
        for argument in arguments {
            self.visit(argument);
        }
        let operator = String::from(name);
        let ops = match FromStr::from_str(&operator) {
            Ok(BinaryOperator::Add) => {
                match return_type {
                    ValType::I64 => BinaryOp::I64Add,
                    // TODO: Complete the remaining ValType
                    _ => BinaryOp::I64Sub
                }
            },
            _ => BinaryOp::I64Sub
        };
        self.function_builder.func_body().binop(ops);
    }

    fn visit_variable(&mut self, variable: &Variable<Jasm>) -> () {
        let index = (variable.id.value - 1001) as usize;
        let locals: Vec<&Local> = self.module.locals.iter().collect();
        self.function_builder.func_body().local_get(locals[index].id());
    }

    fn visit_cast(
        &mut self,
        expression: &JasmExpression,
        typ: &JasmType,
    ) -> () {
        todo!()
    }

    fn visit_struct_access(
        &mut self,
        object: &JasmExpression,
        id: Id,
        name: &Name,
        typ: &JasmType,
    ) -> () {
        todo!()
    }

    fn visit_array_access(
        &mut self,
        _object: &JasmExpression,
        _index: &JasmExpression,
    ) -> () {
        todo!()
    }

    fn visit_reference(&mut self, _expression: &JasmExpression) -> () {
        todo!()
    }

    fn visit_dereference(&mut self, _expression: &JasmExpression) -> () {
        todo!()
    }
}