use std::io::{stdout, stderr,BufRead, BufReader, Write};
use std::process::{Command,exit,Stdio};
use std::sync::mpsc::channel;
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
//     Workspaces(Workspace),
//     Time(String),
//     Title(String),
// }

// workspace_info() -> String {

// }

fn main() {
    let output = match Command::new("bspc").arg("subscribe").arg("report").stdout(Stdio::piped()).spawn() {
        Ok(out) => out,
        Err(_) => {
            let _ = stderr().write("Failed to run 'bspc'\n".as_bytes()).unwrap();
            exit(1);
        },
    };

    let reader = BufReader::new(output.stdout.unwrap());
    for line in reader.lines() {
        print!("%{{c}}");
        print!("%{{A4:bspc desktop -f prev:}}%{{A5:bspc desktop -f next:}}");
        let line = line.unwrap();
        let line_vector = line.split(":").collect::<Vec<&str>>();

        for line in &line_vector {
            let workspace = &line[1..];

            print!("%{{A:bspc desktop -f {}:}}", workspace);

            match &line[0..1] {
                "O" => {
                    print!("%{{F#FFF6F9FF}} {} %{{F-}}", OCCUPIED_ICON); // Occupied focused
                }
                "o" => {
                    print!("%{{F#FFA3A6AB}} {} %{{F-}}", OCCUPIED_ICON); // Occupied unfocused
                }
                "F" => {
                    print!("%{{F#FFF6F9FF}} {} %{{F-}}", FREE_ICON); // Free focused
                }
                "f" => {
                    print!("%{{F#FF6F7277}} {} %{{F-}}", FREE_ICON); // Free unfocused
                }
                "U" => {
                    print!("%{{F#FF916255}} {} %{{F-}}", OCCUPIED_ICON); // Urgent focused
                }
                "u" => {
                    print!("%{{F#FF543B3B}} {} %{{F-}}", OCCUPIED_ICON); // Urgent unfocused
                }
                _ => {},
            }
            print!("%{{A}}");
        }
        print!("%{{A}}%{{A}}");
        println!("");
    }
}
