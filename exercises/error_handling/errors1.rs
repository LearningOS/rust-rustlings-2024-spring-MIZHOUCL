// errors1.rs
//
// This function refuses to generate text to be printed on a nametag if you pass
// it an empty string. It'd be nicer if it explained what the problem was,
// instead of just sometimes returning `None`. Thankfully, Rust has a similar
// construct to `Result` that can be used to express error conditions. Let's use
// it!
//
// Execute `rustlings hint errors1` or use the `hint` watch subcommand for a
// hint.
use std::error::Error;
#[derive(Debug)]
enum NametagGeneratorError {
    EmptyName,
}
impl Error for NametagGeneratorError {}

impl std::fmt::Display for NametagGeneratorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NametagGeneratorError::EmptyName => write!(f, "`name` was empty; it must be nonempty."),
        }
    }
}

pub fn generate_nametag_text(name: String) -> Result<String,NametagGeneratorError> {
    if name.is_empty() {
        // Empty names aren't allowed.
        Err(NametagGeneratorError::EmptyName)
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
            generate_nametag_text("Beyoncé".into()).unwrap(),
            "Hi! My name is Beyoncé"
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        let result = generate_nametag_text("".into());
        assert!(result.is_err());

        let error = result.unwrap_err();
        assert_eq!(format!("{}", error), "`name` was empty; it must be nonempty.");
    }
}
