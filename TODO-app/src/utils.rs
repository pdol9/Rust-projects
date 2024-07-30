
use crate::structs::{List, Task};

use std::fs;
use std::io::{self, Write, BufRead};
use figlet_rs::FIGfont;
use console::Term;
use anyhow::Result;

// create database
pub fn db_exists(db_file_path: &str) -> bool {
    fs::metadata(db_file_path).is_ok()
}

pub fn print_welcome_screen(mut term: &Term) -> io::Result<()> {
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

pub fn display_lists(lists: &Vec<List>, mut term: &Term) -> Result<()> {

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

pub fn display_tasks(tasks: &Vec<Task>, mut term: &Term) -> Result<()> {

    for task in tasks {
        writeln!(term, "Task Name: {}, List Name: {}, Priority: {:?}, Status: {:?}, Tags: {:?}, Deadline: {:?}, Completed On: {:?}, Description: {:?}",
            task.task_name, task.list_name, task.priority, task.status, task.tags, task.deadline, task.completed_on, task.description)?;
    }
    Ok(())
}

pub fn prompt_list(stdin: &io::Stdin) -> Result<List, io::Error> {
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

pub fn prompt_task(stdin: &io::Stdin) -> Result<Task, io::Error> {
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

pub fn prompt_search_word(stdin: &io::Stdin, for_what: &str) -> Result<String, io::Error> {
    let mut search_word = String::new();
    print!("Enter search word for {}: ", for_what);
    io::stdout().flush()?;
    stdin.lock().read_line(&mut search_word)?;
    Ok(search_word)
}