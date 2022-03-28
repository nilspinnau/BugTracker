mod database;
mod webserver;
mod utils;
mod chat;
// mod dataprocessor;

use webserver::webserver_handler;

use chat::chat_handler;

extern crate clap;
extern crate argon2;

use std::{thread, sync::atomic::AtomicBool};
use std::thread::JoinHandle;

use clap::{Arg, Command};

use toml;
use utils::types::AppConfig;


// we return an Appconfig always, might be dangerous but thats why we print errors
fn read_config(path: &str) -> AppConfig {
    match std::fs::read_to_string(path) {
        Ok(p) => {
            match toml::from_str(&p) {
                Ok(p) => p,
                Err(e) => {
                    panic!("Error occurred, maybe check config again: {:?}", e);
                }
            }
        },
        Err(e) => {
            panic!("Error occurred, maybe check config again: {:?}", e);
        },
    }
}

fn main() {

    // handle command line arguments
    let args = Command::new("BugTracker")
        .version("0.1.0")
        .author("Nils Pinnau")
        .about("")
        .arg(Arg::new("config")
            .long("config")
            .short('c')
            .help("Path to config file including all relevant information. Should be a .toml file")
            .default_value("config/default_config.toml"))
        .get_matches();
    println!("{:?}", args);


    let config: AppConfig = read_config(args.value_of("config").unwrap());

    // create joinhandle vector and pipes
    let mut threads: Vec<JoinHandle<()>> = Vec::new();


    // create tables/database
    println!("Setting up sqllite3 database at '{}'", config.general.dbs_path);
    let sqllite_connection = database::dbs::setup_sqllite(&config.general.dbs_path).unwrap();

    // start web server
    println!("Starting thread for web server at '{}'", config.general.web_url);
    let web_server_thread = thread::Builder::new().name("web_server_thread".into()).spawn(move || {
        webserver_handler::run_webserver(config, sqllite_connection);
    }).unwrap();

    let chat_thread = thread::Builder::new().name("chat_thread".into()).spawn(move || {
        //chat_handler::run_chat_handler();
    }).unwrap();

    /*
        // Maybe have an API here? this is a dataprocessing tool which collects data on users maybe? and then via API we can receive this data for BI processing
        let dataprocessor = None;
        thread::spawn(move || {
            dataprocessor::run_dataprocessor(dataprocessor);
        });
    */

    threads.push(web_server_thread);
    threads.push(chat_thread);

    for thread in threads {
        thread.join().unwrap();
    }

    println!("Exiting BugTracker");
}