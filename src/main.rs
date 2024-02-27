#![allow(dead_code)]

use crate::{enums::term::Term, use_cases::if_case::if_case};
use enums::return_types::ReturnTypes;
use memory::scope::Scope;
use serde::Deserialize;
use std::fs;
use structs::{location::Location, str, syntax_error::new_syntax_error};
use use_cases::{
    binary_case::binary_case, bool_case::bool_case, call_case::call_case, first_case::first_case,
    function_case::function_case, int_case::int_case, let_case::let_case, print_case::print_case,
    second_case::second_case, str_case::str_case, tuple_case::tuple_case,
};

mod enums;
mod memory;
mod structs;
mod use_cases;

#[derive(Deserialize, Debug)]
struct File {
    name: String,
    expression: Term,
    location: Location,
}

pub fn eval(term: Term, scope: &Scope) -> ReturnTypes {
    match term {
        Term::Int(int) => int_case(int.value),
        Term::Str(string) => str_case(string.value),
        Term::Print(print) => print_case(print, scope),
        Term::Bool(bool) => bool_case(bool),
        Term::Binary(binary) => binary_case(binary, scope),
        Term::Tuple(tuple) => tuple_case(tuple, scope),
        Term::First(first) => first_case(first, scope),
        Term::Second(second) => second_case(second, scope),
        Term::If(if_struct) => if_case(if_struct, scope),
        Term::Let(let_struct) => let_case(let_struct, scope),
        Term::Function(function) => function_case(function, scope),
        Term::Var(var) => match scope.get(&var.text) {
            Some(value) => value,
            None => new_syntax_error(var.location),
        },
        Term::Call(call) => call_case(call, scope),
    }
}

fn read_and_parse_json(file_path: &str) -> File {
    let data = fs::read_to_string(file_path).unwrap();
    serde_json::from_str::<File>(&data).unwrap()
}

fn main() {
    let global_scope = Scope::default();

    let file_add = read_and_parse_json("./examples/add.json");
    let file_div = read_and_parse_json("./examples/div.json");
    let file_mul = read_and_parse_json("./examples/mul.json");
    let file_neq = read_and_parse_json("./examples/neq.json");
    let file_rem = read_and_parse_json("./examples/rem.json");
    let file_sub = read_and_parse_json("./examples/sub.json");
    let file_eq = read_and_parse_json("./examples/eq.json");
    let file_lt = read_and_parse_json("./examples/lt.json");
    let file_gt = read_and_parse_json("./examples/gt.json");
    let file_lte = read_and_parse_json("./examples/lte.json");
    let file_gte = read_and_parse_json("./examples/gte.json");
    let file_and = read_and_parse_json("./examples/and.json");
    let file_or = read_and_parse_json("./examples/or.json");
    let file_tup = read_and_parse_json("./examples/tup.json");
    let file_fir = read_and_parse_json("./examples/first.json");
    let file_sec = read_and_parse_json("./examples/second.json");
    let file_if = read_and_parse_json("./examples/if.json");
    let file_let = read_and_parse_json("./examples/let.json");
    let file_fib = read_and_parse_json("./examples/fib.json");
    let file_clo = read_and_parse_json("./examples/closure.json");
    let file_com = read_and_parse_json("./examples/combination.json");

    assert_eq!(
        ReturnTypes::Int(4),
        eval(file_add.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Int(-1),
        eval(file_sub.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Int(4),
        eval(file_mul.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Int(1),
        eval(file_div.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Int(0),
        eval(file_rem.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Bool(true),
        eval(file_neq.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Bool(true),
        eval(file_eq.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Bool(true),
        eval(file_lt.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Bool(false),
        eval(file_gt.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Bool(true),
        eval(file_lte.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Bool(false),
        eval(file_gte.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Bool(false),
        eval(file_and.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Bool(true),
        eval(file_or.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Tuple((Box::new(ReturnTypes::Int(2)), Box::new(ReturnTypes::Int(3)))),
        eval(file_tup.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Int(1),
        eval(file_fir.expression, &global_scope)
    );
    assert_eq!(
        ReturnTypes::Int(2),
        eval(file_sec.expression, &global_scope)
    );
    assert_eq!(ReturnTypes::Void, eval(file_if.expression, &global_scope));

    eval(file_let.expression, &global_scope);
    assert_eq!(ReturnTypes::Void, eval(file_clo.expression, &global_scope));
    assert_eq!(ReturnTypes::Void, eval(file_com.expression, &global_scope));
    assert_eq!(ReturnTypes::Void, eval(file_fib.expression, &global_scope));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_test() {
        let global_scope = Scope::default();

        let file_hello = read_and_parse_json("./examples/hello.json");
        assert_eq!(
            ReturnTypes::Void,
            eval(file_hello.expression, &global_scope)
        )
    }
}
