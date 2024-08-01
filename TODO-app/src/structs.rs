// Structs
#[derive(Debug)]
pub struct List {
    #[allow(dead_code)]
    pub id: i32,
    pub list_name: String,
    pub summary: Option<String>,
    pub category: Option<String>,
}

#[derive(Debug)]
pub struct Task {
    #[allow(dead_code)]
    pub id: i32,
    pub task_name: String,
    pub list_id: i32,
    pub list_name: String,
    pub priority: Option<Priority>,
    pub status: Option<Status>,
    pub tags: Option<Vec<String>>,
    pub deadline: Option<String>,
    pub completed_on: Option<String>,
    pub description: Option<String>,
}

// Enums
#[derive(Debug, Clone)]
pub enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone)]
pub enum Priority {
    High,
    Medium,
    Low,
}

impl From<i32> for Priority {
    fn from(value: i32) -> Self {
        match value {
            2 => Priority::High,
            1 => Priority::Medium,
            _ => Priority::Low,
        }
    }
}

impl From<&str> for Priority {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "high" => Priority::High,
            "medium" => Priority::Medium,
            "low" => Priority::Low,
            _ => Priority::Medium, // default to Medium if parsing fails
        }
    }
}

impl From<&str> for Status {
    fn from(s: &str) -> Self {
        match s {
            "not started" => Status::NotStarted,
            "in progress" => Status::InProgress,
            "completed" => Status::Completed,
            _ => Status::NotStarted, // default to NotStarted if parsing fails
        }
    }
}

impl From<i32> for Status {
    fn from(value: i32) -> Self {
        match value {
            2 => Status::Completed,
            1 => Status::InProgress,
            _ => Status::NotStarted,
        }
    }
}
