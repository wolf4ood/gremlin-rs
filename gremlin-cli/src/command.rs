use crate::context::GremlinContext;
use futures::future::BoxFuture;
use futures::FutureExt;
use prettytable::Table;
pub enum Command {
    Quit(Option<String>),
    Print(Option<String>),
    Exec(Box<dyn FnOnce(&GremlinContext) -> BoxFuture<'static, Vec<Command>> + Send>),
    Update(Box<dyn FnOnce(GremlinContext) -> GremlinContext + Send>),
    PrintTable(Table),
}

impl Command {
    pub async fn exec(self, ctx: GremlinContext) -> (GremlinContext, bool) {
        match self {
            Command::Quit(msg) => {
                if let Some(message) = msg {
                    println!("{}", message);
                }
                (ctx, true)
            }
            Command::Print(msg) => {
                if let Some(message) = msg {
                    print!("{}", message);
                }
                (ctx, false)
            }
            Command::Exec(cb) => {
                let future = cb(&ctx);
                let commands = future.await;
                execute_commands(ctx, commands).await
            }
            Command::Update(update) => (update(ctx), false),
            Command::PrintTable(table) => {
                table.printstd();
                (ctx, false)
            }
        }
    }
}

fn execute_commands(
    mut ctx: GremlinContext,
    commands: Vec<Command>,
) -> BoxFuture<'static, (GremlinContext, bool)> {
    let future = async move {
        for command in commands {
            let cmd_result = command.exec(ctx).await;
            ctx = cmd_result.0;
        }
        (ctx, false)
    };
    future.boxed()
}
