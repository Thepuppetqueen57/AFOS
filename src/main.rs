use std::{thread::sleep, time::Duration};
use std::io::stdin;
use std::io::Write;
use std::io;

use reqwest::Client;
use reqwest::Proxy;
use tokio::net::TcpStream;

async fn is_tor_running() -> bool {
    match TcpStream::connect("127.0.0.1:9050").await {
        Ok(_) => true,
        Err(_) => false,
    }
}

async fn make_request(url: String) -> String {
    let proxy = Proxy::all("socks5h://127.0.0.1:9050")
        .expect("Failed to set up proxy");

    let client = Client::builder()
        .proxy(proxy)
        .build()
        .expect("Failed to build client");

    let res = client
        .get(url)
        .send()
        .await
        .expect("Failed to send request");

    let text = res.text().await.unwrap();
    return format!("{}", text);
}

#[tokio::main]
async fn main() {
    println!("Booting...");
    sleep(Duration::from_millis(500));
    println!("Retrieving data from hard drive...");
    let mut cmd: String = Default::default();
    let version: String = "ALPHA".to_string();
    let mut _latest_version: String = Default::default();
    let tor_link: String = "http://yuoqw7ywmixj55zxljkhqvcwunovze32df7pqemwacfaq2itqefbixad.onion/afos/php/".to_string();
    sleep(Duration::from_millis(500));
    println!("Loading kernel...");
    sleep(Duration::from_millis(500));
    println!("Checking dependencies...");
    sleep(Duration::from_millis(100));
    
    let tor_running = is_tor_running().await;
    if !tor_running {
        println!("Tor is not running. Please start tor");
    } else {
        println!("Tor running.");
    }

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
        }

        else if cmd == "exit" {
            break;
        }

        else if cmd == "version" {
            println!("Getting data...");

            if tor_running {
                _latest_version = make_request(format!("{}get_latest_version.php", tor_link)).await;
                println!("Installed Version: {}", version);
                println!("Latest Version: {}", _latest_version);
            } else {
                sleep(Duration::from_millis(1000));
                println!("Tor not running! Cancelling...");
            }
        }

        else {
            println!("{} is a stupid command! Try again!", cmd);
        }

        cmd = Default::default();
    }
}