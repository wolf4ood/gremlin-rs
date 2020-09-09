use crate::print;
use crate::{actions::Action, command::Command, context::GremlinContext};
use clap::{App, AppSettings};
use gremlin_client::GValue;
use std::fmt::Write;
pub struct DisplayAction(App<'static, 'static>);

impl DisplayAction {
    pub fn new() -> DisplayAction {
        DisplayAction(
            App::new("display")
                .setting(AppSettings::NoBinaryName)
                .setting(AppSettings::DisableVersion)
                .setting(AppSettings::ColoredHelp),
        )
    }
}

impl Action for DisplayAction {
    fn name(&self) -> &str {
        "display"
    }

    fn handle(&mut self, ctx: &GremlinContext, _: String, _: Vec<String>) -> Vec<Command> {
        vec![display_results(&ctx.last_results)]
    }
}

pub fn display_results(results: &Vec<GValue>) -> Command {
    let mut buffer = String::new();

    for result in results {
        writeln!(buffer, "==> {}", print::fmt(result)).expect("Failed to write");
    }
    Command::Print(Some(buffer))
}
