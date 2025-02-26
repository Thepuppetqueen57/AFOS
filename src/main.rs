use std::collections::HashMap;
use std::{thread::sleep, time::Duration};
use std::io::stdin;
use std::io::Write;
use std::io;

use reqwest::Client;

use mlua::Lua;

macro_rules! hashmap {
    ( $( $x:expr => $x1:expr ),* ) => {
        {
            let mut _temp_hashmap = HashMap::new();
            $(
                _temp_hashmap.insert($x, $x1);
            )*
            _temp_hashmap
        }
    };

    () => {
        HashMap::new()
    }
}

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
    let main_link: String = "http://afos.puppet57.xyz/php/".to_string();
    let mut program_to_run: String = Default::default();
    let mut program_to_install: String = Default::default();
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
            sleep(Duration::from_millis(100));
            println!("2: version");
            sleep(Duration::from_millis(100));
            println!("3: run program");
            sleep(Duration::from_millis(100));
            println!("4: install program");
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

        else if cmd == "install program" {
            print!("Program to install: ");
            io::stdout().flush().unwrap();
            let _ = stdin().read_line(&mut program_to_install).unwrap();
            program_to_install = program_to_install.trim().to_string();

            let parameters: HashMap<String, String> = hashmap! {
                "program".to_string() => program_to_install.clone()
            };

            let program_file = make_request(
                format!("{}get_program.php", main_link),
                Some(parameters)
            ).await;

            let _ = std::fs::write(format!("./programs/{}.lua", program_to_install), program_file);

            println!("Program {} has been installed!", program_to_install);
        }

        else {
            println!("{} is a stupid command! Try again!", cmd);
        }

        cmd = Default::default();
    }
}