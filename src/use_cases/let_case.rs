use crate::enums::return_types::ReturnTypes;
use crate::structs::let_struct::Let;
use crate::structs::syntax_error::new_syntax_error;
use crate::{eval, Scope};

pub fn let_case(let_struct: Let, scope: &Scope) -> ReturnTypes {
    match eval(*let_struct.value, scope) {
        ReturnTypes::Int(int) => scope.set(let_struct.name.text, ReturnTypes::Int(int)),
        ReturnTypes::Bool(bool) => scope.set(let_struct.name.text, ReturnTypes::Bool(bool)),
        ReturnTypes::Str(str) => scope.set(let_struct.name.text, ReturnTypes::Str(str)),
        ReturnTypes::Tuple(tuple) => scope.set(let_struct.name.text, ReturnTypes::Tuple(tuple)),
        ReturnTypes::Closure(closure) => {
            scope.set(let_struct.name.text, ReturnTypes::Closure(closure))
        }
        _ => return new_syntax_error(let_struct.location),
    }

    eval(*let_struct.next, scope)
}
