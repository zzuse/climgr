use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Command {
    pub id: String,
    pub name: String,
    pub script: String,
    pub shortcut: Option<String>,
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_struct_serialization() {
        let command = Command {
            id: "123".to_string(),
            name: "Test Command".to_string(),
            script: "echo hello".to_string(),
            shortcut: Some("Ctrl+T".to_string()),
            description: Some("A test command".to_string()),
        };

        let json = serde_json::to_string(&command).expect("Failed to serialize");

        // Deserialize back to verify
        let deserialized: Command = serde_json::from_str(&json).expect("Failed to deserialize");

        assert_eq!(command.id, deserialized.id);
        assert_eq!(command.name, deserialized.name);
        assert_eq!(command.script, deserialized.script);
        assert_eq!(command.shortcut, deserialized.shortcut);
        assert_eq!(command.description, deserialized.description);
    }
}
