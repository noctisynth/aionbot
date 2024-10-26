use crate::event::Event;

pub trait Router: Send + Sync {
    fn matches(&self, event: &dyn Event) -> bool;
}

impl Router for &str {
    fn matches(&self, event: &dyn Event) -> bool {
        if let Ok(val) = event.get_content().downcast::<&str>() {
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
        if let Ok(val) = event.get_content().downcast::<T>() {
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
        if let Ok(val) = event.get_content().downcast::<&str>() {
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

#[derive(Default)]
pub struct AnyRouter;

impl Router for AnyRouter {
    fn matches(&self, _event: &dyn Event) -> bool {
        true
    }
}
