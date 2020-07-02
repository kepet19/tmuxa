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
    let name = read_input("new session name: ").unwrap();

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
        println!("{}. Delete A tmux session", index);
    }
    index += 1;
    println!("{}. Create new", index);
    let mut names: Vec<String> = Vec::new();
    println!("there are {} session(s)", count);

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
            println!(" {}. {}, (attached)", index, name);
        } else {
            println!(" {}. {}", index, name);
        }
        names.push(name);
    }
    let mut chosen: usize = read_input("input: ")
        .unwrap()
        .parse()
        .expect("not a number");
    if chosen == 0 {
        chosen = read_input("Which tmux session to delete expect a number: ")
            .unwrap()
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
