use crate::{
    enums::return_types::ReturnTypes,
    eval,
    structs::{call::Call, syntax_error::new_syntax_error},
    Scope,
};

pub fn call_case(call: Call, scope: &Scope) -> ReturnTypes {
    match eval(*call.callee, scope) {
        ReturnTypes::Closure(closure) => {
            if closure.parameters.len() != call.arguments.len() {
                return new_syntax_error(call.location);
            }

            for (param, arg) in closure.parameters.into_iter().zip(call.arguments) {
                closure.env.set(param.text, eval(arg, scope))
            }

            eval(closure.body, &closure.env)
        }
        _ => new_syntax_error(call.location),
    }
}
