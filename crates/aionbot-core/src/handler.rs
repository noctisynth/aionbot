use std::sync::Arc;

use anyhow::Result;

use crate::{entry::Entry, event::Event, queue::EventQueue};

#[derive(Default, Clone)]
pub struct Handler {
    pub entries: Vec<Entry>,
}

impl Handler {
    pub fn new(entries: Vec<Entry>) -> Self {
        Self { entries }
    }

    pub fn empty() -> Self {
        Self { entries: vec![] }
    }

    pub fn extend<E: IntoIterator<Item = Entry>>(&mut self, entries: E) {
        self.entries.extend(entries);
    }

    pub async fn input(&self, event: Arc<Box<dyn Event>>) -> Result<()> {
        let mut queue = self.matches(&**event);
        while let Some(entry) = queue.pop() {
            entry.get_handler()(event.clone()).await?;
        }
        Ok(())
    }

    #[inline]
    pub fn matches(&self, event: &dyn Event) -> EventQueue<Entry> {
        let mut queue = EventQueue::new();
        for entry in self.entries.iter() {
            if entry.get_router().matches(event) {
                queue.push(entry.get_priority(), entry.clone());
            }
        }
        queue
    }
}

unsafe impl Send for Handler {}
unsafe impl Sync for Handler {}
