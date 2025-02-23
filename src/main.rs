use std::collections::HashMap;
use std::{thread::sleep, time::Duration};
use std::io::stdin;
use std::io::Write;
use std::io;

use reqwest::Client;

use mlua::Lua;

async fn make_request(url: String, params: Option<HashMap<String, String>>) -> String {
    let client = Client::builder()
        .build()
        .expect("Failed to build client");

    let mut request = client.get(url);

    if let Some(p) = params {
        request = request.query(&p);
    }

    let res = request
        .send()
        .await
        .expect("Failed to send request");

    let text = res.text().await.unwrap();
    return format!("{}", text);
}

fn run_lua(code: String) {
    let lua = Lua::new();

    let _ = lua.load(code).exec();
}

#[tokio::main]
async fn main() {
    println!("Booting...");
    sleep(Duration::from_millis(500));
    println!("Retrieving data from hard drive...");
    let mut cmd: String = Default::default();
    let version: String = "ALPHA".to_string();
    let mut _latest_version: String = Default::default();
    let main_link: String = "http://puppet57.xyz/afos/php/".to_string();
    let mut program_to_run: String = Default::default();
    sleep(Duration::from_millis(500));
    println!("Loading kernel...");
    sleep(Duration::from_millis(500));
    println!("Checking dependencies...");
    sleep(Duration::from_millis(100));
    sleep(Duration::from_millis(500));
    println!("Loading video drivers...");
    sleep(Duration::from_millis(500));
    println!("Booted!");
    println!("Welcome to AFOS!");
    println!("Type \"help\" for a list of commands!");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let _ = stdin().read_line(&mut cmd).unwrap();
        cmd = cmd.trim().to_string();

        if cmd == "help" {
            println!("1: exit");
            println!("2: version");
            println!("3: run program");
        }

        else if cmd == "exit" {
            break;
        }

        else if cmd == "version" {
            println!("Getting data...");
            if _latest_version == "" {
                _latest_version = make_request(format!("{}get_latest_version.php", main_link), None).await;
            } else {
                sleep(Duration::from_millis(300));
            }
            println!("Installed Version: {}", version);
            println!("Latest Version: {}", _latest_version);
        }

        else if cmd == "run program" {
            print!("Program to run: ");
            io::stdout().flush().unwrap();
            let _ = stdin().read_line(&mut program_to_run).unwrap();
            program_to_run = program_to_run.trim().to_string();

            let lua_code = std::fs::read_to_string(format!("./programs/{}.lua", program_to_run))
                .expect("Failed to load lua code. Does the program exist?");

            run_lua(format!("{}", lua_code));
        }

        else {
            println!("{} is a stupid command! Try again!", cmd);
        }

        cmd = Default::default();
    }
}