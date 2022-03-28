/*

This file includes all calls that can be made to the database.
Used only in the dbs_handler.rs

*/

use rusqlite::{Connection, Result};
use serde::Serialize;

use crate::utils::types;
use types::DbsParse;

// in the future another sql system should definitly be used since sqllite is not really suited for user data
// however we use it here just to demonstrate and its simplicity
pub fn setup_sqllite(path: &str) -> Result<rusqlite::Connection, rusqlite::Error> {
   
    let conn = match path.is_empty() {
        true => rusqlite::Connection::open_in_memory()?,
        _ => rusqlite::Connection::open(path)?,
    }; 

    // we still create_tables since some might not already exist (case if the database got setup beforehand)
    create_tables_bugtracker(&conn)?;
    Ok(conn)
}


pub fn check_system_already_set_up(conn: &Connection) -> Result<bool> {

    let query = "SELECT id, rights FROM users WHERE rights=(?1)";

    #[derive(Debug)]
    struct Check {
        id: u64,
        rights: u8
    } 

    let mut stmt = conn.prepare(query)?;
    // we get all admins here, we need at least one tho
    let rows = stmt.query_map([types::UserRights::ADMIN as u8], |row| {
        Ok(Check {
            id: row.get(0).unwrap(),
            rights: row.get(1).unwrap()
        })
    })?;

    let mut admins = Vec::new();
    for check_res in rows {
        let p = check_res?;
        admins.push(p);
    }

    Ok(admins.len() > 0)
}

fn create_tables_bugtracker(conn: &Connection) -> Result<(), rusqlite::Error> {

    conn.execute(
        "CREATE TABLE IF NOT EXISTS teams (
                id              INTEGER PRIMARY KEY,
                name            TEXT
                description     TEXT
                )",
        [],
    )?;
    println!("Created table teams if it did not exist");
    // we could also set up here the first teams, admins (id:0) and others (id:1)

    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
                id                  INTEGER PRIMARY KEY,
                username            TEXT NOT NULL,
                email               TEXT NOT NULL,
                password            TEXT NOT NULL,    
                name                TEXT NOT NULL,
                team                INTEGER,
                rights              INTEGER NOT NULL,       
                FOREIGN KEY(team) REFERENCES team(id)
                )",
        [],
    )?;
    println!("Created table users if it did not exist");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS projects (
                id              INTEGER PRIMARY KEY,
                name            TEXT,
                description     TEXT,
                team            INTEGER,
                FOREIGN KEY(team) REFERENCES team(id)
            )",
        [],
    )?;
    println!("Created table projects if it did not exist");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS issues (
                id                  INTEGER PRIMARY KEY,
                project             INTEGER,
                description         TEXT,
                status              INTEGER,
                severity            INTEGER,
                assignee            INTEGER,
                workload            INTEGER,
                FOREIGN KEY(project) REFERENCES projects(id)
                FOREIGN KEY(assignee) REFERENCES users(id)
            )",
        [],
    )?;
    println!("Created table issues if it did not exist");

    Ok(())
}


// FUNCTIONS


pub fn into_table<T>(
    sql_connection: &rusqlite::Connection,
    sql: &str,
    params: &[&dyn rusqlite::ToSql],
) -> Result<usize> 
where T: DbsParse + Serialize
{
    sql_connection.execute(sql, params)?;

    Ok(0)
}

// generic function to query data from a table, converted to generic type T with traits Serialize and DbsParse
// returns a hyper::Response
pub fn get_from_table<T>(
    sql_connection: &rusqlite::Connection,
    sql: &str,
    params: &[&dyn rusqlite::ToSql],
) -> Result<Vec<T>> 
where T: DbsParse + Serialize
{

    let mut stmt = sql_connection.prepare(sql)?;
    let t_iter = stmt.query_map(params, |row| {
        Ok(T::from_row(row))
    })?;

    let mut t_vec: Vec<T> = vec![];
    for t_option in t_iter {
        match t_option {
            Ok(t) => t_vec.push(t),
            _ => continue
        }
    }

    Ok(t_vec)
}
