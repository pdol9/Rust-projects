use std::str::FromStr;

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

impl From<Status> for i32 {
    fn from(status: Status) -> Self {
        match status {
            Status::NotStarted => 0,
            Status::InProgress => 1,
            Status::Completed => 2,
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

impl From<Priority> for i32 {
    fn from(priority: Priority) -> Self {
        match priority {
            Priority::High => 2,
            Priority::Medium => 1,
            Priority::Low => 0,
        }
    }
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

impl FromStr for Priority {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Low" => Ok(Priority::Low),
            "Medium" => Ok(Priority::Medium),
            "High" => Ok(Priority::High),
            _ => Err(()),
        }
    }
}

impl FromStr for Status {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "NotStarted" => Ok(Status::NotStarted),
            "InProgress" => Ok(Status::InProgress),
            "Completed" => Ok(Status::Completed),
            _ => Err(()),
        }
    }
}
