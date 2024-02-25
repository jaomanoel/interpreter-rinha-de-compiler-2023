use crate::{
    enums::{binary_op::BinaryOp, return_types::ReturnTypes},
    eval,
    structs::{binary::Binary, syntax_error::new_syntax_error},
    Scope,
};

pub fn binary_case(binary: Binary, scope: &Scope) -> ReturnTypes {
    match (eval(*binary.lhs, scope), eval(*binary.rhs, scope)) {
        (ReturnTypes::Int(a), ReturnTypes::Int(b)) => match binary.op {
            BinaryOp::Add => ReturnTypes::Int(a + b),
            BinaryOp::Sub => ReturnTypes::Int(a - b),
            BinaryOp::Mul => ReturnTypes::Int(a * b),
            BinaryOp::Div => ReturnTypes::Int(a / b),
            BinaryOp::Rem => ReturnTypes::Int(a % b),
            BinaryOp::Eq => ReturnTypes::Bool(a == b),
            BinaryOp::Neq => ReturnTypes::Bool(a != b),
            BinaryOp::Lt => ReturnTypes::Bool(a < b),
            BinaryOp::Gt => ReturnTypes::Bool(a > b),
            BinaryOp::Lte => ReturnTypes::Bool(a <= b),
            BinaryOp::Gte => ReturnTypes::Bool(a >= b),
            _ => new_syntax_error(binary.location),
        },
        (ReturnTypes::Int(a), ReturnTypes::Str(b)) => match binary.op {
            BinaryOp::Add => ReturnTypes::Str(a.to_string() + &b),
            _ => new_syntax_error(binary.location),
        },
        (ReturnTypes::Str(a), ReturnTypes::Str(b)) => match binary.op {
            BinaryOp::Add => ReturnTypes::Str(a + &b),
            BinaryOp::Eq => ReturnTypes::Bool(a == b),
            BinaryOp::Neq => ReturnTypes::Bool(a != b),
            _ => new_syntax_error(binary.location),
        },
        (ReturnTypes::Str(a), ReturnTypes::Int(b)) => match binary.op {
            BinaryOp::Add => ReturnTypes::Str(a + &b.to_string()),
            _ => new_syntax_error(binary.location),
        },
        (ReturnTypes::Bool(a), ReturnTypes::Bool(b)) => match binary.op {
            BinaryOp::Eq => ReturnTypes::Bool(a == b),
            BinaryOp::Neq => ReturnTypes::Bool(a != b),
            BinaryOp::And => ReturnTypes::Bool(a && b),
            BinaryOp::Or => ReturnTypes::Bool(a || b),
            _ => new_syntax_error(binary.location),
        },
        _ => new_syntax_error(binary.location),
    }
}
