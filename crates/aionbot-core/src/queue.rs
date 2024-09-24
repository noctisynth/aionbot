use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::hash::Hash;

#[derive(Clone, Eq, PartialEq)]
struct EventEntry<T> {
    priority: i8,
    counter: usize,
    item: T,
}

impl<T: Eq + PartialEq> EventEntry<T> {
    fn new(priority: i8, counter: usize, item: T) -> Self {
        Self {
            priority,
            counter,
            item,
        }
    }
}

impl<T: Eq + PartialEq> Ord for EventEntry<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // In order to make BinaryHeap a min-heap, we use `Reverse`.
        Reverse(self.priority)
            .cmp(&Reverse(other.priority))
            .then(self.counter.cmp(&other.counter))
    }
}

impl<T: Eq + PartialEq> PartialOrd for EventEntry<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct EventQueue<T: PartialEq + Hash + Clone> {
    heap: BinaryHeap<EventEntry<T>>,
    entry_finder: HashMap<T, Option<EventEntry<T>>>,
    counter: usize,
    order_queue: VecDeque<T>,
}

impl<T: Eq + Hash + Clone> EventQueue<T> {
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
            entry_finder: HashMap::new(),
            counter: 0,
            order_queue: VecDeque::new(),
        }
    }

    pub fn push(&mut self, priority: i8, item: T) {
        if self.entry_finder.contains_key(&item) {
            self.remove(&item);
        }
        let entry = EventEntry::new(priority, self.counter, item.clone());
        self.counter += 1;
        self.entry_finder.insert(item.clone(), Some(entry.clone()));
        self.heap.push(entry);
        self.order_queue.push_front(item);
    }

    pub fn pop(&mut self) -> Option<T> {
        while let Some(EventEntry {
            priority: _,
            counter: _,
            item,
        }) = self.heap.pop()
        {
            if let Some(Some(_)) = self.entry_finder.remove(&item) {
                self.order_queue.retain(|x| x != &item);
                return Some(item);
            }
        }
        None
    }

    pub fn remove(&mut self, item: &T) {
        if let Some(Some(mut entry)) = self.entry_finder.remove(item) {
            entry.item = item.clone(); // Invalidate the entry
            self.order_queue.retain(|x| x != item);
        }
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }
}

impl<T: Eq + Hash + Clone> Default for EventQueue<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_queue() {
        let mut queue = EventQueue::new();
        queue.push(1, "a");
        queue.push(2, "b");
        queue.push(3, "c");
        queue.push(2, "d");
        queue.push(1, "e");
        assert_eq!(queue.pop(), Some("e"));
        assert_eq!(queue.pop(), Some("a"));
        assert_eq!(queue.pop(), Some("d"));
        assert_eq!(queue.pop(), Some("b"));
        assert_eq!(queue.pop(), Some("c"));
        assert_eq!(queue.pop(), None);
    }
}
