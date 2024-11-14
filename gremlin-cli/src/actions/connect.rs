use crate::{actions::Action, command::Command, context::GremlinContext};
use futures::FutureExt;
use gremlin_client::{aio::GremlinClient, ConnectionOptions, IoProtocol, TlsOptions};
use std::str::FromStr;
use structopt::StructOpt;

use anyhow::{anyhow, Error};
use structopt::clap::AppSettings;

pub struct ConnectAction;

#[derive(Debug)]
pub enum Serializer {
    GraphSONV2,
    GraphSONV3,
}

impl FromStr for Serializer {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "graphson_v2" => Ok(Serializer::GraphSONV2),
            "graphson_v3" => Ok(Serializer::GraphSONV3),
            _ => Err(anyhow!(
                "Serializer {} not valid, possible values [graphson_v1, graphson_v2, graphson_v3]",
                s
            )),
        }
    }
}

impl From<Serializer> for IoProtocol {
    fn from(serializer: Serializer) -> Self {
        match serializer {
            Serializer::GraphSONV2 => IoProtocol::GraphSONV2,
            Serializer::GraphSONV3 => IoProtocol::GraphSONV3,
        }
    }
}
#[derive(Debug, StructOpt)]
#[structopt(name = "connect", no_version, global_settings = &[AppSettings::DisableVersion, AppSettings::NoBinaryName, AppSettings::ColoredHelp])]
struct Connect {
    #[structopt(short, long, default_value = "localhost")]
    host: String,
    #[structopt(short, long, default_value = "8182")]
    port: u16,
    #[structopt(long)]
    ssl: bool,

    #[structopt(long)]
    insecure: bool,

    #[structopt(long)]
    user: Option<String>,

    #[structopt(long)]
    password: Option<String>,

    #[structopt(long, default_value = "graphson_v3")]
    serializer: Serializer,
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

    fn description(&self) -> &str {
        "Connect to the Gremlin Server."
    }

    fn handle(
        &self,
        _: &crate::context::GremlinContext,
        _: String,
        args: Vec<String>,
    ) -> Vec<Command> {
        match Connect::from_iter_safe(args[0..args.len()].to_vec()) {
            Ok(connect) => {
                let task = |_ctx: &GremlinContext| {
                    let future = async move {
                        let mut options_builder = ConnectionOptions::builder()
                            .host(connect.host.as_str())
                            .port(connect.port)
                            .ssl(connect.ssl)
                            .serializer(connect.serializer.into())
                            .tls_options(TlsOptions {
                                accept_invalid_certs: connect.insecure,
                            });

                        if let (Some(username), Some(password)) = (connect.user, connect.password) {
                            options_builder = options_builder.credentials(&username, &password);
                        }
                        match GremlinClient::connect(options_builder.build()).await {
                            Ok(client) => vec![
                                Command::Update(Box::new(move |ctx| GremlinContext {
                                    client: Some(client),
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
