use crate::event::Event;

pub trait Router: Send + Sync {
    fn matches(&self, event: &Box<dyn Event>) -> bool;
}

impl Router for &str {
    fn matches(&self, event: &Box<dyn Event>) -> bool {
        if let Ok(val) = event.get_plain_data().downcast::<String>() {
            &*val == self
        } else {
            false
        }
    }
}

pub struct ExactMatchRouter<T>
where
    T: Send + Sync,
{
    pub pattern: T,
}

impl<T> Router for ExactMatchRouter<T>
where
    T: Send + Sync + PartialEq + 'static,
{
    fn matches(&self, event: &Box<dyn Event>) -> bool {
        if let Ok(val) = event.get_plain_data().downcast::<T>() {
            *val == self.pattern
        } else {
            false
        }
    }
}

impl<T> ExactMatchRouter<T>
where
    T: Send + Sync,
{
    pub fn new(pattern: T) -> Self {
        Self { pattern }
    }
}

#[derive(Default)]
pub struct AnyRouter;

impl Router for AnyRouter {
    fn matches(&self, _event: &Box<dyn Event>) -> bool {
        true
    }
}
