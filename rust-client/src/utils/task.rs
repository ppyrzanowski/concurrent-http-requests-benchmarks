use std::fmt::Display;

#[derive(Debug)]
pub struct TaskId(pub u32);

impl Display for TaskId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Task {:0pad$}]", self.0.to_string(), pad = 3)
    }
}
