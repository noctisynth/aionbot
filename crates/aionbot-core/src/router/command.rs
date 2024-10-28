use crate::event::Event;

use super::Router;

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
