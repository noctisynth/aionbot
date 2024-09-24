use crate::event::Event;

pub trait Router {
    fn matches(&self, event: &Event) -> bool;
}

impl Router for str {
    fn matches(&self, event: &Event) -> bool {
        self == event.plain_data.to_string()
    }
}

impl Router for &str {
    fn matches(&self, event: &Event) -> bool {
        self == &event.plain_data.to_string()
    }
}

impl Router for String {
    fn matches(&self, event: &Event) -> bool {
        self == &event.plain_data.to_string()
    }
}

#[derive(Default)]
pub struct AnyRouter;

impl Router for AnyRouter {
    fn matches(&self, _event: &Event) -> bool {
        true
    }
}

pub struct ExactMatchRouter {
    pattern: String,
    ignore_spaces: bool,
}

impl ExactMatchRouter {
    pub fn new(pattern: &str, ignore_spaces: bool) -> Self {
        Self {
            pattern: pattern.to_string(),
            ignore_spaces,
        }
    }
}

impl Router for ExactMatchRouter {
    fn matches(&self, event: &Event) -> bool {
        let message = event.plain_data.to_string();
        if self.ignore_spaces {
            message.replace(" ", "") == self.pattern.replace(" ", "")
        } else {
            message == self.pattern
        }
    }
}

pub struct RegexRouter {
    pattern: regex::Regex,
}

impl RegexRouter {
    pub fn new(pattern: &str) -> Self {
        Self {
            pattern: regex::Regex::new(pattern).unwrap(),
        }
    }
}

impl Router for RegexRouter {
    fn matches(&self, event: &Event) -> bool {
        self.pattern.is_match(&event.plain_data.to_string())
    }
}

// pub struct StartsWithRouter {
//     pattern: String,
//     strip: bool,
// }

// impl StartsWithRouter {
//     pub fn new(pattern: &str, ignore_spaces: bool) -> Self {
//         Self {
//             pattern: pattern.to_string(),
//             strip: ignore_spaces,
//         }
//     }
// }

// impl Router for StartsWithRouter {
//     fn matches(&self, message: &str) -> bool {
//         if self.strip {
//             message
//                 .replace(" ", "")
//                 .starts_with(&self.pattern.replace(" ", ""))
//         } else {
//             message.starts_with(&self.pattern)
//         }
//     }
// }

// pub struct ContainsRouter {
//     pattern: String,
//     ignore_spaces: bool,
// }

// impl ContainsRouter {
//     pub fn new(pattern: &str, ignore_spaces: bool) -> Self {
//         Self {
//             pattern: pattern.to_string(),
//             ignore_spaces,
//         }
//     }
// }

// impl Router for ContainsRouter {
//     fn matches(&self, message: &str) -> bool {
//         if self.ignore_spaces {
//             message
//                 .replace(" ", "")
//                 .contains(&self.pattern.replace(" ", ""))
//         } else {
//             message.contains(&self.pattern)
//         }
//     }
// }

// pub struct EndsWithRouter {
//     pattern: String,
//     ignore_spaces: bool,
// }

// impl EndsWithRouter {
//     pub fn new(pattern: &str, ignore_spaces: bool) -> Self {
//         Self {
//             pattern: pattern.to_string(),
//             ignore_spaces,
//         }
//     }
// }

// impl Router for EndsWithRouter {
//     fn matches(&self, message: &str) -> bool {
//         if self.ignore_spaces {
//             message
//                 .replace(" ", "")
//                 .ends_with(&self.pattern.replace(" ", ""))
//         } else {
//             message.ends_with(&self.pattern)
//         }
//     }
// }

// pub struct OrRouter {
//     routers: Vec<Box<dyn Router>>,
// }

// impl OrRouter {
//     pub fn new(routers: Vec<Box<dyn Router>>) -> Self {
//         Self { routers }
//     }
// }

// impl Router for OrRouter {
//     fn matches(&self, message: &str) -> bool {
//         self.routers.iter().any(|r| r.matches(message))
//     }
// }

// pub struct AndRouter {
//     routers: Vec<Box<dyn Router>>,
// }

// impl AndRouter {
//     pub fn new(routers: Vec<Box<dyn Router>>) -> Self {
//         Self { routers }
//     }
// }

// impl Router for AndRouter {
//     fn matches(&self, message: &str) -> bool {
//         self.routers.iter().all(|r| r.matches(message))
//     }
// }

// pub struct NotRouter {
//     router: Box<dyn Router>,
// }

// impl NotRouter {
//     pub fn new(router: Box<dyn Router>) -> Self {
//         Self { router }
//     }
// }

// impl Router for NotRouter {
//     fn matches(&self, message: &str) -> bool {
//         !self.router.matches(message)
//     }
// }

#[cfg(test)]
mod tests {
    use std::cell::LazyCell;

    use crate::event::Message;

    use super::*;

    const HELLO_EVENT: LazyCell<Event> = LazyCell::new(|| Event {
        plain_data: Message {
            segments: vec!["hello".into()],
            ..Default::default()
        },
        ..Default::default()
    });

    const WORLD_EVENT: LazyCell<Event> = LazyCell::new(|| Event {
        plain_data: Message {
            segments: vec!["world".into()],
            ..Default::default()
        },
        ..Default::default()
    });

    const HELLO_WORLD_EVENT: LazyCell<Event> = LazyCell::new(|| Event {
        plain_data: Message {
            segments: vec!["hello".into(), " ".into(), "world".into()],
            ..Default::default()
        },
        ..Default::default()
    });

    const HAPPY_HELLO_WORLD_EVENT: LazyCell<Event> = LazyCell::new(|| Event {
        plain_data: Message {
            segments: vec!["hello".into(), " ".into(), "world!".into()],
            ..Default::default()
        },
        ..Default::default()
    });

    #[test]
    fn test_exact_match_router() {
        let router = ExactMatchRouter::new("hello", false);
        assert!(router.matches(&HELLO_EVENT));
        assert!(!router.matches(&WORLD_EVENT));
        assert!(!router.matches(&HELLO_WORLD_EVENT));
    }

    #[test]
    fn test_exact_match_router_ignore_spaces() {
        let router = ExactMatchRouter::new("hello world", true);
        assert!(router.matches(&HELLO_WORLD_EVENT));
        assert!(!router.matches(&HELLO_EVENT));
        assert!(!router.matches(&WORLD_EVENT));
        assert!(!router.matches(&HAPPY_HELLO_WORLD_EVENT));
    }

    #[test]
    fn test_regex_router() {
        let router = RegexRouter::new(r"^hello.*$");
        assert!(router.matches(&HELLO_WORLD_EVENT));
        assert!(!router.matches(&WORLD_EVENT));
        assert!(router.matches(&HAPPY_HELLO_WORLD_EVENT));
    }

    // #[test]
    // fn test_starts_with_router() {
    //     let router = StartsWithRouter::new("hello", false);
    //     assert!(router.matches("hello world"));
    //     assert!(!router.matches("world"));
    //     assert!(router.matches("hello world!"));
    // }

    // #[test]
    // fn test_starts_with_router_ignore_spaces() {
    //     let router = StartsWithRouter::new("hello world", true);
    //     assert!(router.matches("hello world"));
    //     assert!(!router.matches("hello"));
    //     assert!(!router.matches("world"));
    //     assert!(router.matches("hello world!"));
    // }

    // #[test]
    // fn test_contains_router() {
    //     let router = ContainsRouter::new("world", false);
    //     assert!(router.matches("hello world"));
    //     assert!(!router.matches("hello"));
    //     assert!(router.matches("world"));
    //     assert!(router.matches("hello world!"));
    // }

    // #[test]
    // fn test_contains_router_ignore_spaces() {
    //     let router = ContainsRouter::new("hello world", true);
    //     assert!(router.matches("hello world"));
    //     assert!(!router.matches("hello"));
    //     assert!(!router.matches("world"));
    //     assert!(router.matches("hello world!"));
    // }

    // #[test]
    // fn test_ends_with_router() {
    //     let router = EndsWithRouter::new("world", false);
    //     assert!(router.matches("hello world"));
    //     assert!(!router.matches("hello"));
    //     assert!(router.matches("world"));
    //     assert!(!router.matches("hello world!"));
    // }

    // #[test]
    // fn test_ends_with_router_ignore_spaces() {
    //     let router = EndsWithRouter::new("hello world", true);
    //     assert!(router.matches("hello world"));
    //     assert!(!router.matches("hello"));
    //     assert!(!router.matches("world"));
    //     assert!(!router.matches("hello world!"));
    // }

    // #[test]
    // fn test_or_router() {
    //     let router = OrRouter::new(vec![
    //         Box::new(ExactMatchRouter::new("hello", false)),
    //         Box::new(ExactMatchRouter::new("world", false)),
    //     ]);
    //     assert!(router.matches("hello"));
    //     assert!(router.matches("world"));
    //     assert!(!router.matches("foo"));
    // }

    // #[test]
    // fn test_and_router() {
    //     let router = AndRouter::new(vec![
    //         Box::new(ExactMatchRouter::new("hello", false)),
    //         Box::new(ExactMatchRouter::new("world", false)),
    //     ]);
    //     assert!(!router.matches("hello"));
    //     assert!(!router.matches("world"));
    //     assert!(!router.matches("foo"));
    //     assert!(!router.matches("hello world"));
    // }

    // #[test]
    // fn test_not_router() {
    //     let router = NotRouter::new(Box::new(ExactMatchRouter::new("hello", false)));
    //     assert!(!router.matches("hello"));
    //     assert!(router.matches("world"));
    // }
}
