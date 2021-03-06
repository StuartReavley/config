use std::rc::Rc;
use std::collections::HashMap;
use crate::core::{Id, IdContext};
use crate::semantic::{BinaryOperator, UnaryOperator, Function, Functions, Parameter, Implementation, Variable};
use crate::semantic::jasm::{Jasm, NumberType, JasmType, JasmPrimitiveImplementation};
use crate::parsing::Symbols;

mod new;
pub use new::*;


pub fn new_jasm_stdl(ids:&mut impl IdContext, other_functions:&Functions<Jasm>, symbols:&[Variable<Jasm>]) -> (Functions<Jasm>, Symbols<Jasm>) {
    let mut functions = Functions::new();
    use JasmType::*;
    let types = [U64, I64, F64];
    let binary_operators = {use BinaryOperator::*;[Add, Subtract, Multiply, Divide]};
    let unary_operators = {use UnaryOperator::*;[Negate]};
    for typ in &types {
        for operator in &binary_operators {
            functions.insert_function(new_binary(ids, *operator, typ, typ));
        }
        for operator in &unary_operators {
            functions.insert_function(new_unary(ids, *operator, typ, typ));
        }
    }

    let predicates = {use BinaryOperator::*;[Equal, NotEqual, GreaterThan, GreaterThanOrEqual, LessThan, LessThanOrEqual]};
    let predicate_types = [U8, U64, I64, F64, Bool];
    for typ in &predicate_types {
        for operator in &predicates {
            functions.insert_function(new_binary(ids, *operator, typ, &Bool));
        }
    }

    let integer_binary_operators = {use BinaryOperator::*;[ShiftLeft, ShiftRight, And, Or, Xor]};
    let integer_unary_operators = {use UnaryOperator::*;[Not]};
    let integer_types = [U8, U64, I64];

    for typ in &integer_types {
        for operator in &integer_binary_operators {
            functions.insert_function(new_binary(ids, *operator, typ, typ));
        }
        for operator in &integer_unary_operators {
            functions.insert_function(new_unary(ids, *operator, typ, typ));
        }
    }

    let bool_binary_operators = {use BinaryOperator::*;[And, Or, Xor]};
    let bool_unary_operators = {use UnaryOperator::*;[Not]};

    for operator in &bool_binary_operators {
        functions.insert_function(new_binary(ids, *operator, &Bool, &Bool));
    }
    for operator in &bool_unary_operators {
        functions.insert_function(new_unary(ids, *operator, &Bool, &Bool));
    }

    functions.union_self(other_functions);
    let symbols = functions
        .iter()
        .map(|f|Variable::new(f.id, &f.name, &f.get_type().into()))
        .chain(symbols.to_owned().into_iter())
        .collect();

    (functions, symbols)
}


fn new_binary(ids:&mut impl IdContext, operator:BinaryOperator, parameter:&JasmType, retrn:&JasmType) -> Rc<Function<Jasm>> {
    Function::new(
        ids.new_id(),
        operator.to_string().into(),
        vec![
            Parameter::new(ids.new_id(), &"a".into(), parameter),
            Parameter::new(ids.new_id(), &"b".into(), parameter)
        ],
        Implementation::Primitive(retrn.to_owned(), JasmPrimitiveImplementation::Binary(operator).into()))
}


fn new_unary(ids:&mut impl IdContext, operator:UnaryOperator, parameter:&JasmType, retrn:&JasmType) -> Rc<Function<Jasm>> {
    Function::new(
        ids.new_id(),
        operator.to_string().into(),
        vec![
            Parameter::new(ids.new_id(), &"a".into(), parameter),
        ],
        Implementation::Primitive(retrn.to_owned(), JasmPrimitiveImplementation::Unary(operator).into()))
}