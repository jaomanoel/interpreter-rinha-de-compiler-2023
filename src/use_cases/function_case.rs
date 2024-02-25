use crate::{
    enums::return_types::ReturnTypes,
    structs::{closure::Closure, function::Function},
    Scope,
};

pub fn function_case(function: Function, scope: &Scope) -> ReturnTypes {
    let closure: Closure = Closure {
        body: *function.value,
        parameters: function.parameters,
        env: scope.clone(),
    };
    ReturnTypes::Closure(closure)
}
