use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mysql::{Opts, Pool};
use std::env;
use mysql::prelude::Queryable;

#[get("/")]
async fn hello() -> impl Responder {
    let res = query_sql("SHOW TABLES;");
    HttpResponse::Ok().body(res)
}

#[get("/hosts")]
async fn hosts_page() -> impl Responder {
    let res = query_sql("SELECT * FROM hosts;");
    HttpResponse::Ok().body(res)
}

#[post("/hosts/{id}/commands")]
fn get_commands_for_host(id: web::Path<String>) -> impl Responder {
    let res = query_sql(&format!("SELECT * FROM commands WHERE host_id = {};", id));
    HttpResponse::Ok().body(res)
}

#[derive(serde::Deserialize)]
struct CommandResponse {
    id: i32,
    response: String,
}

//function to take json via post request and insert into database
#[post("/hosts/{id}/response")]
fn get_response_for_host(id: web::Path<String>, command_response: web::Json<CommandResponse>) -> impl Responder {
    //the response is json, so we need to parse #TODO
    let res = query_sql(&format!("SELECT update_command_response ({}, {});", id, command_response.response));
    HttpResponse::Ok().body(res)
}



fn query_sql(query: &str) -> String {
    // Get database URL from environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Establish a database connection
    let opts = Opts::from_url(&database_url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Make a query and get the result
    let result = conn.query_map(query, |row: mysql::Row| row.get::<String, _>(0)).unwrap();


    // Convert the result to a string
    let result_string = result.iter().fold(String::new(), |acc, x| acc + &x.clone().unwrap() + "\n");

    println!("{}", result_string);
    result_string
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
