use crate::actions::Action;
use std::collections::HashMap;

use crate::actions;
use actions::{ConnectAction, DisconnectAction, DisplayAction, QuitAction};
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
    pub fn action(&mut self, name: &str) -> &mut dyn Action {
        self.actions
            .get_mut(name)
            .map(|action| action.as_mut())
            .unwrap_or(&mut self.fallback)
    }

    pub fn add(mut self, action: Box<dyn Action>) -> Self {
        self.actions.insert(action.name().into(), action);
        self
    }
}

pub fn init() -> ActionRouter<actions::FallbackAction> {
    ActionRouter::new(actions::FallbackAction {})
        .add(Box::new(QuitAction))
        .add(Box::new(ConnectAction::new()))
        .add(Box::new(DisconnectAction::new()))
        .add(Box::new(DisplayAction::new()))
}

#[cfg(test)]
mod tests {

    // struct MockFallbackAction {}

    // use crate::{command::Command, context::GremlinContext};

    // impl super::Action for MockFallbackAction {
    //     fn name(&self) -> &str {
    //         "quit"
    //     }

    //     fn handle(
    //         &self,
    //         _ctx: &crate::context::GremlinContext,
    //         _action: &str,
    //     ) -> Vec<crate::command::Command> {
    //         vec![Command::Quit(None)]
    //     }
    // }

    // #[test]
    // fn fallback_test() {
    //     let router = super::ActionRouter::new(MockFallbackAction {});

    //     let action = router.action("fake");

    //     let ctx = GremlinContext::builder().build();
    //     assert_eq!("quit", action.name());

    //     matches
    //     assert_eq!(vec![Command::Quit(None)], action.handle(&ctx, "quit"));
    // }
}
