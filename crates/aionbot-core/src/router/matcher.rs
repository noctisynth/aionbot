use crate::event::Event;

use super::Router;

pub struct ExactMatchRouter<T>
where
    T: Send + Sync + PartialEq + 'static,
{
    pub pattern: T,
}

impl<T> Router for ExactMatchRouter<T>
where
    T: Send + Sync + PartialEq + 'static,
{
    fn matches(&self, event: &dyn Event) -> bool {
        if let Ok(val) = event.content().downcast::<T>() {
            *val == self.pattern
        } else {
            false
        }
    }
}

impl<T> ExactMatchRouter<T>
where
    T: Send + Sync + PartialEq + 'static,
{
    pub fn new(pattern: T) -> Self {
        Self { pattern }
    }
}

pub struct StartsWithRouter<T>
where
    T: Send + Sync + AsRef<str> + 'static,
{
    pub pattern: T,
}

impl Router for StartsWithRouter<&str> {
    fn matches(&self, event: &dyn Event) -> bool {
        if let Ok(val) = event.content().downcast::<&str>() {
            val.starts_with(self.pattern)
        } else {
            false
        }
    }
}

impl<T> StartsWithRouter<T>
where
    T: Send + Sync + AsRef<str> + 'static,
{
    pub fn new(pattern: T) -> Self {
        Self { pattern }
    }
}

pub struct ContainsRouter<T>
where
    T: Send + Sync + AsRef<str> + 'static,
{
    pub pattern: T,
}

impl Router for ContainsRouter<&str> {
    fn matches(&self, event: &dyn Event) -> bool {
        if let Ok(val) = event.content().downcast::<&str>() {
            val.contains(self.pattern)
        } else {
            false
        }
    }
}

impl<T> ContainsRouter<T>
where
    T: Send + Sync + AsRef<str> + 'static,
{
    pub fn new(pattern: T) -> Self {
        Self { pattern }
    }
}

pub struct EndsWithRouter<T>
where
    T: Send + Sync + AsRef<str> + 'static,
{
    pub pattern: T,
}

impl Router for EndsWithRouter<&str> {
    fn matches(&self, event: &dyn Event) -> bool {
        if let Ok(val) = event.content().downcast::<&str>() {
            val.ends_with(self.pattern)
        } else {
            false
        }
    }
}

impl<T> EndsWithRouter<T>
where
    T: Send + Sync + AsRef<str> + 'static,
{
    pub fn new(pattern: T) -> Self {
        Self { pattern }
    }
}
