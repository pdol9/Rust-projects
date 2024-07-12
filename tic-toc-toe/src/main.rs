use std::io;
use console::{Key, Term};

fn main() -> io::Result<()> {
    let mut tic_grid = vec![' '; 9];
    let term = Term::stdout();
    let mut is_x_turn = true;

    print_grid(&tic_grid, &term, Some(""))?;

    loop {
        term.write_line("Press a number: 1 - 9")?;
        if is_x_turn {
            term.write_line("--- Player 1 ---")?;
        } else {
            term.write_line("-*- Player 2 -*- ")?;
        }

        let key = term.read_key()?;
        if key == Key::Escape {
            term.write_line("Quiting the game!")?;
            break;
        } else if let Key::Char(c) = key{
            if let Some(num) = c.to_digit(10) {
                if num >= 1 && num <= 9 {
                    let idx = (num - 1) as usize;
                    if tic_grid[idx] != ' ' {
                        print_grid(&tic_grid, &term, Some("This Move Was Already Played !"))?;
                        continue;
                    } else {
                        tic_grid[idx] = if is_x_turn { 'X' } else { 'O' };
                        is_x_turn = !is_x_turn;
                        // check for winner
                        if let Some(winner) = check_grid(&tic_grid) {
                            print_grid(&tic_grid, &term, Some(""))?;
                            let winning_player = if winner == 'X' { "Player 1" } else { "Player 2" };
                            term.write_line(&format!("{} wins!", winning_player))?;
                            break; // play again??
                        }
                    }
                }
            }
        }
        print_grid(&tic_grid, &term, Some(""))?;
    }
    Ok(())
}

fn check_grid(grid: &[char]) -> Option<char> {
    // Check rows
    for i in 0..3 {
        if grid[i * 3] != ' ' && grid[i * 3] == grid[i * 3 + 1] && grid[i * 3] == grid[i * 3 + 2] {
            return Some(grid[i * 3]);
        }
    }

    // Check columns
    for i in 0..3 {
        if grid[i] != ' ' && grid[i] == grid[i + 3] && grid[i] == grid[i + 6] {
            return Some(grid[i]);
        }
    }

    // Check diagonals
    if grid[0] != ' ' && grid[0] == grid[4] && grid[0] == grid[8] {
        return Some(grid[0]);
    }
    if grid[2] != ' ' && grid[2] == grid[4] && grid[2] == grid[6] {
        return Some(grid[2]);
    }

    None
}

fn print_grid(grid: &[char], term: &Term, msg: Option<&str>) -> io::Result<()> {

    term.clear_screen()?;
    term.write_line("Tic - Toc - Toe Game \n")?;
    term.write_line(&format!(" {} | {} | {}", grid[0],grid[1],grid[2]))?;
    term.write_line("-----------")?;
    term.write_line(&format!(" {} | {} | {}", grid[3],grid[4],grid[5]))?;
    term.write_line("-----------")?;
    term.write_line(&format!(" {} | {} | {}", grid[6],grid[7],grid[8]))?;
    term.write_line(msg.unwrap())?;
    Ok(())
}
