// CRUD Operations

use crate::List;
use crate::Task;
use rusqlite::{Connection, Result};

pub fn insert_list(conn: &Connection, list: &List) -> Result<()> {
    // Check if the list already exists
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM list WHERE list_name = ?1")?;
    let count: i32 = stmt.query_row([&list.list_name], |row| row.get(0))?;

    if count > 0 {
        println!("List with name '{}' already exists. Skipping insertion.", &list.list_name);
    } else {
        conn.execute(
            "INSERT INTO list (list_name, summary, category) VALUES (?1, ?2, ?3)",
            (&list.list_name, &list.summary, &list.category),
        )?;
    }
    Ok(())
}

pub fn insert_task(conn: &Connection, task: &Task) -> Result<()> {
    conn.execute(
        "INSERT INTO task (task_name, list_name, priority, status, tags, deadline, completed_on, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (
            &task.task_name,
            &task.list_name,
            task.priority.clone().map(|p| p as i32),
            task.status.clone().map(|s| s as i32),
            &task.tags.clone().map(|t| t.join(",")),
            &task.deadline,
            &task.completed_on,
            &task.description,
        ),
    )?;
    Ok(())
}

pub fn fetch_lists(conn: &Connection) -> Result<Vec<List>> {
    let mut stmt = conn.prepare("SELECT list_name, summary, category FROM list")?;
    let list_iter = stmt.query_map([], |row| {
        Ok(List {
            list_name: row.get(0)?,
            summary: row.get(1)?,
            category: row.get(2)?,
        })
    })?;

    let mut lists = Vec::new();
    for list in list_iter {
        lists.push(list?);
    }
    Ok(lists)
}

pub fn fetch_tasks(conn: &Connection, list_name: &str) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, task_name, list_name, priority, status, tags, deadline, completed_on, description FROM task WHERE list_name = ?1")?;
    let task_iter = stmt.query_map([list_name], |row| {
        let tags: Option<String> = row.get(5)?;
        Ok(Task {
            id: row.get(0)?,
            task_name: row.get(1)?,
            list_name: row.get(2)?,
            priority: row.get::<usize, Option<i32>>(3)?.map(|p| p.into()),
            status: row.get::<usize, Option<i32>>(4)?.map(|s| s.into()),
            tags: tags.map(|t| t.split(',').map(|s| s.to_string()).collect()),
            deadline: row.get(6)?,
            completed_on: row.get(7)?,
            description: row.get(8)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

// Table Creation

pub fn create_list_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS list (
            list_name TEXT PRIMARY KEY,
            summary TEXT,
            category TEXT
        )",
        [],
    )?;
    Ok(())
}

pub fn create_task_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS task (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            task_name TEXT NOT NULL,
            list_name TEXT NOT NULL,
            priority INTEGER,
            status INTEGER,
            tags TEXT,
            deadline DATE,
            completed_on DATE,
            description TEXT,
            FOREIGN KEY(list_name) REFERENCES list(list_name)
        )",
        [],
    )?;
    Ok(())
}
