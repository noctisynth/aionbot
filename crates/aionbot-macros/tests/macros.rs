use aionbot_core::event::{Message, MessageSegment};
use aionbot_macros::register;

#[register(router = "test_router")]
pub fn test_register_fn(_event: Arc<Event>) -> Result<()> {
    Ok(())
}

#[register(router = "test_router", priority = 1)]
pub fn test_register_fn_priority(_event: Arc<Event>) -> Result<()> {
    Ok(())
}

#[test]
fn test_register_router() {
    let event = Event {
        plain_data: Message {
            entity: Some("test".to_string()),
            segments: vec![MessageSegment {
                text: "test_router".to_string(),
                r#type: "text".to_string(),
            }],
        },
        user_id: "test".to_string(),
        channel_id: "test".to_string(),
        timestamp: "test".to_string(),
        event_type: "text".to_string(),
        variables: Default::default(),
    };
    let entry = test_register_fn();
    assert!(entry.priority == 0);
    assert!(entry.router.matches(&event));
}

#[test]
fn test_register_router_priority() {
    let event = Event {
        plain_data: Message {
            entity: Some("test".to_string()),
            segments: vec![MessageSegment {
                text: "test_router".to_string(),
                r#type: "text".to_string(),
            }],
        },
        user_id: "test".to_string(),
        channel_id: "test".to_string(),
        timestamp: "test".to_string(),
        event_type: "text".to_string(),
        variables: Default::default(),
    };
    let entry = test_register_fn_priority();
    assert!(entry.priority == 1);
    assert!(entry.router.matches(&event));
}
