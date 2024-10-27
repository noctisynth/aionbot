use crate::entry::Entry;

#[derive(Default)]
pub struct AionPlugin {
    name: &'static str,
    entries: Vec<Entry>,
}

impl AionPlugin {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn entries(&self) -> &[Entry] {
        &self.entries
    }

    pub fn invoke_handler(mut self, entries: Vec<Entry>) -> Self {
        self.entries.extend(entries);
        self
    }
}
