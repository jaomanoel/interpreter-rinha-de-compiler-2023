use crate::{
    enums::return_types::ReturnTypes,
    eval,
    structs::{print::Print, syntax_error::new_syntax_error},
    Scope,
};

pub fn print_case(print: Print, scope: &Scope) -> ReturnTypes {
    match eval(*print.value, scope) {
        ReturnTypes::Str(s) => {
            println!("{s}");
            ReturnTypes::Void
        }
        ReturnTypes::Int(i) => {
            println!("{i}");
            ReturnTypes::Void
        }
        ReturnTypes::Bool(b) => {
            println!("{b}");
            ReturnTypes::Void
        }
        ReturnTypes::Tuple(tuple) => {
            println!("{tuple:?}");
            ReturnTypes::Void
        }
        _ => new_syntax_error(print.location),
    }
}
