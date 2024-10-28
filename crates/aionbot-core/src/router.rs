use std::{error::Error, marker::PhantomData};

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

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

pub struct CommandRouter {
    pub prefixes: Vec<String>,
    pub command: Vec<String>,
}

impl Default for CommandRouter {
    fn default() -> Self {
        Self {
            prefixes: vec!["/".into()],
            command: ["help".into()].to_vec(),
        }
    }
}

impl CommandRouter {
    pub fn new<S: Into<String>, C: IntoIterator<Item = S>>(
        prefixes: Vec<String>,
        command: C,
    ) -> Self {
        Self {
            prefixes,
            command: command.into_iter().map(Into::into).collect(),
        }
    }

    pub fn command<S: Into<String>, C: IntoIterator<Item = S>>(command: C) -> Self {
        Self {
            command: command.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

impl Router for CommandRouter {
    fn matches(&self, event: &dyn Event) -> bool {
        if let Ok(val) = event.content().downcast::<&str>() {
            for prefix in &self.prefixes {
                if val.starts_with(prefix) {
                    let command = val.strip_prefix(prefix).unwrap();
                    if self.command.iter().any(|c| command.starts_with(c)) {
                        return true;
                    }
                }
            }
            false
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_router() {
        let router = CommandRouter::default();
        assert!(!router.matches(&"help".to_string()));
        assert!(router.matches(&"/help".to_string()));
        assert!(router.matches(&"/help@bot".to_string()));
        assert!(!router.matches(&"/not help".to_string()));

        let router = CommandRouter::command(["cmd", "command"]);
        assert!(!router.matches(&"help".to_string()));
        assert!(router.matches(&"/cmd".to_string()));
        assert!(router.matches(&"/cmd@bot".to_string()));
        assert!(!router.matches(&"/not cmd".to_string()));
        assert!(router.matches(&"/command".to_string()));

        let router = CommandRouter::new(vec!["!".to_string()], ["cmd"]);
        assert!(!router.matches(&"help".to_string()));
        assert!(router.matches(&"!cmd".to_string()));
        assert!(router.matches(&"!cmd@bot".to_string()));
        assert!(!router.matches(&"!not cmd".to_string()));
        assert!(!router.matches(&"/cmd arg1 arg2".to_string()))
    }
}

pub struct ErrorRouter<E: Error> {
    marker: PhantomData<E>,
}

impl<E: Error> ErrorRouter<E> {
    pub fn new() -> Self {
        Self {
            marker: PhantomData,
        }
    }
}

impl<E: Error + Send + Sync + 'static> Router for ErrorRouter<E> {
    fn matches(&self, event: &dyn Event) -> bool {
        event.content().downcast::<E>().is_ok()
    }
}
