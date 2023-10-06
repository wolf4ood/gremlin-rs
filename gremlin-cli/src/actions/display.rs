use crate::print;
use crate::{actions::Action, command::Command, context::GremlinContext};
use gremlin_client::GValue;
use prettytable::{
    format::{FormatBuilder, LinePosition, LineSeparator},
    row, Row, Table,
};
pub struct DisplayAction;
use anyhow::Result;

impl DisplayAction {
    pub fn new() -> DisplayAction {
        DisplayAction
    }
}

impl Action for DisplayAction {
    fn name(&self) -> &str {
        "display"
    }

    fn description(&self) -> &str {
        "Display the last result."
    }

    fn handle(&self, ctx: &GremlinContext, _: String, _: Vec<String>) -> Vec<Command> {
        vec![display_results(&ctx.last_results)]
    }
}

pub fn display_results(results: &[GValue]) -> Command {
    let mut table = Table::new();

    let format = FormatBuilder::new()
        .column_separator('|')
        .borders('|')
        .separator(LinePosition::Bottom, LineSeparator::new('-', '+', '+', '+'))
        .separator(LinePosition::Top, LineSeparator::new('-', '+', '+', '+'))
        .padding(1, 1)
        .build();
    table.set_format(format);

    let collected: Result<Vec<Row>> = results
        .iter()
        .enumerate()
        .map(|(idx, item)| Ok(row![idx, print::fmt(item)?.as_str()]))
        .collect();

    match collected {
        Ok(rows) => {
            rows.into_iter().for_each(|item| {
                table.add_row(item);
            });
            Command::PrintTable(table)
        }
        Err(e) => Command::Print(Some(e.to_string())),
    }
}
