use crate::{
    enums::{return_types::ReturnTypes, term::Term},
    eval,
    structs::{if_struct::If, syntax_error::new_syntax_error},
    Scope,
};

pub fn if_case(if_struct: If, scope: &Scope) -> ReturnTypes {
    let condition = *if_struct.condition;

    match condition {
        Term::Bool(_) => if_case_execute(condition, *if_struct.then, *if_struct.otherwise, scope),
        Term::Binary(_) => if_case_execute(condition, *if_struct.then, *if_struct.otherwise, scope),
        Term::Print(print) => new_syntax_error(print.location),
        Term::Str(str) => new_syntax_error(str.location),
        Term::First(first) => new_syntax_error(first.location),
        Term::Second(second) => new_syntax_error(second.location),
        Term::If(if_s) => new_syntax_error(if_s.location),
        Term::Int(int) => new_syntax_error(int.location),
        Term::Tuple(tuple) => new_syntax_error(tuple.location),
        Term::Let(let_s) => new_syntax_error(let_s.location),
        Term::Function(function) => new_syntax_error(function.location),
        Term::Var(var) => new_syntax_error(var.location),
        Term::Call(call) => new_syntax_error(call.location),
    }
}

fn if_case_execute(condition: Term, then: Term, otherwise: Term, scope: &Scope) -> ReturnTypes {
    match eval(condition, scope) {
        ReturnTypes::Bool(value) => {
            if value {
                eval(then, scope)
            } else {
                eval(otherwise, scope)
            }
        }
        _ => ReturnTypes::Void,
    }
}
