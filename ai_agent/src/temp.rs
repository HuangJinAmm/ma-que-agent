use genai::chat::{ChatMessage, ToolCall, ToolResponse};
use serde_json::json;

pub fn test_msg() {
    let tc = ToolCall {
        call_id: "123".to_string(),
        fn_name: "test".to_string(),
        fn_arguments: json!({}),
        thought_signatures: None,
    };
    let msg5 = ChatMessage::from(vec![tc]);
    
    let tr = ToolResponse::new("123", "result");
    let msg6 = ChatMessage::from(tr);
}