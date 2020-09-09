use crate::{actions::Action, command::Command};

pub struct QuitAction;

impl Action for QuitAction {
    fn name(&self) -> &str {
        "quit"
    }

    fn handle(
        &mut self,
        _: &crate::context::GremlinContext,
        _: String,
        _: Vec<String>,
    ) -> Vec<Command> {
        vec![Command::Quit(None)]
    }
}
