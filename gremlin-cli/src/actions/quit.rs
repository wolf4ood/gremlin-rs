use crate::{actions::Action, command::Command};

pub struct QuitAction;

impl Action for QuitAction {
    fn name(&self) -> &str {
        "quit"
    }

    fn description(&self) -> &str {
        "Exit the gremlin-cli."
    }

    fn handle(
        &self,
        _: &crate::context::GremlinContext,
        _: String,
        _: Vec<String>,
    ) -> Vec<Command> {
        vec![
            Command::Print(Some(String::from("Bye!"))),
            Command::Quit(None),
        ]
    }
}
