// The server will be responsible for starting the game and ticking forward
// The frontend will send a list of actions to take
// The server will apply the changes, save the gamestate, and return - what?  changes?  the whole game?
// For now, just resend the whole gamestate

extern crate actix;
extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate futures;
#[macro_use]
extern crate log;
extern crate pretty_env_logger;
extern crate r2d2;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate uuid;

mod db;
mod handlers;
mod models;
mod schema;

use actix::{Addr, SyncArbiter};
use actix_web::{
    fs::StaticFiles,
    middleware::{self, cors::Cors},
    server::HttpServer,
    App,
};
use db::{establish_connection_manager, DbExecutor};
use handlers::*;
use std::env::{set_var, var};

pub struct AppState {
    db: Addr<DbExecutor>,
}

// Start env_logger - for now, change this number to change log level
fn init_logging(level: u64) -> Result<(), String> {
    // if RUST_BACKTRACE is set, ignore the arg given and set `trace` no matter what
    let verbosity = if var("RUST_BACKTRACE").unwrap_or_else(|_| "0".into()) == "1" {
        "trace"
    } else {
        match level {
            0 => "warn",
            1 => "info",
            2 => "debug",
            3 | _ => "trace",
        }
    };
    if verbosity == "trace" {
        set_var("RUST_BACKTRACE", "1");
    };
    set_var("RUST_LOG", verbosity);
    pretty_env_logger::init();
    info!(
        "Set verbosity to {}",
        var("RUST_LOG").unwrap_or_else(|_| "Could not read RUST_LOG".to_string())
    );
    Ok(())
}

fn serve() -> Result<(), String> {
    dotenv::dotenv().ok();
    init_logging(1)?;

    // actix setup
    let sys = actix::System::new("drat");
    let url = "127.0.0.1:8080";

    let manager = establish_connection_manager();
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let addr = SyncArbiter::start(3, move || DbExecutor(pool.clone()));

    HttpServer::new(move || {
        App::with_state(AppState { db: addr.clone() })
            .configure({
                |app| {
                    Cors::for_app(app)
                        .send_wildcard()
                        .allowed_methods(vec!["GET"])
                        .max_age(3600)
                        // async handler, returning Box<Future<Item=HttpResponse, Error=actix_web::Error>>
                        .resource("/", |r| r.route().a(index))
                        .resource("/task/all", |r| r.route().with(get_all_tasks))
                        .resource("/task/new/{name}", |r| r.route().with(new_task))
                        .resource("/task/toggle/{id}", |r| r.route().with(toggle_task))
                        .resource("/task/{id}", |r| r.route().with(get_task))
                        .register()
                }
            }).middleware(middleware::Logger::default())
            .handler("/public", StaticFiles::new("./public/").unwrap())
    }).bind(url)
    .unwrap()
    .start();
    info!("Server initialized at {}", url);
    let _ = sys.run();
    Ok(())
}

fn main() {
    serve().unwrap();
}
