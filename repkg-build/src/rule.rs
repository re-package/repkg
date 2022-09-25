use crate::{Command, Name};

#[derive(Debug)]
pub struct Rule {
    pub name: Name,
    pub cmds: Vec<Command>,
}
