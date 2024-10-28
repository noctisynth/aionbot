use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::event::Event;

use super::Router;

#[derive(Default)]
pub struct AllRouter;

impl Router for AllRouter {
    fn matches(&self, _event: &dyn Event) -> bool {
        true
    }
}

pub struct AnyRouter {
    pub routers: Vec<Box<dyn Router>>,
}

impl Router for AnyRouter {
    fn matches(&self, event: &dyn Event) -> bool {
        self.routers.par_iter().any(|r| r.matches(event))
    }
}

impl AnyRouter {
    pub fn new(routers: Vec<Box<dyn Router>>) -> Self {
        Self { routers }
    }
}
