use actix_files::Files;
use actix_web::{middleware, App, HttpResponse, HttpServer, Error, Responder, web, get};
use actix;
use tokio_postgres::{NoTls, Error as DbError,Client};
use std::env;
use std::ffi::OsString;
use std::sync::Mutex;

struct AppState {
    db_client: Mutex<Client>,
}

#[get("/completions")]
async fn get_completions( data: web::Data<AppState>) -> Result<HttpResponse, Error>{
    let mut db = data.db_client.lock().unwrap();
    
    let mut res = Vec::new();

    for row in db.query("SELECT \"challenge_id\" FROM \"completions\" WHERE \"user\" = 'peppe'",&[]).await.unwrap(){
        let challenge: String = row.get("challenge_id");
        res.push(challenge);
    }

    Ok(HttpResponse::Ok().body(res.join(",")))
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let config = match env::var_os("DATABASE_URL") {
                    Some(cfg) => cfg.into_string().expect("postgresql://please:changeme@hackit-postgresql/hackit"),
                    None => String::from("postgresql://please:changeme@hackit-postgresql/hackit")
                };

    let (client, connection) = tokio_postgres::connect(&config, NoTls).await.unwrap();

    actix::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let appState = web::Data::new(AppState{
        db_client : Mutex::new(client),
    });

    HttpServer::new( move || {
        App::new()
            .app_data(appState.clone())
            .wrap(middleware::Logger::default())
            .service(get_completions)
            //.service(Files::new("/static","static/").show_files_listing())
            //.service(Files::new("/","static/").index_file("index.html"))
    })

    .bind("0.0.0.0:1337")?
    .run()
    .await
}