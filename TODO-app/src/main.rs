/*  ****************************************************************************  */
/*                                                                                */
/*                                     TODO app                                   */
/*                                                                                */
/*  ****************************************************************************  */

mod crud_op;
mod structs;
mod utils;

use crud_op::*;
use utils::*;

use anyhow::Result;
use console::Term;
use std::io::{self, Write};
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<()> {
    // initialization
    let conn = set_up_db_connection()?;
    let stdin = io::stdin();
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
                        let search_word = prompt_search_word(&stdin)?;
                        let lists = fetch_lists(&conn, search_word.trim())?;
                        let _ = display_lists(&stdin, &lists, &mut term)?;
                    }
                    4 => {
                        let search_word = prompt_search_word(&stdin)?;
                        let tasks = fetch_tasks(&conn, search_word.trim())?;
                        let _ = display_tasks(&stdin, &tasks, &mut term)?;
                    }
                    5 => {
                        writeln!(term, "Bye!")?;
                        exit_app = true;
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
