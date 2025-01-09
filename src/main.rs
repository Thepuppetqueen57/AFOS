use std::{thread::sleep, time::Duration};
use std::io::stdin;
use std::io::Write;
use std::io;

fn main() {
    println!("Booting...");
    sleep(Duration::from_millis(500));
    println!("Retrieving data from hard drive...");
    let mut cmd: String = Default::default();
    sleep(Duration::from_millis(500));
    println!("Loading kernel...");
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
            println!("exit");
        }

        else if cmd == "exit" {
            break;
        }

        else {
            println!("{} is a stupid command! Try again!", cmd);
        }

        cmd = Default::default();
    }
}