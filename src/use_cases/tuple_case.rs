use crate::{
    enums::return_types::ReturnTypes,
    eval,
    structs::{syntax_error::new_syntax_error, tuple::Tuple},
};

pub fn tuple_case(tuple: Tuple) -> ReturnTypes {
    let first = eval(*tuple.first);
    let second = eval(*tuple.second);

    match (&first, &second) {
        (ReturnTypes::Bool(_), ReturnTypes::Bool(_)) => {
            ReturnTypes::Tuple((Box::new(first), Box::new(second)))
        }
        (ReturnTypes::Bool(_), ReturnTypes::Int(_)) => {
            ReturnTypes::Tuple((Box::new(first), Box::new(second)))
        }
        (ReturnTypes::Bool(_), ReturnTypes::Str(_)) => {
            ReturnTypes::Tuple((Box::new(first), Box::new(second)))
        }
        (ReturnTypes::Str(_), ReturnTypes::Str(_)) => {
            ReturnTypes::Tuple((Box::new(first), Box::new(second)))
        }
        (ReturnTypes::Str(_), ReturnTypes::Int(_)) => {
            ReturnTypes::Tuple((Box::new(first), Box::new(second)))
        }
        (ReturnTypes::Str(_), ReturnTypes::Bool(_)) => {
            ReturnTypes::Tuple((Box::new(first), Box::new(second)))
        }
        (ReturnTypes::Int(_), ReturnTypes::Int(_)) => {
            ReturnTypes::Tuple((Box::new(first), Box::new(second)))
        }
        (ReturnTypes::Int(_), ReturnTypes::Bool(_)) => {
            ReturnTypes::Tuple((Box::new(first), Box::new(second)))
        }
        (ReturnTypes::Int(_), ReturnTypes::Str(_)) => {
            ReturnTypes::Tuple((Box::new(first), Box::new(second)))
        }
        _ => new_syntax_error(tuple.location),
    }
}
