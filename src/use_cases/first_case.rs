use crate::{
    enums::return_types::ReturnTypes,
    eval,
    structs::{first::First, syntax_error::new_syntax_error},
};

pub fn first_case(first: First) -> ReturnTypes {
    let value = eval(*first.value);

    match value {
        ReturnTypes::Tuple(tuple) => {
            let type_first = *tuple.0;

            match type_first {
                ReturnTypes::Str(s) => ReturnTypes::Str(s),
                ReturnTypes::Int(i) => ReturnTypes::Int(i),
                ReturnTypes::Bool(b) => ReturnTypes::Bool(b),
                _ => new_syntax_error(first.location),
            }
        }
        _ => new_syntax_error(first.location),
    }
}
