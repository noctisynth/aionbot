use std::{error::Error, marker::PhantomData};

use crate::event::Event;

use super::Router;

pub struct ErrorRouter<E: Error> {
    marker: PhantomData<E>,
}

impl<E: Error> ErrorRouter<E> {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<E: Error> Default for ErrorRouter<E> {
    fn default() -> Self {
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
