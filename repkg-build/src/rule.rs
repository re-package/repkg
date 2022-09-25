use crate::{Command, Name};

#[derive(Debug, Clone)]
pub struct Rule {
    pub name: Name,
    pub cmds: Vec<Command>,
}
