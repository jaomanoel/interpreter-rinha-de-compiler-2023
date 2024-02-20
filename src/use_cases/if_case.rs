use crate::{
    enums::{return_types::ReturnTypes, term::Term},
    eval,
    structs::{if_struct::If, syntax_error::new_syntax_error},
};

pub fn if_case(if_struct: If) -> ReturnTypes {
    let condition = *if_struct.condition;

    match condition {
        Term::Bool(_) => if_case_execute(condition, *if_struct.then, *if_struct.otherwise),
        Term::Binary(_) => if_case_execute(condition, *if_struct.then, *if_struct.otherwise),
        Term::Print(print) => new_syntax_error(print.location),
        Term::Str(str) => new_syntax_error(str.location),
        Term::First(first) => new_syntax_error(first.location),
        Term::Second(second) => new_syntax_error(second.location),
        Term::If(if_s) => new_syntax_error(if_s.location),
        Term::Int(int) => new_syntax_error(int.location),
        Term::Tuple(tuple) => new_syntax_error(tuple.location),
    }
}

fn if_case_execute(condition: Term, then: Term, otherwise: Term) -> ReturnTypes {
    match eval(condition) {
        ReturnTypes::Bool(value) => {
            if value {
                eval(then)
            } else {
                eval(otherwise)
            }
        }
        _ => ReturnTypes::Void,
    }
}
