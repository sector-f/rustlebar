extern crate chrono;

use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::mpsc::{channel, Sender};
use std::thread::{self, sleep};
use std::time::Duration;

const OCCUPIED_ICON: &'static str = "";
const FREE_ICON: &'static str = "";

enum Update {
    Workspaces(String),
    Time(String),
    Title(String),
}

fn workspace_info(sender: Sender<Update>) {
    let output = Command::new("bspc").arg("subscribe").arg("report").stdout(Stdio::piped()).spawn().unwrap();

    let reader = BufReader::new(output.stdout.unwrap());
    for line in reader.lines() {
        let mut message: Vec<String> = Vec::new();
        message.push(format!(" %{{l}}"));
        message.push(format!("%{{A4:bspc desktop -f prev:}}%{{A5:bspc desktop -f next:}}"));

        let line = line.unwrap();
        let line_vector = line.split(":").collect::<Vec<&str>>();

        for line in &line_vector {
            match &line[0..1] {
                // Occupied focused
                "O" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F#FFF6F9FF}} {}%{{F-}}", OCCUPIED_ICON));
                    message.push(format!("%{{A}}"));
                }

                // Occupied unfocused
                "o" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F#FFA3A6AB}} {}%{{F-}}", OCCUPIED_ICON));
                    message.push(format!("%{{A}}"));
                }

                // Free focused
                "F" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F#FFF6F9FF}} {}%{{F-}}", FREE_ICON));
                    message.push(format!("%{{A}}"));
                }

                // Free unfocused
                "f" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F#FF6F7277}} {}%{{F-}}", FREE_ICON));
                    message.push(format!("%{{A}}"));
                }

                // Urgent focused
                "U" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F#FF916255}} {}%{{F-}}", OCCUPIED_ICON));
                    message.push(format!("%{{A}}"));
                }

                // Urgent unfocused
                "u" => {
                    message.push(format!("%{{A:bspc desktop -f {}:}}", &line[1..]));
                    message.push(format!("%{{F#FF543B3B}} {}%{{F-}}", OCCUPIED_ICON));
                    message.push(format!("%{{A}}"));
                }

                _ => {}
            }
        }
        message.push(format!("%{{A}}%{{A}}"));

        let _ = sender.send(Update::Workspaces(message.join("")));
    }
}

fn title(sender: Sender<Update>) {
    let output = Command::new("xtitle").arg("-s").arg("-i").arg("-t").arg("100").stdout(Stdio::piped()).spawn().unwrap();

    let reader = BufReader::new(output.stdout.unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let _ = sender.send(Update::Title(format!("%{{c}}{}", line)));
    }
}

fn time(sender: Sender<Update>) {
    loop {
        let time = chrono::Local::now().format("%I:%M %p");
        let _ = sender.send(Update::Time(format!("%{{r}}%{{B#FF665B5B}} {} %{{B-}}", time)));
        sleep(Duration::from_secs(1));
    }
}

fn main() {
    let (sender, receiver) = channel::<Update>();

    let mut workspace_message: String = "".to_owned();
    let mut title_message: String = "".to_owned();
    let mut time_message: String = "".to_owned();
    let mut message: String = "".to_owned();

    let sender_clone = sender.clone();
    let _ = thread::spawn(move || {
        workspace_info(sender_clone);
    });

    let sender_clone = sender.clone();
    let _ = thread::spawn(move || {
        title(sender_clone);
    });

    let sender_clone = sender.clone();
    let _ = thread::spawn(move || {
        time(sender_clone);
    });

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
            println!("{}", message);
        }
    }
}
