use itertools::Itertools;

#[derive(Default)]
pub(crate) struct Handler {
    errors: Vec<Message>,
}

#[derive(Debug)]
pub enum Message {
    Str(String),
}

impl From<String> for Message {
    fn from(value: String) -> Self {
        Message::Str(value)
    }
}

impl From<&'_ str> for Message {
    fn from(value: &'_ str) -> Self {
        value.to_string().into()
    }
}

impl Handler {
    pub fn has_errors(&mut self) -> bool {
        !self.errors.is_empty()
    }

    pub fn emit_error<E: Into<Message>>(&mut self, err: E) {
        self.errors.push(err.into());
    }

    pub fn abort_if_errors(&mut self) {
        if !self.has_errors() {
            return;
        }

        eprintln!(
            "Errors: {}",
            self.errors
                .iter()
                .map(|s| match s {
                    Message::Str(s) => s,
                })
                .join("\n")
        );

        std::panic::resume_unwind(Box::new(()));
    }
}
