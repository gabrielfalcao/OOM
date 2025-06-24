pub mod caller;
pub use caller::Caller;

pub trait Traceback: std::error::Error {
    fn message(&self) -> String;
    fn with(&self, caller: Caller) -> Self;
    fn callers(&self) -> Vec<Caller>;

    fn callers_to_string(&self, indent: usize) -> String {
        let indentation = " ".repeat(indent).to_string();
        self.callers()
            .iter()
            .enumerate()
            .map(|(index, caller)| format!("{}{}", indentation.repeat(index), caller))
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn highlight_message(&self) -> String {
        format!("{}", self.message())
    }
    fn previous_as_debug(&self) -> String {
        String::new()
    }
    fn previous_as_string(&self) -> String {
        String::new()
    }
}
