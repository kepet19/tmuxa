use std::process::Command;
use std::{io, io::Write};

enum Chosen {
    Chosen(String),
    Delete(String),
    New,
}

fn main() {
    let chosen = list_sessions();
    match chosen {
        Chosen::Chosen(name) => attatch(&name),
        Chosen::New => create_new(),
        Chosen::Delete(name) => delete_session(&name),
    }
}

fn attatch(name: &str) {
    let mut tmux = Command::new("tmux");
    match std::env::var("TMUX") {
        Ok(_) => {
            tmux.args(&["switch-client", "-t", &name]);
        }
        Err(_) => {
            tmux.args(&["attach", "-t", &name]);
        }
    }
    let mut result = tmux.spawn().expect("Error could not spawn tmux process");
    result.wait().expect("failed to wait");
}

fn delete_session(name: &str) {
    let mut tmux = Command::new("tmux");
    tmux.args(&["kill-session", "-t", &name]);
    let mut result = tmux.spawn().expect("Error could not spawn tmux process");
    result.wait().expect("failed to wait");
}

fn create_new() {
    let name = read_input("\x1b[96mnew session name:> \x1b[m").unwrap();

    match std::env::var("TMUX") {
        Ok(_) => {
            let mut tmux = Command::new("tmux");
            tmux.args(&["new", "-s", &name, "-d"]);
            tmux.spawn()
                .expect("Error could not spawn tmux process")
                .wait()
                .expect("failed to wait");

            println!("does this work");
            let mut tmux = Command::new("tmux");
            tmux.args(&["switch-client", "-t", &name]);
            tmux.spawn()
                .expect("Error could not spawn tmux process")
                .wait()
                .expect("failed to wait");
        }
        Err(_) => {
            let mut tmux = Command::new("tmux");
            tmux.args(&["new", "-s", &name]);
            tmux.spawn()
                .expect("Error could not spawn tmux process")
                .wait()
                .expect("failed to wait");
        }
    }
}

fn read_input(output_text: &str) -> io::Result<String> {
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
    let count = sessions.split("\n").count() - 1;
    let sessions = sessions.split("\n");

    let mut index = 0;
    if count > 0 {
        println!("\x1b[1;31m{}. Delete A tmux session\x1b[m", index);
    }
    index += 1;
    println!("\x1b[96m{}. Create new\x1b[m", index);
    let mut names: Vec<String> = Vec::new();
    println!("\x1b[93mthere are {} session(s):\x1b[m", count);

    for session in sessions {
        index += 1;
        if index > count + 1 {
            break;
        }
        let mut session = session.split("-");
        let name = session.next().unwrap().trim().into();
        let attach = session.next().unwrap().trim();
        let attach: u8 = attach.parse().unwrap();
        if attach > 0 {
            println!(" \x1b[34m{}. {}, \x1b[m\x1b[31m(attached)\x1b[m", index, name);
        } else {
            println!(" \x1b[34m{}. {}\x1b[m", index, name);
        }
        names.push(name);
    }
    let mut chosen: usize = read_input("> ")
        .unwrap()
        .trim()
        .parse()
        .expect("not a number");
    if chosen == 0 {
        chosen = read_input("\x1b[1;31mWhich tmux session to delete expects a number: \x1b[m")
            .unwrap()
            .trim()
            .parse()
            .expect("not a number");
        if chosen < 2{
            panic!("the chosing number is not valid");
        }
        Chosen::Delete(names.remove(chosen - 2))
    } else if chosen == 1 || chosen > count + 1{
        Chosen::New
    } else {
        Chosen::Chosen(names.remove(chosen - 2))
    }
}
// Author: Kevin Kamer Meejach Petersen
// Data: 24-10-2019
// License: GPLv3
