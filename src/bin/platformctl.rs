use clap::{crate_authors, crate_version, App, Arg, SubCommand};

use platformctl::audio::pulseaudio::PulseAudio;
use platformctl::backlight::Backlight;

fn main() {
    let matches = App::new("platformctl")
        .version(crate_version!())
        .author(crate_authors!())
        .subcommand(
            SubCommand::with_name("backlight")
                .about("Control backlight device")
                .arg(
                    Arg::with_name("add")
                        .help("Value to add to backlight output")
                        .index(1),
                )
                .arg(Arg::with_name("device").help("Device to access")),
        )
        .subcommand(
            SubCommand::with_name("audio")
                .about("Control audio devices")
                .arg(
                    Arg::with_name("add")
                        .help("Value to add to audio volume")
                        .index(1),
                )
                .arg(Arg::with_name("device").help("Device to access")),
        )
        .get_matches();

    match matches.subcommand() {
        ("backlight", Some(sub)) => {
            let mut b = match Backlight::new() {
                Err(_) => {
                    eprintln!("could not initialize a backlight connector");
                    std::process::exit(2);
                }
                Ok(v) => v,
            };
            match sub.value_of_lossy("add") {
                None => println!("{}", b.brightness()),
                Some(v) => {
                    let step: f32 = match v.parse() {
                        Err(e) => {
                            eprintln!("unable to parse add: {:}", e);
                            std::process::exit(1);
                        }
                        Ok(v) => v,
                    };
                    match b.add(step) {
                        Err(e) => {
                            eprintln!("unable to add brightness: {:}", e);
                            std::process::exit(3);
                        }
                        Ok(_) => match b.sync() {
                            Err(e) => {
                                eprintln!("unable to set brightness: {:}", e);
                                std::process::exit(3);
                            }
                            Ok(_) => (),
                        },
                    };
                }
            };
        }
        ("audio", Some(sub)) => {
            let p = match PulseAudio::new(None) {
                Err(_) => {
                    eprintln!("could not initialize an audio connector");
                    std::process::exit(2);
                }
                Ok(v) => v,
            };
            match sub.value_of_lossy("add") {
                None => println!("{}", p.volume()),
                Some(v) => {
                    let step: f32 = match v.parse() {
                        Err(e) => {
                            eprintln!("unable to parse add: {:}", e);
                            std::process::exit(1);
                        }
                        Ok(v) => v,
                    };
                    match p.add_volume(step) {
                        Err(e) => {
                            eprintln!("unable to add volume: {:}", e);
                            std::process::exit(3);
                        }
                        Ok(_) => (),
                    };
                }
            }
        }
        _ => {
            eprintln!("must specify subcommand");
            std::process::exit(1);
        }
    }
}
