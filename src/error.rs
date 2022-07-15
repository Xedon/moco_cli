use std::fmt;

use error_stack::Context;

#[derive(Debug)]
pub struct MocoCliError(pub String);

impl Default for MocoCliError {
    fn default() -> Self {
        Self(String::new())
    }
}

impl Context for MocoCliError {}

impl fmt::Display for MocoCliError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.0)
    }
}
