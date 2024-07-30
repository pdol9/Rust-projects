// Enums with Serialization/Deserialization

mod crud_op;

use crud_op::*;

use console::Term;
use figlet_rs::FIGfont;
use rusqlite::Connection;
use std::fs;
use anyhow::Result;
use std::io::{self, BufRead, Write};
use std::str::FromStr;

#[derive(Debug,Clone)]
enum Status {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug,Clone)]
enum Priority {
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

#[derive(Debug)]
struct List {
    list_name: String,
    summary: Option<String>,
    category: Option<String>,
}

#[derive(Debug)]
struct Task {
    id: i32,
    task_name: String,
    list_name: String,
    priority: Option<Priority>,
    status: Option<Status>,
    tags: Option<Vec<String>>,
    deadline: Option<String>,
    completed_on: Option<String>,
    description: Option<String>,
}

fn main() -> Result<()> {

    // init
    let db_file_path = "todo-list.db";
    if !db_exists(db_file_path) {
        println!("No existing database! New database will be created at: {}", db_file_path);
    } else {
        println!("Database exists! Opened DB created at: {}", db_file_path);
    }
    let conn = Connection::open(db_file_path)?;
    // Create tables
    create_list_table(&conn)?;
    create_task_table(&conn)?;

    let stdin = io::stdin();
    let mut term = Term::stdout();

    // main loop
    let mut exit = false;
    loop {
        print_welcome_screen(&term)?;

        let mut input = String::new();
        BufRead::read_line(&mut stdin.lock(), &mut input)?;

        for c in input.trim().chars() {
            if c.is_digit(10) {
                let choice = c.to_digit(10).unwrap();
                match choice {
                    1 => {
                        let list = prompt_list(&stdin)?;
                        insert_list(&conn, &list)?;
                    }
                    2 => {
                        let task = prompt_task(&stdin)?;
                        insert_task(&conn, &task)?;
                    }
                    3 => {
                        let _search_word = prompt_search_word(&stdin, "list")?;
                        let lists = fetch_lists(&conn/* , search_word.trim() */)?;
                        display_lists(&lists, &mut term)?;
                    }
                    4 => {
                        let search_word = prompt_search_word(&stdin, "task")?;
                        let tasks = fetch_tasks(&conn, search_word.trim())?;
                        display_tasks(&tasks, &mut term)?;
                    }
                    5 => {
                        writeln!(term, "bye!")?;
                        exit = true; // Set the flag to true to exit the outer loop
                        break;
                    }
                    _ => writeln!(term, "Invalid input.")?,
                }
            } else {
                writeln!(term, "Invalid input.")?;
            }
        }
        if exit { break; }
    }
    Ok(())
}

// create database
fn db_exists(db_file_path: &str) -> bool {
    fs::metadata(db_file_path).is_ok()
}

fn print_welcome_screen(mut term: &Term) -> io::Result<()> {
    term.clear_screen()?;

    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("TODO app");

    if let Some(figure) = figure {
        term.write_line(&figure.to_string())?;
    } else {
        term.write_line("Failed to generate ASCII art")?;
    }

    let msg: Vec<String> = vec![
        "Create a new LIST".to_string(),
        "Create a new TASK".to_string(),
        "Show current LIST".to_string(),
        "Show current TASKS".to_string(),
        "QUIT".to_string(),
    ];
    term.write_all(b"Press a corresponding number:\n")?;

    for (i, line) in msg.iter().enumerate() {
        term.write_all(format!("{}. {}\n", i + 1, line).as_bytes())?;
    }
    term.flush()?;

    Ok(())
}

// Display List with Tasks

fn display_lists(lists: &Vec<List>, mut term: &Term) -> Result<()> {

    for list in lists {
        writeln!(
            term,
            "List Name: {}, Summary: {}, Category: {}",
            list.list_name,
            list.summary.clone().unwrap_or("No summary".to_string()),
            list.category.clone().unwrap_or("No category".to_string())
        )?;
    }
    Ok(())
}

fn display_tasks(tasks: &Vec<Task>, mut term: &Term) -> Result<()> {

    for task in tasks {
        writeln!(term, "Task Name: {}, List Name: {}, Priority: {:?}, Status: {:?}, Tags: {:?}, Deadline: {:?}, Completed On: {:?}, Description: {:?}",
            task.task_name, task.list_name, task.priority, task.status, task.tags, task.deadline, task.completed_on, task.description)?;
    }
    Ok(())
}

fn prompt_list(stdin: &io::Stdin) -> Result<List, io::Error> {
    let mut list_name = String::new();
    let mut summary = String::new();
    let mut category = String::new();

    print!("Enter list name: ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut list_name)?;

    print!("Enter summary: ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut summary)?;

    print!("Enter category: ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut category)?;

    Ok(List {
        list_name: list_name.trim().to_string(),
        summary: Some(summary.trim().to_string()),
        category: Some(category.trim().to_string()),
    })
}

fn prompt_task(stdin: &io::Stdin) -> Result<Task, io::Error> {
    let mut task_name = String::new();
    let mut list_name = String::new();
    let mut priority = String::new();
    let mut status = String::new();
    let mut tags = String::new();
    let mut deadline = String::new();
    let mut description = String::new();

    print!("Enter task name: ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut task_name)?;

    print!("Enter list name: ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut list_name)?;

    print!("Enter priority: ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut priority)?;

    print!("Enter status: ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut status)?;

    print!("Enter tags (comma-separated): ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut tags)?;

    print!("Enter deadline: ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut deadline)?;

    print!("Enter description: ");
    io::stdout().flush()?;
    stdin.lock().read_line(&mut description)?;

    Ok(Task {
        id: 0,
        task_name: task_name.trim().to_string(),
        list_name: list_name.trim().to_string(),
        priority: Some(priority.trim().parse().unwrap()),
        status: Some(status.trim().parse().unwrap()),
        tags: Some(tags.trim().split(',').map(|s| s.to_string()).collect()),
        deadline: Some(deadline.trim().to_string()),
        completed_on: None,
        description: Some(description.trim().to_string()),
    })
}

fn prompt_search_word(stdin: &io::Stdin, for_what: &str) -> Result<String, io::Error> {
    let mut search_word = String::new();
    print!("Enter search word for {}: ", for_what);
    io::stdout().flush()?;
    stdin.lock().read_line(&mut search_word)?;
    Ok(search_word)
}