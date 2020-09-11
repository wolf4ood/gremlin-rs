use context::GremlinContext;

use smol;
use structopt::StructOpt;
mod actions;
mod command;
mod context;
pub(crate) mod print;
mod reader;
mod router;
use std::path::PathBuf;

use reader::Reader;

const WELCOME: &str = r#"
  ________                      .__  .__
 /  _____/______   ____   _____ |  | |__| ____           _______  ______
/   \  __\_  __ \_/ __ \ /     \|  | |  |/    \   ______ \_  __ \/  ___/
\    \_\  \  | \/\  ___/|  Y Y  \  |_|  |   |  \ /_____/  |  | \/\___ \
 \______  /__|    \___  >__|_|  /____/__|___|  /          |__|  /____  >
        \/            \/      \/             \/                      \/

"#;
fn main() {
    let opt = GremlinOpt::from_args();

    smol::block_on(async {
        let mut reader = Reader::new(opt);
        let mut ctx = GremlinContext::builder().build();
        let router = router::init();

        println!("{}", WELCOME);
        let mut should_quit = false;

        loop {
            if should_quit {
                break;
            }
            match reader.next(&ctx.prompt) {
                Some((line, args)) => {
                    if !args.is_empty() {
                        let commands = router.action(&args[0]).handle(
                            &ctx,
                            line.clone(),
                            args[1..args.len()].to_vec(),
                        );
                        for command in commands {
                            let cmd_result = command.exec(ctx).await;
                            ctx = cmd_result.0;
                            should_quit = cmd_result.1;
                        }
                        reader.update_history(&line);
                    }
                }
                None => {
                    break;
                }
            }
        }
        reader.save_history();
    })
}

#[derive(Debug, StructOpt)]
#[structopt(name = "gremlin-cli", about = "A Rusty gremlin cli")]
pub struct GremlinOpt {
    /// Command history file
    #[structopt(long, parse(from_os_str))]
    history: Option<PathBuf>,
}
