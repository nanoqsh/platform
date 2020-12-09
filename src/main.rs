mod auth;
mod config;
mod db;
mod models;
mod pool;
mod schema;
mod server;
mod session;
mod tokens;

#[macro_use]
extern crate diesel;

use actix_web::{
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use auth::{Failed, SignIn};
use config::Config;
use diesel::prelude::*;
use models::user::{NewUser, User};
use server::Server;
use std::net::SocketAddr;

async fn list_users(state: Data<Server>) -> impl Responder {
    use schema::users;

    let conn = state.pool().get().unwrap();
    let users: Vec<User> = users::table.load(&conn).expect("Error loading users");

    users.into_iter().fold(String::new(), |mut res, user| {
        use std::fmt::Write;
        writeln!(&mut res, "({}) {} {}", user.id, user.login, user.email).unwrap();
        res
    })
}

async fn sign_up(Json(user): Json<NewUser>, server: Data<Server>) -> impl Responder {
    let db = server.pool().get_db();
    let result = server.auth().sign_up(user, &db);

    match result {
        Ok(user) => HttpResponse::Ok().json(user),

        // TODO: Send error code
        Err(_) => HttpResponse::Ok().body("User not found"),
    }
}

async fn sign_in(Json(form): Json<SignIn>, server: Data<Server>) -> impl Responder {
    let db = server.pool().get_db();
    let result = server.auth().sign_in(&form, &db);

    match result {
        Ok(user) => format!("Hello, {}!", user.login),
        Err(Failed::UnknownLogin) => "Unknown login!".to_string(),
        Err(Failed::IncorrectPassword) => "Incorrect password!".to_string(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    use actix_web::web;

    let config = Config::load();
    let addr = parse_addr(&config.server);
    let state = Data::new(Server::new(config));

    let http_server = HttpServer::new(move || {
        App::new().app_data(state.clone()).service(
            web::scope("/api")
                .route("/list", web::get().to(list_users))
                .route("/user", web::post().to(sign_up))
                .route("/user", web::get().to(sign_in)),
        )
    })
    .bind(&addr)?;

    println!("Bound at http://{}", addr);

    http_server.run().await
}

fn parse_addr(server_config: &config::HttpServerConfig) -> SocketAddr {
    use std::str::FromStr;
    const LOCALHOST: &str = "127.0.0.1";

    let mut host = server_config.host.as_deref().unwrap_or(LOCALHOST);
    let port = server_config.port.unwrap_or(8080);

    if host == "localhost" {
        host = LOCALHOST;
    }

    let ip = std::net::IpAddr::from_str(host).expect("Failed ip addr parsing");
    SocketAddr::new(ip, port)
}
