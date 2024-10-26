use aionbot_macros::register;

struct ConcreteEvent {
    plain_data: String,
}

impl Event for ConcreteEvent {
    fn get_emitter_id(&self) -> &str {
        unimplemented!()
    }

    fn get_channel_id(&self) -> &str {
        unimplemented!()
    }

    fn get_plain_data(&self) -> Box<dyn std::any::Any> {
        unimplemented!()
    }

    fn get_content(&self) -> Box<dyn std::any::Any> {
        Box::new(self.plain_data.clone().leak() as &str)
    }

    fn get_type(&self) -> &str {
        unimplemented!()
    }
}

#[register(router = "test_router")]
pub fn test_register_fn(_event: Arc<Box<dyn Event>>) -> Result<()> {
    Ok(())
}

#[register(router = "test_router", priority = 1)]
pub fn test_register_fn_priority(_event: Arc<Box<dyn Event>>) -> Result<()> {
    Ok(())
}

#[test]
fn test_register_router() {
    let event: Box<dyn Event> = Box::new(ConcreteEvent {
        plain_data: "test_router".to_string(),
    });
    let entry = test_register_fn();
    assert!(entry.priority == 0);
    assert!(entry.router.matches(&*event));
}

#[test]
fn test_register_router_priority() {
    let event: Box<dyn Event> = Box::new(ConcreteEvent {
        plain_data: "test_router".to_string(),
    });
    let entry = test_register_fn_priority();
    assert!(entry.priority == 1);
    assert!(entry.router.matches(&*event));
}
