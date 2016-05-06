use std::io::{stderr,BufRead, BufReader, Write};
use std::process::{Command,exit,Stdio};
use std::sync::mpsc::{channel, Sender};
use std::thread;

const OCCUPIED_ICON: &'static str = "";
const FREE_ICON: &'static str = "";

// enum Workspace {
//     OccupiedFocused,
//     OccupiedUnfocused,
//     FreeFocused,
//     FreeUnfocused,
//     UrgentFocused,
//     UrgentUnfocused,
// }

// enum Update {
//     Workspaces(String),
//     Time(String),
//     Title(String),
// }

fn workspace_info(sender: Sender<String>) {
    let output = match Command::new("bspc").arg("subscribe").arg("report").stdout(Stdio::piped()).spawn() {
        Ok(out) => out,
        Err(_) => {
            let _ = stderr().write("Failed to run 'bspc'\n".as_bytes()).unwrap();
            exit(1);
        },
    };

    let reader = BufReader::new(output.stdout.unwrap());
    for line in reader.lines() {
        let _ = sender.send(format!("%{{c}}"));
        let _ = sender.send(format!("%{{A4:bspc desktop -f prev:}}%{{A5:bspc desktop -f next:}}"));
        let line = line.unwrap();
        let line_vector = line.split(":").collect::<Vec<&str>>();

        for line in &line_vector {
            let workspace = &line[1..];

            let _ = sender.send(format!("%{{A:bspc desktop -f {}:}}", workspace));

            match &line[0..1] {
                // Occupied focused
                "O" => {
                    let _ = sender.send(format!("%{{F#FFF6F9FF}} {} %{{F-}}", OCCUPIED_ICON));
                }

                // Occupied unfocused
                "o" => {
                    let _ = sender.send(format!("%{{F#FFA3A6AB}} {} %{{F-}}", OCCUPIED_ICON));
                }

                // Free focused
                "F" => {
                    let _ = sender.send(format!("%{{F#FFF6F9FF}} {} %{{F-}}", FREE_ICON));
                }

                // Free unfocused
                "f" => {
                    let _ = sender.send(format!("%{{F#FF6F7277}} {} %{{F-}}", FREE_ICON));
                }

                // Urgent focused
                "U" => {
                    let _ = sender.send(format!("%{{F#FF916255}} {} %{{F-}}", OCCUPIED_ICON));
                }

                // Urgent unfocused
                "u" => {
                    let _ = sender.send(format!("%{{F#FF543B3B}} {} %{{F-}}", OCCUPIED_ICON));
                }

                _ => {}
            }
            let _ = sender.send(format!("%{{A}}"));
        }
        let _ = sender.send(format!("%{{A}}%{{A}}"));
        let _ = sender.send(format!("\n"));
    }
}

fn main() {
    let (sender, receiver) = channel::<String>();

    let _ = thread::spawn(move || {
        workspace_info(sender.clone());
    });

    for line in receiver.iter() {
        print!("{}", line);
    }
}
