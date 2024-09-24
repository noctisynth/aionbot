use std::{hash::Hash, sync::Arc};

use crate::{router::Router, types::Callback};

#[derive(Clone)]
pub struct Entry {
    pub id: &'static str,
    pub priority: i8,
    pub router: Arc<Box<dyn Router>>,
    pub callback: Arc<Callback>,
}

impl Entry {
    pub fn get_priority(&self) -> i8 {
        self.priority
    }

    pub fn get_router(&self) -> &dyn Router {
        self.router.as_ref().as_ref()
    }

    pub fn get_handler(&self) -> Arc<Callback> {
        self.callback.clone()
    }
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Entry {}

impl Ord for Entry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.priority.cmp(&other.priority)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Entry {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}
