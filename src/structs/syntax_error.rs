use crate::enums::return_types::ReturnTypes;

use super::location::Location;

#[derive(Debug)]
struct SyntaxError {
    details: String,
}

pub fn new_syntax_error(location: Location) -> ReturnTypes {
    let error = SyntaxError {
        details: format!(
            "Syntax Error, between: {} and {} in file: {}",
            location.start, location.end, location.filename,
        ),
    };

    println!("{}", error.details);
    ReturnTypes::Void
}
