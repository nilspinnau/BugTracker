use rusqlite::{Row};

use serde::{Deserialize, Serialize};


// types

// as suggested by hyper.rs
pub type GenericError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, GenericError>;

// CONSTANTS


// TRAITS
pub trait DbsParse {
    fn from_row(row: &Row) -> Self;
}


// ENUMS
pub enum Code {
    NOTFOUND,
    ACCEPT,
    DENY,
    TODO
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BugStatus {
    OPEN,
    PROGRESS,
    REVIEW,
    CLOSED,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum BugSeverity {
    LOW,
    MEDIUM,
    HIGH,
    IMMEDIATE
}

#[derive(Debug, Clone, Copy)]
pub enum UserRights {
    VIEWER = 1,
    DEVELOPER = 2,
    TEAMLEAD = 3,
    ADMIN = 4,
}


// STRUCTS

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub general: GeneralConfig,
    pub secrets: SecretConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub web_url: String,
    pub dbs_path: String,
    pub static_file_root: String,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretConfig {
    pub jwt_secret: String,
    pub pw_salt: String,
    pub pw_secret: String, 
}

impl Default for AppConfig {
    fn default() -> Self {
        AppConfig { 
            general: GeneralConfig { web_url: "127.0.0.1:8080".to_string(), dbs_path: "".to_string(), static_file_root: "frontend/build".to_string() },
            secrets: SecretConfig { jwt_secret: "aiowdzWdlawJBNowuaZTDdwjOPAlWAJK".to_string(), pw_secret: "iwODHlkgjEPOIWlkdjaJIPOpjOHNDNca".to_string(), pw_salt: "jdoiWazqdPODUWAPmjdowadupWNWADpi".to_string() }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TokenClaims {
    pub sub: String,        // this will be the user's username which is UNIQUE
    pub rights: u64,        // is the subject an admin??
    pub iat: usize,
    pub exp: usize,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BugItem {
    pub id: u64,
    pub title: String,
    pub project: u64,
    pub assignee: u64,  // user id who is assigned to the task, convert for web to username 
    pub severity: isize,   // possible stati: low 0, medium 1, high 2, immediate 3
    pub effort: u8,  // how many days are required, 0-8
    pub description: String,
    pub status: isize,     // possible stati: open (0), in-progress (1), done (2)
}
impl DbsParse for BugItem {
    fn from_row(row: &Row) -> Self {
        BugItem {
            id: row.get(0).unwrap(),
            title: row.get(1).unwrap(),
            project: row.get(2).unwrap(),
            assignee: row.get(3).unwrap(),  
            severity: row.get(4).unwrap(),   
            effort: row.get(5).unwrap(),  
            description: row.get(6).unwrap(),
            status: row.get(7).unwrap(),   
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: u64,
    pub title: String,
    pub team: u64,
    pub description: String,
}
impl DbsParse for Project {
    fn from_row(row: &Row) -> Self {
        Project {
            id: row.get(0).unwrap(),
            title: row.get(1).unwrap(),
            team: row.get(2).unwrap(),
            description: row.get(3).unwrap(),
        }
    }
}




#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: u64,
    pub title: String,
    pub description: String,
}
impl DbsParse for Team {
    fn from_row(row: &Row) -> Self {
        Team {
            id: row.get(0).unwrap(),
            title: row.get(1).unwrap(),
            description: row.get(2).unwrap(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub email: String,
    pub password: String,
    pub name: String,
    pub team: u64,
    pub rights: u8,        // 0 for external user (visibility rights for selected projects), 1 for developer (limited rights, creation), 2 for admin (all rights)
}
impl User {
    pub fn new(id: u64, username: &str, name: &str, email: &str, password: &str, team: u64, rights: u8) -> User {
        Self {
            id: id,
            username: username.to_string(),
            email: email.to_string(),
            password: password.to_string(),
            name: name.to_string(),
            team: team,
            rights: rights,
        }
    }

    pub fn empty() -> User {
        User::new(0, "", "", "", "", 0, 0)
    }
}
impl DbsParse for User {
    // maybe we should do it more secure since this can panic which is not exactly what we want 
    fn from_row(row: &Row) -> Self {
        User {
            id: row.get(0).unwrap(),
            username: row.get(1).unwrap(),
            email: row.get(2).unwrap(),
            password: row.get(3).unwrap(),
            name: row.get(4).unwrap(),
            team: row.get(5).unwrap(),
            rights: row.get(6).unwrap(),
        }
    }
}

pub struct NoneType {}

impl DbsParse for NoneType {
    fn from_row(row: &Row) -> Self {
        NoneType {  }
    }
}
