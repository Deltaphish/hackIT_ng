mod challenge;

#[macro_use]
extern crate dotenv_codegen;

// use std::collections::HashMap;

// use actix_files::Files;

// use actix_http::{body::Body, Response};
// use actix_web::dev::ServiceResponse;
// use actix_web::http::StatusCode;
// use actix_web::middleware::errhandlers::{ErrorHandlerResponse, ErrorHandlers};
use actix::clock::delay_for;
use actix_files::Files;
use actix_web::{get, middleware, web, App, Error, HttpResponse, HttpServer, Result};
use tokio_postgres::{Client, NoTls};
// use tera::Tera;
use listenfd::ListenFd;

use dotenv;

use std::sync::Mutex;
use std::time::Duration;

struct AppState {
    db_client: Mutex<Client>,
}

#[get("/completions")]
async fn get_completions(data: web::Data<AppState>) -> Result<HttpResponse, Error> {
    // Will cause panic if mutex is poisoned. This is intentional since the client could be corrupted"
    let db = data.db_client.lock().unwrap();

    let query = db
        .query(
            "SELECT \"challenge_id\" FROM \"completions\" WHERE \"user\" = 'peppe'",
            &[],
        )
        .await;

    match query {
        Ok(rows) => {
            let mut res = Vec::new();
            for row in rows {
                let challenge: String = row.get("challenge_id");
                res.push(challenge);
            }
            Ok(HttpResponse::Ok().body(res.join(",")))
        }
        Err(e) => Ok(HttpResponse::InternalServerError().body("Error, faild to run db query")),
    }
}

// async fn index( data: web::Data<AppState>) -> Result<HttpResponse, Error>{

//     tmpl.render("index.html", &tera::Context::new())
//         .map_err(|_| error::ErrorInternalServerError("Template error"))?;

//     Ok(HttpResponse::Ok().content_type("text/html").body(s))
// }

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let url = dotenv!("DATABASE_URL");
    println!("{}", url);
    let mut connection_attempt = tokio_postgres::connect(&url, NoTls).await;

    while let Err(e) = connection_attempt {
        eprintln!(
            "Error establishing connection to db: {}\n Reattempting connection in 10 seconds",
            e
        );
        delay_for(Duration::new(10, 0)).await;
        connection_attempt = tokio_postgres::connect(&url, NoTls).await;
    }

    let (client, connection) =
        connection_attempt.expect("Internal Error: database connection failiure was not handled");

    println!("Successfully connected to db");

    // Run connection in seperate thread
    actix::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let app_state = web::Data::new(AppState {
        db_client: Mutex::new(client),
        // tera : new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**")).unwrap(),
    });

    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(middleware::Logger::default())
            .service(get_completions)
            .service(Files::new("/static", "static/").show_files_listing())
            .service(Files::new("/", "static/").index_file("index.html"))
    });

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
}
