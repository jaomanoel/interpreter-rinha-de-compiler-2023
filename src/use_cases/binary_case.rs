use crate::{
    enums::{binary_op::BinaryOp, return_types::ReturnTypes},
    eval,
    structs::{binary::Binary, syntax_error::new_syntax_error},
};

pub fn binary_case(binary: Binary) -> ReturnTypes {
    let eval_lhs = eval(*binary.lhs);
    let eval_rhs = eval(*binary.rhs);

    match (eval_lhs, eval_rhs) {
        (ReturnTypes::Int(a), ReturnTypes::Int(b)) => match binary.op {
            BinaryOp::Add => ReturnTypes::Int(a + b),
            BinaryOp::Sub => ReturnTypes::Int(a - b),
            BinaryOp::Mul => ReturnTypes::Int(a * b),
            BinaryOp::Div => ReturnTypes::Int(a / b),
            BinaryOp::Rem => ReturnTypes::Int(a % b),
            BinaryOp::Eq => {
                if a == b {
                    return ReturnTypes::Bool(true);
                };
                ReturnTypes::Bool(false)
            }
            BinaryOp::Neq => {
                if a != b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            BinaryOp::Lt => {
                if a < b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            BinaryOp::Gt => {
                if a > b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            BinaryOp::Lte => {
                if a <= b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            BinaryOp::Gte => {
                if a >= b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            _ => new_syntax_error(binary.location),
        },
        (ReturnTypes::Int(a), ReturnTypes::Str(b)) => match binary.op {
            BinaryOp::Add => ReturnTypes::Str(a.to_string() + &b),
            _ => new_syntax_error(binary.location),
        },
        (ReturnTypes::Str(a), ReturnTypes::Str(b)) => match binary.op {
            BinaryOp::Add => ReturnTypes::Str(a + &b),
            BinaryOp::Eq => {
                if a == b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            BinaryOp::Neq => {
                if a != b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            _ => new_syntax_error(binary.location),
        },
        (ReturnTypes::Str(a), ReturnTypes::Int(b)) => match binary.op {
            BinaryOp::Add => ReturnTypes::Str(a + &b.to_string()),
            _ => panic!("Syntax Error"),
        },
        (ReturnTypes::Bool(a), ReturnTypes::Bool(b)) => match binary.op {
            BinaryOp::Eq => {
                if a == b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            BinaryOp::Neq => {
                if a != b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            BinaryOp::And => {
                if a && b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            BinaryOp::Or => {
                if a || b {
                    return ReturnTypes::Bool(true);
                }
                ReturnTypes::Bool(false)
            }
            _ => new_syntax_error(binary.location),
        },
        _ => new_syntax_error(binary.location),
    }
}
