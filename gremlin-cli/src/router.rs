use crate::actions::Action;
use crate::command::Command;
use std::collections::HashMap;

use crate::actions;
use actions::{ConnectAction, DisconnectAction, DisplayAction, QuitAction};
use std::fmt::Write;
pub struct ActionRouter<T: Action> {
    fallback: T,
    actions: HashMap<String, Box<dyn Action>>,
}

impl<T: Action + 'static> ActionRouter<T> {
    pub fn new(fallback: T) -> ActionRouter<T> {
        ActionRouter {
            fallback,
            actions: HashMap::new(),
        }
    }
    pub fn action(&self, name: &str) -> &dyn Action {
        match (self.actions.get(name), name) {
            (Some(action), _) => action.as_ref(),
            (None, "help") => self,
            (None, _) => &self.fallback,
        }
    }

    pub fn add(mut self, action: Box<dyn Action>) -> Self {
        self.actions.insert(action.name().into(), action);
        self
    }
}

impl<T: Action> Action for ActionRouter<T> {
    fn name(&self) -> &str {
        todo!()
    }

    fn handle(
        &self,
        _: &crate::context::GremlinContext,
        _: String,
        _: Vec<String>,
    ) -> Vec<crate::command::Command> {
        let mut output = String::new();
        writeln!(output, "Available Commands: ").unwrap();
        writeln!(output).expect("Failed to write a line");
        let mut names: Vec<(&str, &str)> = self
            .actions
            .iter()
            .map(|(_, v)| (v.name(), v.description()))
            .collect();

        names.sort_by(|(a, _), (b, _)| a.cmp(b));

        output = names.iter().fold(output, |mut acc, action| {
            writeln!(acc, "{:5}{:20}{}", "", action.0, action.1).unwrap();
            acc
        });

        writeln!(output).expect("Failed to write a line");
        writeln!(output, "For help on a specific command type:").unwrap();
        writeln!(output, "{:5}command --help", "").unwrap();

        vec![Command::Print(Some(output))]
    }

    fn description(&self) -> &str {
        "Print this help."
    }
}

pub fn init() -> ActionRouter<actions::FallbackAction> {
    ActionRouter::new(actions::FallbackAction {})
        .add(Box::new(QuitAction))
        .add(Box::new(ConnectAction::new()))
        .add(Box::new(DisconnectAction::new()))
        .add(Box::new(DisplayAction::new()))
}
