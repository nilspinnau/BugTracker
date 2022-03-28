use std::convert::Infallible;
use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use futures::lock::Mutex;
use hyper::{Body, Method, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::server::conn::AddrStream;
use hyper::body::HttpBody;

use crate::database::dbs;
use crate::utils::types::{Result, AppConfig};

use crate::webserver::routes;

const MAX_ALLOWED_REQUEST_SIZE: u64 = 1024;

async fn handle(
    _addr: SocketAddr,
    mut req: Request<Body>,
    sql_connection: Arc<Mutex<rusqlite::Connection>>,
    app_config: AppConfig,
    is_set_up: Arc<AtomicBool>,
) -> Result<Response<Body>> 
{

    let request_content_length = match req.body().size_hint().upper() {
        Some(v) => v,
        None => MAX_ALLOWED_REQUEST_SIZE + 1 // Just to protect ourselves from a malicious response
    };

    if request_content_length < MAX_ALLOWED_REQUEST_SIZE {
        // here we match every route that might be coming from the web client
        match req.method() {
            &Method::GET => {
                let req_queries = req.uri().query();
                // we could do this in each method, but makes it look more complicated
                let _query_dict = super::utils::request_queries_to_dict(req_queries);

                // some regex to serve static files

                match req.uri().path() {
                    "/" => routes::routes::route_get_main(req, sql_connection, &app_config).await,
                    "/login" => routes::routes::route_get_main(req, sql_connection, &app_config).await,
                    #[cfg(not)]
                    "/admin" => {
                        // admin page, maybe we should have two tokens to validate the user is an admin
                        routes::routes::route_get_admin(req, sql_connection, query_dict).await
                    },
                    #[cfg(not)]
                    "/projects" => {
                        routes::routes::route_get_projects(req, sql_connection, query_dict).await
                    },
                    #[cfg(not)]
                    "/newsfeed" => {
                        routes::routes::route_get_newsfeed(req, sql_connection, query_dict).await
                    },
                    #[cfg(not)]
                    "/project" => {
                        routes::routes::route_get_project(req, sql_connection, query_dict).await
                    },
                    #[cfg(not)]
                    "/item" => {
                        routes::routes::route_get_item(req, sql_connection, query_dict).await
                    },
                    "/user" => {
                        routes::routes::route_get_user(req, sql_connection).await
                    },
                    #[cfg(not)]
                    "/team" => {
                        routes::routes::route_get_team(req, sql_connection).await
                    },
                    "/user_ranking" => routes::routes::route_get_user_ranking(req, sql_connection).await,
                    "/setup" => {
                        println!("BugTracker was already set up? {:?}", is_set_up);
                        if !is_set_up.load(std::sync::atomic::Ordering::Relaxed) {
                            routes::routes::route_get_main(req, sql_connection, &app_config).await
                        } else {
                            // we build the form on the client side and just make sure that the setup api POST call is only accepted once
                            Ok(super::utils::status_response(hyper::StatusCode::FORBIDDEN))
                        }
                    },
                    _ => {
                        println!("Checking if a static file is required. Otherwise we return 404 to client.");
                        // we check for static files, if they match we return the files, if not we return 404
                        super::utils::check_static_files(req, &app_config)
                    },
                }
            },
            &Method::POST => {
                // we have to validate that the user is legit, we do not want to violate integrity
                // check jwt token, user must be identified
                let mut claims = None;
                if req.uri().path() != "/api/auth/login" && req.uri().path() != "/api/setup" {
                    if let Some(cookies) = super::utils::get_cookies(req.headers()) {
                        let token = cookies.get("auth_token");
                        println!("We got a cookie");
                        claims = super::utils::validate_jwt_token(token, &app_config.secrets.jwt_secret);
                    } else {
                        println!("We did not get a cookie");
                        return Ok(super::utils::status_response(hyper::StatusCode::UNAUTHORIZED))
                    }
                } 
                // our api
                match req.uri().path() {
                    "/api/auth/login" => routes::api::login(req, sql_connection, &app_config).await,
                    "/api/auth/logout" => {
                        match req.headers_mut().remove("auth-token") {
                            Some(_) => Ok(super::utils::status_response(hyper::StatusCode::OK)),
                            _ => Ok(super::utils::status_response(hyper::StatusCode::OK))
                        }
                    }, // we delete the cookie
                    "/api/setup" => routes::api::setup(req, sql_connection, &app_config, &is_set_up).await,
                    #[cfg(not)]
                    "/api/auth/register" => routes::api::register(req, sql_connection).await,
                    "/api/add/item" => routes::api::new_issue(req, sql_connection, &app_config, &claims).await,
                    "/api/add/user" => routes::api::new_user(req, sql_connection, &app_config, &claims).await, // only admins
                    "/api/add/project" => routes::api::new_project(req, sql_connection, &app_config, &claims).await, // this can be done by teamleads or higher
                    "/api/update/item" => routes::api::update_issue(req, sql_connection, &app_config, &claims).await,
                    "/api/update/user" => routes::api::new_user(req, sql_connection, &app_config, &claims).await, // only admins
                    _ => Ok(super::utils::status_response(hyper::StatusCode::NOT_FOUND)),
                }
            },
            _ => Ok(super::utils::status_response(hyper::StatusCode::NOT_FOUND)),
        }
    } else {
        Ok(super::utils::status_response(hyper::StatusCode::NOT_FOUND))
    }
}

#[tokio::main]
pub async fn run_webserver(
    app_config: AppConfig,
    sql_connection: rusqlite::Connection,
) -> () 
{
    let web_url = app_config.general.web_url.clone();
    
    // we need this later to check if we allow to generate an admin in /setup route
    let is_set_up = match dbs::check_system_already_set_up(&sql_connection) {
        Ok(p) => {
            println!("The system is already setup. No setup allowed anymore, if required delete database file.");
            p
        },
        Err(e) => {
            eprintln!("{:?}", e);
            false
        },
    };
    let is_set_up = Arc::new(AtomicBool::new(is_set_up));

    let sql_mutex_arc = Arc::new(Mutex::new(sql_connection));

    // A `MakeService` that produces a `Service` to handle each connection.
    let make_service = make_service_fn(move |conn: &AddrStream| {

        // You can grab the address of the incoming connection like so.
        let addr = conn.remote_addr();

        // Create a `Service` for responding to the request.
        let sma_clone = sql_mutex_arc.clone();
        let is_set_up_clone = is_set_up.clone();
        let config_clone = app_config.clone();
        let service = {
            service_fn(move |req| {
                handle(addr, req, sma_clone.to_owned(), config_clone.to_owned(), is_set_up_clone.to_owned())
            })
        };

        // Return the service to hyper.
        async move { Ok::<_, Infallible>(service) }
    });

    // Run the server like above...
    let addr = SocketAddr::from_str(&web_url).unwrap();

    let server = Server::bind(&addr)
        .http1_header_read_timeout(std::time::Duration::from_secs(1))
        .http2_enable_connect_protocol()
        .serve(make_service);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
