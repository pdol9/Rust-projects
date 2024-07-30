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

use std::io::{self, BufRead, Write};
use rusqlite::Connection;
use anyhow::Result;
use console::Term;

fn main() -> Result<()> {

    // init
    let db_file_path = "todo-app.db";
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
    let mut exit_app = false;
    while !exit_app {
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
                        exit_app = true; // Set the flag to true to exit the outer loop
                        break;
                    }
                    _ => writeln!(term, "Invalid input.")?,
                }
            } else {
                writeln!(term, "Invalid input.")?;
            }
        }
    }
    Ok(())
}
