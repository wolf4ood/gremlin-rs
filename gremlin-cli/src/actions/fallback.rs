use crate::actions::Action;
use crate::{command::Command, context::GremlinContext};
use async_std::prelude::*;
use futures::future::BoxFuture;
use futures::FutureExt;
use gremlin_client::{
    aio::{GResultSet, GremlinClient},
    GValue,
};

use crate::actions::display_results;
pub struct FallbackAction;

impl Action for FallbackAction {
    fn name(&self) -> &str {
        "fallback"
    }

    fn handle(
        &mut self,
        ctx: &crate::context::GremlinContext,
        cmd: String,
        _: Vec<String>,
    ) -> Vec<crate::command::Command> {
        if cmd.trim().starts_with(&ctx.alias) {
            match ctx.client {
                Some(ref client) => vec![Command::Exec(Box::new(execute_query(
                    client.clone(),
                    cmd.clone(),
                )))],
                None => vec![Command::Print(Some(String::from("Not connected!")))],
            }
        } else {
            vec![Command::Print(Some(String::from("Command unrecognized!")))]
        }
    }
}

fn execute_query(
    client: GremlinClient,
    query: String,
) -> impl FnOnce(&GremlinContext) -> BoxFuture<'static, Vec<Command>> {
    move |_| {
        async move {
            match client.execute(query, &[]).await {
                Ok(stream) => match map_result(stream).await {
                    Ok(results) => vec![
                        display_results(&results),
                        Command::Update(Box::new(|ctx| GremlinContext {
                            last_results: results,
                            ..ctx
                        })),
                    ],
                    Err(error) => vec![Command::Print(Some(error))],
                },
                Err(err) => vec![Command::Print(Some(format!("{}", err)))],
            }
        }
        .boxed()
    }
}

async fn map_result(stream: GResultSet) -> Result<Vec<GValue>, String> {
    stream
        .collect::<Result<Vec<GValue>, _>>()
        .await
        .map_err(|err| format!("{}", err))
}
