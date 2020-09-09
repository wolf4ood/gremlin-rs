use context::GremlinContext;

use smol;

mod actions;
mod command;
mod context;
pub(crate) mod print;
mod reader;
mod router;

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
    smol::block_on(async {
        let mut reader = Reader::new();
        let mut ctx = GremlinContext::builder().build();
        let mut router = router::init();

        println!("{}", WELCOME);

        loop {
            match reader.next(&ctx.prompt) {
                Some((line, args)) => {
                    if !args.is_empty() {
                        let commands = router.action(&args[0]).handle(
                            &ctx,
                            line.clone(),
                            args[1..args.len()].to_vec(),
                        );

                        for command in commands {
                            ctx = command.exec(ctx).await;
                        }
                        reader.update_history(&line);
                    }
                }
                None => {
                    break;
                }
            }
        }
    })
}
