use crate::{command::Command, context::GremlinContext};
mod connect;
mod disconnect;
mod display;
mod fallback;
mod quit;

pub use connect::ConnectAction;
pub use disconnect::DisconnectAction;
pub use display::{display_results, DisplayAction};
pub use fallback::FallbackAction;
pub use quit::QuitAction;
pub trait Action {
    fn name(&self) -> &str;

    fn handle(&mut self, ctx: &GremlinContext, cmd: String, args: Vec<String>) -> Vec<Command>;
}
