use std::sync::Mutex;
use rusqlite::Connection;
use once_cell::sync::Lazy;

static DB_CONN: Lazy<Mutex<Connection>> = Lazy::new(|| {
    let conn = Connection::open("tasks.db").expect("Failed to connect to database");
    Mutex::new(conn)
});

pub fn get_connection() -> rusqlite::Result<std::sync::MutexGuard<'static, Connection>> {
    DB_CONN.lock().map_err(|e| rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(1), Some(format!("Failed to get connection: {}", e))))
}


pub fn init_db() -> rusqlite::Result<()> {
    let conn = get_connection()?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            done BOOLEAN NOT NULL
        )",
        [],
    )?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            uuid TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            passhash TEXT NOT NULL
        )",
        [],
    )?;

    Ok(())
}


/*
pub fn init_db() -> rusqlite::Result<()> {
    let conn = get_connection()?;
    println!("Creating Databasetable: Tasks");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS tasks (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            done BOOLEAN NOT NULL
        )",
        [],
    )?;
    println!("Database table 'Tasks' created successfully");

    print!("Creating Databasetable: Users");
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            uuid TEXT PRIMARY KEY,
            username TEXT NOT NULL,
            passhash TEXT NOT NULL
        )",
        [],
    )?;
    println!("Database table 'Users' created successfully");

    Ok(())
}
 */