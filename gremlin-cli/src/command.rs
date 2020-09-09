use crate::context::GremlinContext;
use futures::future::BoxFuture;
use futures::FutureExt;

pub enum Command {
    Quit(Option<String>),
    Print(Option<String>),
    Exec(Box<dyn FnOnce(&GremlinContext) -> BoxFuture<'static, Vec<Command>> + Send>),
    Update(Box<dyn FnOnce(GremlinContext) -> GremlinContext + Send>),
}

impl Command {
    pub async fn exec(self, ctx: GremlinContext) -> GremlinContext {
        match self {
            Command::Quit(msg) => {
                if let Some(message) = msg {
                    println!("{}", message);
                }
                std::process::exit(0);
            }
            Command::Print(msg) => {
                if let Some(message) = msg {
                    print!("{}", message);
                }
                ctx
            }
            Command::Exec(cb) => {
                let future = cb(&ctx);
                let commands = future.await;
                execute_commands(ctx, commands).await
            }
            Command::Update(update) => update(ctx),
        }
    }
}

fn execute_commands(
    mut ctx: GremlinContext,
    commands: Vec<Command>,
) -> BoxFuture<'static, GremlinContext> {
    let future = async move {
        for command in commands {
            ctx = command.exec(ctx).await;
        }
        ctx
    };
    future.boxed()
}
