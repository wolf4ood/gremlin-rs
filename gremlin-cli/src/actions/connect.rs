use crate::{actions::Action, command::Command, context::GremlinContext};
use clap::{App, AppSettings};
use futures::FutureExt;
use gremlin_client::aio::GremlinClient;

pub struct ConnectAction(App<'static, 'static>);

impl ConnectAction {
    pub fn new() -> ConnectAction {
        ConnectAction(
            App::new("connect")
                .setting(AppSettings::NoBinaryName)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp),
        )
    }
}

impl Action for ConnectAction {
    fn name(&self) -> &str {
        "connect"
    }

    fn handle(
        &mut self,
        _: &crate::context::GremlinContext,
        _: String,
        args: Vec<String>,
    ) -> Vec<Command> {
        self.0
            .get_matches_from_safe_borrow(args[1..args.len()].to_vec())
            .unwrap();

        let task = |_ctx: &GremlinContext| {
            let future = async move {
                match GremlinClient::connect("localhost").await {
                    Ok(client) => vec![
                        Command::Update(Box::new(move |ctx| GremlinContext {
                            client: Some(client.clone()),
                            ..ctx
                        })),
                        Command::Print(Some("Connected!".into())),
                    ],
                    Err(err) => vec![Command::Print(Some(err.to_string()))],
                }
            };
            future.boxed()
        };

        vec![Command::Exec(Box::new(task))]
    }
}
