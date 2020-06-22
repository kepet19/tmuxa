use std::process::Command;
use std::{io, io::Write};

enum Chosen {
    Chosen(String),
    New,
}

fn main() {
    let chosen = list_sessions();
    match chosen {
        Chosen::Chosen(name) => {attatch(&name)},
        Chosen::New => {create_new()},
    }
}


fn attatch(name: &str) {
    let mut tmux = Command::new("tmux");
    match std::env::var("TMUX") {
        Ok(_) => {
            tmux.args(&["switch-client", "-t", &name]);
        },
        Err(_) => {
            tmux.args(&["attach", "-t", &name]);
        },
    } 
    let mut result = tmux.spawn().expect("Error could not spawn tmux process");
    result.wait().expect("failed to wait");
}

fn create_new() {
    let name = read_input("new session name: ").unwrap();

    match std::env::var("TMUX") {
        Ok(_) => {
            let mut tmux = Command::new("tmux");
            tmux.args(&["new", "-s", &name, "-d"]);
            tmux.spawn().expect("Error could not spawn tmux process")
                .wait().expect("failed to wait");

            println!("does this work");
            let mut tmux = Command::new("tmux");
            tmux.args(&["switch-client", "-t", &name]);
            tmux.spawn().expect("Error could not spawn tmux process")
                .wait().expect("failed to wait");
        },
        Err(_) => {
            let mut tmux = Command::new("tmux");
            tmux.args(&["new", "-s", &name]);
            tmux.spawn().expect("Error could not spawn tmux process")
                .wait().expect("failed to wait");
        },
    } 
}


fn read_input(output_text: &str) -> io::Result<String>{
    let mut input = String::new();
    print!("{}", output_text);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)?;
    Ok(input.replace("\n", ""))
}

fn list_sessions() -> Chosen {
    let mut tmux = Command::new("tmux");
    tmux.args(&["list-session", "-F", "#S - #{session_attached}"]);
    let sessions = tmux.output().expect("there is no session?");
    let sessions = String::from_utf8(sessions.stdout).unwrap();
    let count = sessions.split("\n").count()-1;
    let sessions = sessions.split("\n");

    println!("there are {} session(s)", count); 
    let mut index = 0;
    println!("{}. Create new", index);
    let mut names: Vec<String> = Vec::new();
    for session in sessions {
        index += 1;
        if index > count {break;}
        let mut session = session.split("-");
        let name = session.next().unwrap().trim().into();
        let attach = session.next().unwrap().trim();
        let attach: u8 = attach.parse().unwrap();
        if attach > 0 {
            println!("{}. name: {}, attached", index, name);
        } else {
            println!("{}. name: {}", index, name);
        }
        names.push(name);
    }
    let chose: usize = read_input("input: ").unwrap().parse().expect("not a number");
    if chose == 0 || chose > count {
        Chosen::New
    } else {
        Chosen::Chosen(names.remove(chose-1))
    }
}
// Author: Kevin Kamer Meejach Petersen
// Data: 24-10-2019
// License: GPLv3
