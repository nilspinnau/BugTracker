use std::collections::HashMap;
use std::{sync::Arc, fs};

use futures::lock::Mutex;
use hyper::{Request, Response, Body, StatusCode, header};


use crate::database::dbs;
use crate::utils::types::{Team, Project, AppConfig};

use crate::utils::types::Result;

use crate::webserver;


// GET METHODS

pub async fn route_get_main(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
    app_config: &AppConfig,
) -> Result<Response<Body>> {
    
    return Ok(
        webserver::utils::reponse_with_headers()
            .status(StatusCode::OK)
            .body(Body::from(
                fs::read_to_string(format!("{}/index.html", app_config.general.static_file_root)).unwrap(),
    ))?)
}

pub async fn route_get_team(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
    req_dict: HashMap<String, String>,
) -> Result<Response<Body>> { 
    let team_id = &req_dict["id"]; 
    let conn = sql_connection.lock().await;
    let team_vec = dbs::get_from_table::<Team>(&conn, "SELECT * FROM teams WHERE (?1)", rusqlite::params!(team_id));

    match team_vec {
        Ok(team) => {
            if let Ok(json_string) = serde_json::to_string(&team) {
                return Ok(Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_string))?)
            }
        },
        _ => {}
    }

    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())?)
}

pub async fn route_get_projects(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
) -> Result<Response<Body>> {

    let conn = sql_connection.lock().await;
    let projects_vec = dbs::get_from_table::<Project>(&conn, "SELECT * FROM projects", rusqlite::params!());
    match projects_vec {
        Ok(projects) => {
            if let Ok(json_string) = serde_json::to_string(&projects) {
                return Ok(Response::builder()
                .header(header::CONTENT_TYPE, "application/json")
                .status(StatusCode::OK)
                .body(Body::from(json_string))?)
            }
        },
        _ => {}
    }

    Ok(Response::builder()
        .status(StatusCode::INTERNAL_SERVER_ERROR)
        .body(Body::empty())?)
}

pub async fn route_get_user(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
) -> Result<Response<Body>> {
    
    let response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(""))?;

    Ok(response)
}


pub async fn route_get_user_ranking(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
) -> Result<Response<Body>> {
    

    // we have to handle a session, use redis or in-memory sqllite database for this
    // send token to client, which he updates into a session
    Ok(Response::new(Body::from(true.to_string())))
}



// POST METHODS

// HELPERS