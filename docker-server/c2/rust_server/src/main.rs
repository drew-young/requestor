use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use mysql::{Opts, Pool};
use std::env;
use mysql::prelude::Queryable;

#[get("/")]
async fn hello() -> impl Responder {
    let res = query("SHOW TABLES;");
    HttpResponse::Ok().body(res)
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

fn query(query: &str) -> String {
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
