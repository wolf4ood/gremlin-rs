use crate::{actions::Action, command::Command, context::GremlinContext};
use clap::{App, AppSettings};

pub struct DisconnectAction(App<'static, 'static>);

impl DisconnectAction {
    pub fn new() -> DisconnectAction {
        DisconnectAction(
            App::new("disconnect")
                .setting(AppSettings::NoBinaryName)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp),
        )
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
        vec![Command::Update(Box::new(|ctx| GremlinContext {
            client: None,
            ..ctx
        }))]
    }
}
