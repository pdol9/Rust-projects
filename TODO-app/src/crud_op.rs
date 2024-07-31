// CRUD Operations

use crate::structs::{List, Task};

use rusqlite::{Connection, Result};

pub fn fetch_lists(conn: &Connection) -> Result<Vec<List>> {
    let mut stmt = conn.prepare("SELECT id, list_name, summary, category FROM list")?;
    let list_iter = stmt.query_map([], |row| {
        Ok(List {
            id: row.get(0)?,
            list_name: row.get(1)?,
            summary: row.get(2)?,
            category: row.get(3)?,
        })
    })?;

    let mut lists = Vec::new();
    for list in list_iter {
        lists.push(list?);
    }
    Ok(lists)
}

pub fn fetch_tasks(conn: &Connection, list_name: &str) -> Result<Vec<Task>> {
    let mut stmt = conn.prepare("SELECT id, task_name, list_id, list_name, priority, status, tags, deadline, completed_on, description FROM task WHERE list_id = ?1")?;
    let task_iter = stmt.query_map([list_name], |row| {
        let tags: Option<String> = row.get(3)?;
        Ok(Task {
            id: row.get(0)?,
            task_name: row.get(1)?,
            list_id: row.get(2)?,
            list_name: row.get(3)?,
            priority: row.get::<usize, Option<i32>>(4)?.map(|p| p.into()),
            status: row.get::<usize, Option<i32>>(5)?.map(|s| s.into()),
            tags: tags.map(|t| t.split(',').map(|s| s.to_string()).collect()),
            deadline: row.get(7)?,
            completed_on: row.get(8)?,
            description: row.get(9)?,
        })
    })?;

    let mut tasks = Vec::new();
    for task in task_iter {
        tasks.push(task?);
    }
    Ok(tasks)
}

pub fn insert_list(conn: &Connection, list: &List) -> Result<()> {
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM list WHERE list_name = ?1")?;
    let count: i32 = stmt.query_row([&list.list_name], |row| row.get(0))?;

    if count > 0 {
        println!(
            "List with name '{}' already exists. Skipping insertion.",
            &list.list_name
        );
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
        "INSERT INTO task (task_name, list_id, list_name, priority, status, tags, deadline, completed_on, description) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        (
            &task.task_name,
            task.list_id,
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

// Table Creation

pub fn create_list_table(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS list (
            id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
            list_name TEXT NOT NULL,
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
            list_id INTEGER NOT NULL,
            list_name TEXT NOT NULL,
            priority INTEGER,
            status INTEGER,
            tags TEXT,
            deadline DATE,
            completed_on DATE,
            description TEXT,
            FOREIGN KEY(list_id) REFERENCES list(id)        )",
        [],
    )?;
    Ok(())
}
