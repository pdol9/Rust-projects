use crate::structs::{List, Task};

use anyhow::Result;
use console::Term;
use figlet_rs::FIGfont;
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

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

pub fn display_lists(stdin: &io::Stdin, lists: &Vec<List>, mut term: &Term) -> Result<()> {
    for list in lists {
        writeln!(
            term,
            "List Name: {}, Summary: {}, Category: {}",
            list.list_name,
            list.summary.clone().unwrap_or("No summary".to_string()),
            list.category.clone().unwrap_or("No category".to_string())
        )?;
    }
    let mut buffer = String::new();
    print!("\n\t Press ENTER to continue.");
    io::stdout().flush()?;
    stdin.read_line(&mut buffer)?;

    Ok(())
}

pub fn display_tasks(stdin: &io::Stdin, tasks: &Vec<Task>, mut term: &Term) -> Result<()> {
    for task in tasks {
        writeln!(term, "Task Name: {}, List ID: {}, Priority: {:?}, Status: {:?}, Tags: {:?}, Deadline: {:?}, Completed On: {:?}, Description: {:?}",
            task.task_name, task.list_id, task.priority, task.status, task.tags, task.deadline, task.completed_on, task.description)?;
    }
    let mut buffer = String::new();
    print!("\n\t Press ENTER to continue.");
    io::stdout().flush()?;
    stdin.read_line(&mut buffer)?;

    Ok(())
}

pub fn prompt_list(stdin: &io::Stdin) -> Result<List, io::Error> {
    let mut buffer_strings: HashMap<&str, String> = [
        ("Enter list name: ", String::new()),
        ("Enter summary: ", String::new()),
        ("Enter category: ", String::new()),
    ]
    .iter()
    .cloned()
    .collect();

    for (prompt, buffer) in buffer_strings.iter_mut() {
        print!("{}", prompt);
        io::stdout().flush()?;
        stdin.lock().read_line(buffer)?;
    }

    Ok(List {
        id: 0,
        list_name: buffer_strings
            .get("Enter list name: ")
            .unwrap()
            .trim()
            .to_string(),
        summary: Some(
            buffer_strings
                .get("Enter summary: ")
                .unwrap()
                .trim()
                .to_string(),
        ),
        category: Some(
            buffer_strings
                .get("Enter category: ")
                .unwrap()
                .trim()
                .to_string(),
        ),
    })
}

pub fn prompt_task(stdin: &io::Stdin) -> Result<Task, io::Error> {
    let mut buffer_strings: HashMap<&str, String> = [
        ("Enter task name: ", String::new()),
        ("Enter list name: ", String::new()),
        ("Enter priority: ", String::new()),
        ("Enter status: ", String::new()),
        ("Enter tags: ", String::new()),
        ("Enter deadline: ", String::new()),
        ("Enter description: ", String::new()),
    ]
    .iter()
    .cloned()
    .collect();

    for (prompt, buffer) in buffer_strings.iter_mut() {
        print!("{}", prompt);
        io::stdout().flush()?;
        stdin.lock().read_line(buffer)?;
    }

    Ok(Task {
        id: 0,
        task_name: buffer_strings
            .get("Enter task name: ")
            .unwrap()
            .trim()
            .to_string(),
        list_id: 0,
        list_name: buffer_strings
            .get("Enter list name: ")
            .unwrap()
            .trim()
            .to_string(),
        priority: Some(
            buffer_strings
                .get("Enter priority: ")
                .unwrap()
                .trim()
                .parse()
                .unwrap(),
        ),
        status: Some(
            buffer_strings
                .get("Enter status: ")
                .unwrap()
                .trim()
                .parse()
                .unwrap(),
        ),
        tags: Some(
            buffer_strings
                .get("Enter tags: ")
                .unwrap()
                .trim()
                .split(',')
                .map(|s| s.to_string())
                .collect(),
        ),
        deadline: Some(
            buffer_strings
                .get("Enter deadline: ")
                .unwrap()
                .trim()
                .to_string(),
        ),
        completed_on: None,
        description: Some(
            buffer_strings
                .get("Enter description: ")
                .unwrap()
                .trim()
                .to_string(),
        ),
    })
}

pub fn prompt_search_word(stdin: &io::Stdin, for_what: &str) -> Result<String, io::Error> {
    let mut search_word = String::new();
    print!("Enter search word for {}: ", for_what);
    io::stdout().flush()?;
    stdin.lock().read_line(&mut search_word)?;
    Ok(search_word)
}
