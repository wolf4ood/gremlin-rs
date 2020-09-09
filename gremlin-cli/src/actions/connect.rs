use crate::{actions::Action, command::Command, context::GremlinContext};
use futures::FutureExt;
use gremlin_client::aio::GremlinClient;

use structopt::StructOpt;

use structopt::clap::AppSettings;

pub struct ConnectAction;

#[derive(Debug, StructOpt)]
#[structopt(name = "connect", no_version, global_settings = &[AppSettings::DisableVersion, AppSettings::NoBinaryName, AppSettings::ColoredHelp])]
struct Connect {
    #[structopt(short, long, default_value = "localhost")]
    host: String,
    #[structopt(short, long, default_value = "8182")]
    port: u16,
}

impl ConnectAction {
    pub fn new() -> ConnectAction {
        ConnectAction
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
        match Connect::from_iter_safe(args[0..args.len()].to_vec()) {
            Ok(connect) => {
                let task = |_ctx: &GremlinContext| {
                    let future = async move {
                        match GremlinClient::connect((connect.host.as_str(), connect.port)).await {
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
            Err(e) => vec![Command::Print(Some(format!("{}", e)))],
        }
    }
}
