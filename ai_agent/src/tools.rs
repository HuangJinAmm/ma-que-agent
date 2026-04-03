use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use serde_json::Value;

/// Represents the input schema for a tool, conforming to JSON Schema format.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputSchema {
    #[serde(rename = "type")]
    pub schema_type: String, // Typically "object"
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, Property>>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
}

impl Default for InputSchema {
    fn default() -> Self {
        Self {
            schema_type: "object".to_string(),
            properties: Some(HashMap::new()),
            required: Some(Vec::new()),
        }
    }
}

/// Represents a property in the JSON Schema.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    #[serde(rename = "type")]
    pub property_type: String, // e.g., "string", "number", "boolean", "array", "object"
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Box<Property>>, // Used if type is "array"
    
    #[serde(rename = "enum", skip_serializing_if = "Option::is_none")]
    pub enum_values: Option<Vec<Value>>, // Used for enum types
}

/// Represents a Claude Tool definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    
    pub input_schema: InputSchema,
}

impl ToolDefinition {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: None,
            input_schema: InputSchema::default(),
        }
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn with_property(
        mut self,
        name: impl Into<String>,
        prop_type: impl Into<String>,
        description: impl Into<String>,
        required: bool,
    ) -> Self {
        let name_str = name.into();
        let prop = Property {
            property_type: prop_type.into(),
            description: Some(description.into()),
            items: None,
            enum_values: None,
        };

        if let Some(props) = &mut self.input_schema.properties {
            props.insert(name_str.clone(), prop);
        } else {
            let mut props = HashMap::new();
            props.insert(name_str.clone(), prop);
            self.input_schema.properties = Some(props);
        }

        if required {
            if let Some(req) = &mut self.input_schema.required {
                req.push(name_str);
            } else {
                self.input_schema.required = Some(vec![name_str]);
            }
        }
        
        self
    }
}

/// A Skill groups multiple atomic tools into a cohesive capability unit.
#[derive(Debug, Clone)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub tools: Vec<ToolDefinition>,
}

impl Skill {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            tools: Vec::new(),
        }
    }
    
    pub fn add_tool(&mut self, tool: ToolDefinition) {
        self.tools.push(tool);
    }

    pub fn with_tool(mut self, tool: ToolDefinition) -> Self {
        self.tools.push(tool);
        self
    }
}
