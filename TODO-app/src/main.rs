// Enums with Serialization/Deserialization

mod crud_op;

use crud_op::*;

use std::io;
use std::io::Write;
use termion::input::TermRead;
use console::Term;
use figlet_rs::FIGfont;
use rusqlite::Connection;
use std::fs;
use anyhow::Result;
//use rusqlite::{Connection, Result as SqlResult};


#[derive(Debug,Clone)]
enum Status {
    Completed,
    Pending,
}

impl From<Status> for i32 {
    fn from(status: Status) -> Self {
        match status {
            Status::Completed => 1,
            Status::Pending => 0,
        }
    }
}

impl From<i32> for Status {
    fn from(value: i32) -> Self {
        match value {
            1 => Status::Completed,
            _ => Status::Pending,
        }
    }
}

#[derive(Debug,Clone)]
enum Priority {
    High,
    Medium,
    Low,
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
    let db_file_path = "todo-list.db";
    if !db_exists(db_file_path) {
        println!("No existing database! New database will be created at: {}", db_file_path);
    } else {
        println!("Database exists! Opened DB created at: {}", db_file_path);
    }

    let term = Term::stdout();

    let conn = Connection::open(db_file_path)?;

    // Create tables
    create_list_table(&conn)?;
    create_task_table(&conn)?;
    let standard_font = FIGfont::standard().unwrap();
    let figure = standard_font.convert("My TODO app");

    let stdin = io::stdin();

    let mut exit = false;
    loop {
        print_welcome_screen(&term)?;

        for c in stdin.keys() {
            match c? {
                termion::event::Key::Char(c) if c.is_digit(10) => {
                    let choice = c.to_digit(10).unwrap();
                    match choice {
                        1 => insert_list(&conn),
                        2 => insert_task(&conn),
                        3 => fetch_list(&conn),
                        4 => fetch_tasks(&conn),
                        5 => {
                            writeln!(term, "bye!")?;
                            exit = true;
                            break;}
                        _ => writeln!(term, "Invalid input.")?,
                    }
                }
                _ => writeln!(term, "Invalid input.")?,
            }
        }
        if exit { break; }
    }

    // Example data
    //let list = List {
        //list_name: "Work".to_string(),
        //summary: Some("Work related tasks".to_string()),
        //category: Some("Professional".to_string()),
    //};

    //let task = Task {
        //id: 0,
        //task_name: "Complete report".to_string(),
        //list_name: list.list_name.clone(),
        //priority: Some(Priority::High),
        //status: Some(Status::Pending),
        //tags: Some(vec!["urgent".to_string(), "report".to_string()]),
        //deadline: Some("2024-12-31".to_string()),
        //completed_on: None,
        //description: Some("Finish the annual report by the end of the year".to_string()),
    //};

    //// Insert data
    //insert_list(&conn, &list)?;
    //insert_task(&conn, &task)?;

    //// Display lists with their tasks
    //display_list_with_tasks(&conn)?;

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
        "SHOW current LIST".to_string(),
        "SHOW current TASKS".to_string(),
        "Esc --> QUIT".to_string(),
    ];
    term.write_all(b"Press a corresponding number:\n")?;

    for (i, line) in msg.iter().enumerate() {
        term.write_all(format!("{}. {}\n", i + 1, line).as_bytes())?;
    }
    term.flush()?;

    Ok(())
}

// Display List with Tasks

fn display_list_with_tasks(conn: &Connection) -> Result<()> {
    let lists = fetch_lists(conn)?;
    for list in lists {
        println!("List: {:?}", list);
        let tasks = fetch_tasks(conn, &list.list_name)?;
        for task in tasks {
            println!("  Task: {:?}", task);
        }
    }
    Ok(())
}
