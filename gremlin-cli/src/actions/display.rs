use crate::print;
use crate::{actions::Action, command::Command, context::GremlinContext};
use gremlin_client::GValue;
use prettytable::{
    cell,
    format::{FormatBuilder, LinePosition, LineSeparator},
    row, Table,
};
pub struct DisplayAction;

impl DisplayAction {
    pub fn new() -> DisplayAction {
        DisplayAction
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
    let mut table = Table::new();

    let format = FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separator(LinePosition::Bottom, LineSeparator::new('-', '+', '+', '+'))
        .separator(LinePosition::Top, LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 1)
        .build();
    table.set_format(format);

    let mut idx = 1;
    for result in results {
        table.add_row(row![idx, print::fmt(result).as_str()]);
        idx += 1;
    }

    Command::PrintTable(table)
}
