/*  *************************************  */
/*                                         */
/*                TODO app                 */
/*                                         */
/*  *************************************  */

mod crud_op;
mod structs;
mod utils;

use crud_op::*;
use utils::*;

use anyhow::Result;
use console::Term;
use rusqlite::Connection;
use std::io::{self, stdin, Write};
use std::path::Path;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    // init
    let db_file_path = "todo-app.db";
    if Path::new(db_file_path).exists() {
        println!("Database exists! Opened DB created at: {}", db_file_path);
    } else {
        println!(
            "No existing database! New database will be created at: {}",
            db_file_path
        );
    }
    let conn = Connection::open(db_file_path)?;
    // Create tables
    create_list_table(&conn)?;
    create_task_table(&conn)?;

    // let stdin = io::stdin();
    let stdin = stdin();
    let mut term = Term::stdout();

    // main loop
    let mut exit_app = false;
    while !exit_app {
        print_welcome_screen(&term)?;
        let mut input = String::new();
        stdin.read_line(&mut input)?;

        // Trim the input to remove any leading/trailing whitespace
        let input = input.trim();

        // Check if the input is a single character and a digit
        if input.len() == 1 {
            let input_char = input.chars().next().unwrap();
            if let Some(choice) = input_char.to_digit(10) {
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
                        let lists = fetch_lists(&conn /* , search_word.trim() */)?;
                        let _ = display_lists(&stdin, &lists, &mut term)?;
                    }
                    4 => {
                        let search_word = prompt_search_word(&stdin, "task")?;
                        let tasks = fetch_tasks(&conn, search_word.trim())?;
                        let _ = display_tasks(&stdin, &tasks, &mut term)?;
                    }
                    5 => {
                        writeln!(term, "Bye!")?;
                        exit_app = true; // Set the flag to true to exit the outer loop
                    }
                    _ => {
                        writeln!(term, "---  Invalid number choice.  ---")?;
                        sleep(Duration::from_secs(2));
                        io::stdout().flush()?;
                    }
                }
            } else {
                writeln!(term, "--- Invalid input. Please try again! ---")?;
                sleep(Duration::from_secs(2));
                io::stdout().flush()?;
            }
        }
    }
    Ok(())
}
