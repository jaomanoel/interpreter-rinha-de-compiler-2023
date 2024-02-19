use crate::{
    enums::return_types::ReturnTypes,
    eval,
    structs::{print::Print, syntax_error::new_syntax_error},
};

pub fn print_case(print: Print) -> ReturnTypes {
    let eval = eval(*print.value);

    match eval {
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
        _ => new_syntax_error(print.location),
    }
}
