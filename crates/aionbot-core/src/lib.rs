pub mod entry;
pub mod event;
pub mod handler;
pub mod plugin;
pub mod prelude;
pub mod queue;
pub mod router {
    use crate::event::Event;

    pub trait Router: Send + Sync {
        fn matches(&self, event: &dyn Event) -> bool;
    }

    impl<T> Router for T
    where
        T: Send + Sync + AsRef<str> + 'static,
    {
        fn matches(&self, event: &dyn Event) -> bool {
            if let Ok(val) = event.content().downcast::<&str>() {
                *val == self.as_ref()
            } else {
                false
            }
        }
    }

    mod command;
    mod error;
    mod logic;
    mod matcher;

    pub use command::CommandRouter;
    pub use error::ErrorRouter;
    pub use logic::{AllRouter, AnyRouter};
    pub use matcher::{ContainsRouter, EndsWithRouter, ExactMatchRouter, StartsWithRouter};
}
pub mod runtime;
pub mod types;
