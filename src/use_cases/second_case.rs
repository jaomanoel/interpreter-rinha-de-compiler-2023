use crate::{
    enums::return_types::ReturnTypes,
    eval,
    structs::{second::Second, syntax_error::new_syntax_error},
    Scope,
};

pub fn second_case(second: Second, scope: &Scope) -> ReturnTypes {
    match eval(*second.value, scope) {
        ReturnTypes::Tuple(tuple) => {
            let type_second = *tuple.1;

            match type_second {
                ReturnTypes::Str(s) => ReturnTypes::Str(s),
                ReturnTypes::Int(i) => ReturnTypes::Int(i),
                ReturnTypes::Bool(b) => ReturnTypes::Bool(b),
                _ => new_syntax_error(second.location),
            }
        }
        _ => new_syntax_error(second.location),
    }
}
