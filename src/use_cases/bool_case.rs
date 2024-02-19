use crate::{enums::return_types::ReturnTypes, structs::bool::Bool};

pub fn bool_case(bool: Bool) -> ReturnTypes {
    ReturnTypes::Bool(bool.value)
}
