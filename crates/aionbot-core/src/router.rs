use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::event::Event;

pub trait Router: Send + Sync {
    fn matches(&self, event: &dyn Event) -> bool;
}

impl Router for &str {
    fn matches(&self, event: &dyn Event) -> bool {
        if let Ok(val) = event.content().downcast::<&str>() {
            &*val == self
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
    pub command: String,
}

impl Default for CommandRouter {
    fn default() -> Self {
        Self {
            prefixes: vec!["/".to_string()],
            command: "help".to_string(),
        }
    }
}

impl CommandRouter {
    pub fn new(prefixes: Vec<String>, command: String) -> Self {
        Self { prefixes, command }
    }

    pub fn command(command: impl ToString) -> Self {
        Self {
            command: command.to_string(),
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
                    if command.starts_with(self.command.as_str()) {
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

        let router = CommandRouter::command("cmd");
        assert!(!router.matches(&"help".to_string()));
        assert!(router.matches(&"/cmd".to_string()));
        assert!(router.matches(&"/cmd@bot".to_string()));
        assert!(!router.matches(&"/not cmd".to_string()));

        let router = CommandRouter::new(vec!["!".to_string()], "cmd".to_string());
        assert!(!router.matches(&"help".to_string()));
        assert!(router.matches(&"!cmd".to_string()));
        assert!(router.matches(&"!cmd@bot".to_string()));
        assert!(!router.matches(&"!not cmd".to_string()));
        assert!(!router.matches(&"/cmd arg1 arg2".to_string()))
    }
}
