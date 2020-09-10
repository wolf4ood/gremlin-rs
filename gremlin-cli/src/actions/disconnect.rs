use crate::{actions::Action, command::Command, context::GremlinContext};

pub struct DisconnectAction;

impl DisconnectAction {
    pub fn new() -> DisconnectAction {
        DisconnectAction
    }
}

impl Action for DisconnectAction {
    fn name(&self) -> &str {
        "disconnect"
    }

    fn handle(
        &mut self,
        _: &crate::context::GremlinContext,
        _: String,
        _: Vec<String>,
    ) -> Vec<Command> {
        vec![
            Command::Update(Box::new(|ctx| GremlinContext {
                client: None,
                ..ctx
            })),
            Command::Print(Some("Disconnected!".into())),
        ]
    }
}
