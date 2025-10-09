#[cfg(test)]
mod tests {
    use taskur_backend::models::task::{TaskStatus, TaskPriority};

    #[test]
    fn test_task_status_serialization() {
        // Test serialization to lowercase
        let status = TaskStatus::Todo;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"todo\"");

        let status = TaskStatus::InProgress;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"inprogress\"");

        let status = TaskStatus::Completed;
        let json = serde_json::to_string(&status).unwrap();
        assert_eq!(json, "\"completed\"");
    }

    #[test]
    fn test_task_status_deserialization() {
        // Test deserialization from lowercase
        let status: TaskStatus = serde_json::from_str("\"todo\"").unwrap();
        matches!(status, TaskStatus::Todo);

        let status: TaskStatus = serde_json::from_str("\"inprogress\"").unwrap();
        matches!(status, TaskStatus::InProgress);

        let status: TaskStatus = serde_json::from_str("\"completed\"").unwrap();
        matches!(status, TaskStatus::Completed);
    }

    #[test]
    fn test_task_priority_serialization() {
        let priority = TaskPriority::Low;
        let json = serde_json::to_string(&priority).unwrap();
        assert_eq!(json, "\"low\"");

        let priority = TaskPriority::Medium;
        let json = serde_json::to_string(&priority).unwrap();
        assert_eq!(json, "\"medium\"");

        let priority = TaskPriority::High;
        let json = serde_json::to_string(&priority).unwrap();
        assert_eq!(json, "\"high\"");

        let priority = TaskPriority::Urgent;
        let json = serde_json::to_string(&priority).unwrap();
        assert_eq!(json, "\"urgent\"");
    }

    #[test]
    fn test_task_priority_deserialization() {
        let priority: TaskPriority = serde_json::from_str("\"low\"").unwrap();
        matches!(priority, TaskPriority::Low);

        let priority: TaskPriority = serde_json::from_str("\"medium\"").unwrap();
        matches!(priority, TaskPriority::Medium);

        let priority: TaskPriority = serde_json::from_str("\"high\"").unwrap();
        matches!(priority, TaskPriority::High);

        let priority: TaskPriority = serde_json::from_str("\"urgent\"").unwrap();
        matches!(priority, TaskPriority::Urgent);
    }

    #[test]
    fn test_invalid_status_deserialization() {
        let result: Result<TaskStatus, _> = serde_json::from_str("\"invalid\"");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_priority_deserialization() {
        let result: Result<TaskPriority, _> = serde_json::from_str("\"invalid\"");
        assert!(result.is_err());
    }
}
