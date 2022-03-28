use std::sync::{Arc, atomic::AtomicBool};

use futures::lock::Mutex;
use hyper::{Request, Body, Response, StatusCode};

use crate::{utils::types::{self, AppConfig, TokenClaims}, database::dbs, webserver};


pub async fn setup(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
    app_config: &types::AppConfig,
    is_set_up: &Arc<AtomicBool>,
) -> types::Result<Response<Body>> {

    
    println!("BugTracker was already set up? {:?}", is_set_up);
    if is_set_up.load(std::sync::atomic::Ordering::Relaxed) {
        println!("The system has already been set up, thus we reject the api call /api/setup");
        Ok(webserver::utils::status_response(StatusCode::OK))
    } else {
        let body = hyper::body::to_bytes(req).await?;
        let user: Option<types::User> = match serde_json::from_slice(&body) {
            Ok(p) => Some(p),
            Err(e) => {
                eprintln!("{:?}", e);
                None
            }
        };

        if user.is_some() {
            let user = user.unwrap();
            // we have to check beforehand if the data is valid, i.e. password is longer than x characters, other data is not empty etc.

            // here we set up to basic teams which are admins and others where others are users not yet classified to a team

            println!("Setting up user {:?} as admin", user);
            match webserver::utils::hash_pw(&app_config.secrets.pw_salt, &app_config.secrets.pw_secret, &user.password) {
                Some(hashed_pw) => {   
                    let sql = "INSERT INTO users (username, email, password, name, team, rights) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";
        
                    let conn = sql_connection.lock().await;
                    match conn.execute(sql, rusqlite::params!(user.username, user.email, hashed_pw, user.name, user.team, types::UserRights::ADMIN as u8)) {
                        Ok(_) => (),
                        Err(e) => eprintln!("{:?}", e), 
                    };
                    drop(conn);
                    
                    // since the setup of an admin account worked properly, we will never allow this request again 
                    // when the server is restarted we just check the tables for at least one admin account to make sure we are set up
                    is_set_up.store(true, std::sync::atomic::Ordering::Relaxed);
                    Ok(webserver::utils::status_response(StatusCode::OK))
                },
                None => {
                    println!("We could not hash the password, thus we have to reject the api call /api/setup");
                    Ok(webserver::utils::status_response(StatusCode::INTERNAL_SERVER_ERROR))
                }
            } 
        } else {
            Ok(webserver::utils::status_response(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}


pub async fn login(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
    app_config: &AppConfig,
) -> types::Result<Response<Body>> {
    
    let body = hyper::body::to_bytes(req).await?;
    
    #[derive(serde::Deserialize, Debug)]
    struct Login {
        username: String,
        password: String,
    }
    let login_data: Login = serde_json::from_slice(&body)?;
    let sql = "SELECT id, username, email, password, name, team, rights FROM users WHERE email=(?1) or username=(?1)";

    let time_now = std::time::SystemTime::now();
    let expires = std::time::Duration::new(60 * 60, 0);
    let expires_utc = time_now.checked_add(expires);
    let iat: usize = match time_now.duration_since(std::time::UNIX_EPOCH) {
        Ok(n) => n.as_secs() as usize,
        Err(_) => 0,    // could be anything really
    }; 
    let sql_conn = sql_connection.lock().await;
    let user_vec = dbs::get_from_table::<types::User>(&sql_conn, sql, rusqlite::params![login_data.username]);
    drop(sql_conn);

    match user_vec {
        Ok(p) => {
            let user = p.first();
            if user.is_none() {
                eprintln!("The user could not be found in table: {:?}", login_data);
                return Ok(webserver::utils::status_response(StatusCode::UNAUTHORIZED))
            }
            let user = user.unwrap();
            let login_works = webserver::utils::verify_pw(&user.password, &login_data.password);

            if login_works {
                let user_claims = types::TokenClaims {
                    sub: user.name.clone(),
                    iat: iat,
                    exp: expires.as_secs() as usize,
                    rights: user.rights as u64,
                };
                let token = webserver::utils::generate_jwt_token(&app_config.secrets.jwt_secret, user_claims);
        
                if token.is_some() {
                    let token = token.unwrap();
                    // send back the token and reroute to main page
                    // reroute user to mainpage
                    let expires_str = if expires_utc.is_some() {
                        format!(" Expires={:?};", expires_utc.unwrap())
                    } else {
                        "".to_string()
                    };

                    let response = Response::builder()
                            .status(StatusCode::OK)
                            .header(hyper::header::SET_COOKIE, format!("auth-token={};{} Secure; HttpOnly", token, expires_str))
                            .body(Body::empty())?;
            
                    return Ok(response) 
                }
            }
        },
        Err(e) => {
            eprintln!("{:?}", e);
            return Ok(webserver::utils::status_response(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }

    Ok(webserver::utils::status_response(StatusCode::UNAUTHORIZED))
}


pub async fn new_issue(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
    app_config: &AppConfig,
    claims: &Option<TokenClaims>,
) -> types::Result<Response<Body>> {
    
    let body = hyper::body::to_bytes(req).await?;
    let item: types::BugItem = serde_json::from_slice(&body)?;

    let sql = "INSERT (title, project, assignee, severity, effort, description, status) INTO issues VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";

    let conn = sql_connection.lock().await;
    let response = match conn.execute(sql, 
        rusqlite::params!(item.title, item.project, item.assignee, item.severity, item.effort, item.description, item.status)) {
        Ok(_) => {
            Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("New issue successfully created."))?
        },
        Err(e) => {
            eprintln!("{:?}", e);
            Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())?
        }
    };
    drop(conn);

    Ok(response)
}


pub async fn update_issue(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
    app_config: &AppConfig,
    claims: &Option<TokenClaims>,
) -> types::Result<Response<Body>> {
    
    let body = hyper::body::to_bytes(req).await?;
    let item: types::BugItem = serde_json::from_slice(&body)?;

    let sql = "UPDATE (title, project, assignee, severity, effort, description, status) INTO issues VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)";

    let conn = sql_connection.lock().await;
    let response = match conn.execute(sql, 
        rusqlite::params!(item.title, item.project, item.assignee, item.severity, item.effort, item.description, item.status)) {
        Ok(_) => {
            Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("Issue successfully updated."))?
        },
        Err(e) => {
            eprintln!("{:?}", e);
            Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())?
        }
    };
    drop(conn);

    Ok(response)
}

pub async fn new_project(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
    app_config: &AppConfig,
    claims: &Option<TokenClaims>,
) -> types::Result<Response<Body>> {
    
    let body = hyper::body::to_bytes(req).await?;
    let project: types::Project = serde_json::from_slice(&body)?;

    let sql = "INSERT (title, team, description) INTO projects VALUES (?1, ?2, ?3)";

    let conn = sql_connection.lock().await;
    let response = match conn.execute(sql, 
        rusqlite::params!(project.title, project.team, project.description)) {
        Ok(_) => {
            Response::builder()
            .status(StatusCode::OK)
            .body(Body::from("New project successfully created."))?
        },
        Err(e) => {
            eprintln!("{:?}", e);
            Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())?
        }
    };
    drop(conn);

    Ok(response)
}


pub async fn new_user(
    req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
    app_config: &AppConfig,
    claims: &Option<TokenClaims>,
) -> types::Result<Response<Body>> {
    
    let body = hyper::body::to_bytes(req).await?;
    let user: types::User = serde_json::from_slice(&body)?;

    // first we need to check if the request is even allowed, i.e. we have to check the jwt token and the user has to have admin rights


    // we have to hash the password beforehand, we dont want to save plaintext passwords
    match webserver::utils::hash_pw(&app_config.secrets.pw_salt, &app_config.secrets.pw_secret, &user.password) {
        Some(hashed_pw) => {   
            let sql = "INSERT INTO users (username, email, password, name, team, rights) VALUES (?1, ?2, ?3, ?4, ?5, ?6)";

            let conn = sql_connection.lock().await;
            match conn.execute(sql, rusqlite::params!(user.username, user.email, hashed_pw, user.name, user.team, user.rights)) {
                Ok(_) => (),
                Err(e) => eprintln!("{:?}", e), 
            };
            drop(conn);
            

            Ok(webserver::utils::status_response(StatusCode::OK))
        },
        None => {
            println!("We could not hash the password, thus we have to reject the api call /api/add/user");
            Ok(webserver::utils::status_response(StatusCode::INTERNAL_SERVER_ERROR))
        }
    }
}
