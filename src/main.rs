extern crate chrono;

use std::io::{BufRead, BufReader, stderr, Write};
use std::process::{Command, Stdio, exit};
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, sleep};
use std::time::Duration;

mod configuration;

enum Update {
    Workspaces(String),
    Time(String),
    Title(String),
}

fn workspace_info(sender: Sender<Update>) {
    let output = Command::new("bspc").args(&["subscribe", "report"]).stdout(Stdio::piped()).spawn().expect("Failed to run bspc");

    let icons: configuration::Icons = configuration::get_icons();
    let colors: configuration::Colors = configuration::get_colors();

    let reader = BufReader::new(output.stdout.expect("Failed to read stdout of bspc"));
    for line in reader.lines() {
        let mut message: Vec<String> = Vec::new();
        message.push("%{l} ".to_owned());
        message.push("%{A4:bspc desktop -f prev:}%{A5:bspc desktop -f next:}".to_owned());

        let line = line.expect("Failed to read line from bspc");
        let line_vector = line.split(":").collect::<Vec<&str>>();

        for line in &line_vector {
            match &line[0..1] {
                // Occupied focused
                "O" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F{}}}{}%{{F-}}", colors.occupied_focused, icons.occupied_focused));
                    message.push("%{A} ".to_owned());
                }

                // Occupied unfocused
                "o" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F{}}}{}%{{F-}}", colors.occupied_unfocused, icons.occupied_unfocused));
                    message.push("%{A} ".to_owned());
                }

                // Free focused
                "F" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F{}}}{}%{{F-}}", colors.free_focused, icons.free_focused));
                    message.push("%{A} ".to_owned());
                }

                // Free unfocused
                "f" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F{}}}{}%{{F-}}", colors.free_unfocused, icons.free_unfocused));
                    message.push("%{A} ".to_owned());
                }

                // Urgent focused
                "U" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F{}}}{}%{{F-}}", colors.urgent_focused, icons.urgent_focused));
                    message.push("%{A} ".to_owned());
                }

                // Urgent unfocused
                "u" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F{}}}{}%{{F-}}", colors.urgent_unfocused, icons.urgent_unfocused));
                    message.push("%{A} ".to_owned());
                }

                _ => {}
            }
        }
        message.push("%{A}%{A}".to_owned());

        let _ = sender.send(Update::Workspaces(message.join("")));
    }
}

fn title(sender: Sender<Update>, length: &str) {
    let output = Command::new("xtitle").args(&["-s", "-i", "-t", length]).stdout(Stdio::piped()).spawn().expect("Failed to run xtitle");

    let reader = BufReader::new(output.stdout.expect("Failed to read stdout of xtitle"));
    for line in reader.lines() {
        let line = line.expect("Failed to read line from xtitle");
        let _ = sender.send(Update::Title(format!("%{{c}}{}", line)));
    }
}

fn time(sender: Sender<Update>) {
    loop {
        let time = chrono::Local::now().format("%-I:%M %p");
        let _ = sender.send(Update::Time(format!("%{{r}}%{{B#FF665B5B}} {} %{{B-}}", time)));
        sleep(Duration::from_secs(1));
    }
}

fn main() {
    let lemonbar_options = configuration::get_lemonbar_options();

    let lemonbar = match Command::new("lemonbar")
        .arg("-g").arg(format!("{}x{}+{}+{}", &lemonbar_options.width,
                                              &lemonbar_options.height,
                                              &lemonbar_options.x,
                                              &lemonbar_options.y))
        .arg("-a").arg(&lemonbar_options.clickable_areas)
        .arg("-f").arg(&lemonbar_options.text_font)
        .arg("-f").arg(&lemonbar_options.icon_font)
        .arg("-B").arg(&lemonbar_options.background_color)
        .stdin(Stdio::piped())
        .spawn() {
        Ok(bar) => bar,
        Err(_) => {
            let _ = stderr().write("Failed to run lemonbar".as_bytes()).unwrap();
            exit(1);
        },
    };

    let mut lemonbar_stdin = lemonbar.stdin.unwrap();

    let (sender, receiver) = channel::<Update>();
    let (sender_clone1, sender_clone2, sender_clone3) = (sender.clone(), sender.clone(), sender.clone());

    let mut workspace_message = String::new();
    let mut title_message = String::new();
    let mut time_message = String::new();
    let mut message = String::new();

    let _ = thread::spawn(move || { workspace_info(sender_clone1); });
    let _ = thread::spawn(move || { title(sender_clone2, &lemonbar_options.title_length); });
    let _ = thread::spawn(move || { time(sender_clone3); });

    for line in receiver.iter() {
        match line {
            Update::Workspaces(info) => {
                workspace_message = info;
            }

            Update::Title(title) => {
                title_message = title;
            }

            Update::Time(time) => {
                time_message = time;
            }
        }

        if message != workspace_message.to_owned() + &title_message + &time_message {
            message = workspace_message.to_owned() + &title_message + &time_message;
            sleep(Duration::from_millis(5));
            let _ = lemonbar_stdin.write(format!("{}\n", message).as_bytes()).unwrap();
        }
    }
}
