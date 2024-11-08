use crate::db::get_connection;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Local;
use rusqlite::{params, OptionalExtension};
use uuid::Uuid;

pub struct Task {
    id: i32,
    taskownerid: String,
    title: String,
    content: String,
    created_at: i64,
    done: bool,
}

#[derive(Debug)]
pub struct User {
    userid: String,
    username: String,
    passhash: String,
    created_at: i64,
    need_password_change: bool,
}

impl User {
    pub fn create(username: String, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let passhash = Password::hash(password).map_err(|_| "Passwort-Hashing fehlgeschlagen")?;
        let userid = Uuid::new_v4().to_string();
        let created_at = Local::now().timestamp();
        let need_password_change = true;
        let conn = get_connection().map_err(|_| "Datenbankverbindung fehlgeschlagen")?;
        let mut stmt = conn.prepare("INSERT INTO users (uuid, username, passhash, created_at, need_password_change) VALUES (?1, ?2, ?3, ?4, ?5)")
            .map_err(|_| "Fehler beim Vorbereiten der Datenbankabfrage")?;
        if stmt.execute(params![
            userid,
            username,
            passhash,
            created_at,
            need_password_change
        ])? == 1
        {
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub fn login(username: &str, password: &str) -> Result<bool, String> {
        let conn = get_connection().map_err(|_| "Datenbankverbindung fehlgeschlagen")?;
        let mut stmt = conn
            .prepare("SELECT uuid, username, passhash FROM users WHERE username = ?1")
            .map_err(|_| "Fehler beim Vorbereiten der Datenbankabfrage")?;

        let user: Option<User> = stmt
            .query_row(params![username], |row| {
                Ok(User {
                    userid: row.get(0)?,
                    username: row.get(1)?,
                    passhash: row.get(2)?,
                    created_at: 0,
                    need_password_change: false,
                })
            })
            .optional()
            .map_err(|_| "Fehler beim Abrufen des Benutzers")?;

        match user {
            Some(user) => {
                if user.verify_password(password) {
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            None => Ok(false),
        }
    }

    pub fn verify_password(&self, password: &str) -> bool {
        Password::verify(password, &self.passhash)
    }
}

pub struct Password;

impl Password {
    pub fn hash(password: &str) -> Result<String, bcrypt::BcryptError> {
        hash(password, DEFAULT_COST)
    }

    pub fn verify(password: &str, hash: &str) -> bool {
        verify(password, hash).unwrap_or(false)
    }
}

impl Task {
    pub fn new(taskownerid: String, title: String, content: String, done: bool) -> Task {
        let created_at = Local::now().timestamp();
        Task {
            id: 0,
            taskownerid,
            title,
            content,
            created_at,
            done,
        }
    }
}
