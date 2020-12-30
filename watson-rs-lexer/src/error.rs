use std::string::String;

use watson_rs_core::state::State;
pub struct Error {
    state: State,
    message: String,
}

impl Error {
    pub fn new() -> Self {
        Error {
            state: State::new(),
            message: String::new(),
        }
    }

    pub fn with_info(state: State, message: String) -> Self {
        Error {
            state,
            message,
        }
    }

    pub fn line(&self) -> usize {
        self.state.line()
    }

    pub fn column(&self) -> usize {
        self.state.column()
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    pub fn display_message(&self) -> String {
        format!("Ln {}, Col {}: {}", self.line(), self.column(), self.message)
    }
}
