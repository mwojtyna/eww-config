use alsa::{ctl::Ctl, Mixer};
use regex::Regex;
use std::{env, process::Command};

const CARD_ID: &str = "hw:0";
const MIXER_NAME: &str = "Master";

fn main() {
    let ctl = Ctl::new(CARD_ID, false).expect("Error opening ctl");
    ctl.subscribe_events(true)
        .expect("Error subscribing to ctl events");
    let mixer = Mixer::new(CARD_ID, false).expect("Error opening mixer");

    let args = env::args().collect::<Vec<String>>();
    match args.len() {
        1 => {
            let data = read_audio(&mixer);
            if let Some((vol, toggle)) = data {
                println!("{} {}", vol, toggle);

                loop {
                    let event = ctl.read().expect("Error reading ctl event");
                    if event.is_some() {
                        let data = read_audio(&mixer);
                        if let Some((vol, toggle)) = data {
                            println!("{} {}", vol, toggle);
                        }
                    }
                }
            }
        }
        2 => match args[1].as_str() {
            "--volume" => {
                let data = read_audio(&mixer);
                if let Some((vol, _)) = data {
                    println!("{}", vol);

                    loop {
                        let event = ctl.read().expect("Error reading ctl event");
                        if event.is_some() {
                            let data = read_audio(&mixer);
                            if let Some((vol, _)) = data {
                                println!("{}", vol);
                            }
                        }
                    }
                }
            }
            "--toggle" => {
                let data = read_audio(&mixer);
                if let Some((_, toggle)) = data {
                    println!("{}", toggle);

                    loop {
                        let event = ctl.read().expect("Error reading ctl event");
                        if event.is_some() {
                            let data = read_audio(&mixer);
                            if let Some((_, toggle)) = data {
                                println!("{}", toggle);
                            }
                        }
                    }
                }
            }
            _ => (),
        },
        _ => (),
    }
}

fn read_audio(mixer: &Mixer) -> Option<(String, String)> {
    mixer.handle_events().expect("Error handling mixer events");

    let stdout = Command::new("amixer")
        .arg("sget")
        .arg(MIXER_NAME)
        .output()
        .expect("Failed to execute amixer")
        .stdout;
    let output = String::from_utf8(stdout).expect("Failed converting stdout to string");

    let regex = Regex::new(r"\[(.*)%\] \[(.*)\]").unwrap();
    let captures = regex.captures(&output);

    if let Some(captures) = captures {
        let vol = captures
            .get(1)
            .expect("Mixer volume not found in regex")
            .as_str();
        let toggle = captures
            .get(2)
            .expect("Mixer toggle state not found in regex")
            .as_str();

        Some((vol.to_owned(), toggle.to_owned()))
    } else {
        None
    }
}
