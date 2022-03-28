use std::collections::HashMap;
/*
Utils for the webserver
*/
use hyper::{Request, Body, Response, StatusCode, http::response::Builder};


use crate::utils::types;

use argon2;


// we want to protect this recursive function from doing to much iterations with iter and depth
// however requires proper usage ofc
fn walk_dir(dir: &std::path::Path, iter: u64, depth: u64) -> std::io::Result<Vec<std::path::PathBuf>> {
    let mut files = vec![];
    if iter >= depth {
        return Ok(files)
    } else {
        if dir.is_dir() {
            for entry in std::fs::read_dir(dir)? {
    
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    files.append(&mut walk_dir(&path, iter+1, depth)?);
                } else {
                    files.push(path);
                }
            }
        }    
    }
    Ok(files)
}

pub fn check_static_files(req: Request<Body>, app_config: &types::AppConfig) -> types::Result<Response<Body>> {

    // the path starts with '/', we have to remove that otherwise the check fails
    let mut path = req.uri().path().chars();
    path.next();
    let path = path.as_str();
    let root_dir = app_config.general.static_file_root.clone();

    // could make it more efficient by not finding all files with walk_dir, but until we found the file
    match walk_dir(std::path::Path::new(&root_dir), 0, 10) {
        Ok(files) => {
            for file in files {
                if file.ends_with(path) {
                    let file_content = std::fs::read_to_string(file)?;

                    return Ok(Response::builder()
                        .status(StatusCode::OK)
                        .body(Body::from(file_content))?)
                }

                ()
            }
        },
        _ => () 
    }    
    
    Ok(status_response(StatusCode::NOT_FOUND))
}


pub fn request_queries_to_dict(req_queries: Option<&str>) -> HashMap<String, String> {
    let mut query_dict: HashMap<String, String> = HashMap::new();
    match req_queries {
        None => query_dict,
        Some(queries) => {
            let key_value_pairs: Vec<&str> = queries.split("&").collect();
            for k_v_pair in key_value_pairs {
                let mut split_k_v = k_v_pair.splitn(2, "=");
                let (key, mut value) = (
                    split_k_v.next().unwrap().to_string(),
                    split_k_v.next().unwrap().to_string(),
                );

                query_dict.insert(key.to_string(), value.to_string());
            }

            query_dict
        }
    }
}


pub fn status_response(code: StatusCode) -> Response<Body> {
    let mut status_response = Response::default();
    *status_response.status_mut() = code;
    status_response
}


pub fn get_cookies(headers: &hyper::HeaderMap) -> Option<HashMap<String, String>> {
    let _ = headers.contains_key(hyper::header::COOKIE) && match headers[hyper::header::COOKIE].to_str() {
        Ok(cookie_list) => {
            let mut cookie_dict = HashMap::new();
            for k_v in cookie_list.split("; ") {
                let mut split_k_v = k_v.splitn(2, "=");
                let (key, value) = (
                    split_k_v.next().unwrap().to_string(),
                    split_k_v.next().unwrap().to_string(),
                );

                cookie_dict.insert(key.to_string(), value.to_string());
            }

            return Some(cookie_dict)
        },
        Err(e) => {
            eprintln!("{:?}", e);
            return None
        }
    };

    None
}

pub fn hash_pw(salt: &str, secret: &str, password: &str) -> Option<String> {
    let mut config = argon2::Config::default();
    // config.secret = secret.as_bytes();
    match argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config) {
        Ok(pw) => Some(pw),
        Err(e) => {
            eprintln!("{:?}", e);
            None
        }
    }
}

pub fn verify_pw(hash: &str, password: &str) -> bool {
    match argon2::verify_encoded(&hash, password.as_bytes()) {
        Ok(pw_matches) => pw_matches,
        Err(e) => {
            eprintln!("{:?}", e);
            false
        }
    }
}

pub fn generate_jwt_token(secret: &str, user_claims: types::TokenClaims) -> Option<String> {
    match jsonwebtoken::encode::<types::TokenClaims>(
            &jsonwebtoken::Header::default(), 
            &user_claims, 
            &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes())) {
        Ok(p) => Some(p),
        Err(e) => {
            eprintln!("{:?}", e);
            None
        }
    }
}

pub fn validate_jwt_token(token: Option<&String>, secret: &str) -> Option<types::TokenClaims> {
    let _ = token.is_some() && match jsonwebtoken::decode::<types::TokenClaims>(
            token.unwrap(), 
            &jsonwebtoken::DecodingKey::from_secret(secret.as_bytes()), 
            &jsonwebtoken::Validation::default()) {
        Ok(p) => return Some(p.claims),
        Err(e) => {
            eprintln!("{:?}", e);
            return None
        }
    };

    None
}


pub fn reponse_with_headers() -> Builder {
    Response::builder()
        .header( "Access-Control-Allow-Origin", "*")
}