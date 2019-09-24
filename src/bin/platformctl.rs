use clap::{crate_authors, crate_version, App, Arg, SubCommand};

use platformctl::audio::pulseaudio::PulseAudioSoundDevice;
use platformctl::backlight::Backlight;

fn parse_bool(value: &str, current: bool) -> bool {
    match value {
        "on" | "true"   =>  true,
        "off" | "false" => false,
        "toggle"        => !current,
        v               => {
            eprintln!("unable to parse boolean: {:}", v);
            eprintln!("    use on|off|toggle");
            std::process::exit(1);
        },
    }
}

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
                .subcommand(SubCommand::with_name("volume")
                    .arg(
                        Arg::with_name("add")
                            .help("Value to add to sound volume")
                            .index(1),
                    )
                    .arg(
                        Arg::with_name("max")
                            .help("Max value to cap volume to")
                            .index(2),
                    )
                )
                .subcommand(SubCommand::with_name("mute")
                    .arg(
                        Arg::with_name("state")
                            .help("Mute state to set (on|off|toggle)")
                            .index(1),
                    )
                )
                .subcommand(SubCommand::with_name("mute"))
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
            let mut p = match PulseAudioSoundDevice::new(|| {}, true) {
                Err(_) => {
                    eprintln!("could not initialize an audio connector");
                    std::process::exit(2);
                }
                Ok(v) => v,
            };

            match sub.subcommand() {
                ("mute", Some(val)) => {
                    match val.value_of_lossy("state") {
                        Some(state) => {
                            let mute_state = parse_bool(&state, p.muted());
                            match p.set_muted(mute_state) {
                                Err(e) => {
                                    eprintln!("unable to change mute: {:}", e);
                                    std::process::exit(3);
                                }
                                Ok(_) => (),
                            };
                        },
                        None => {
                            eprintln!("Must specify mute state");
                            std::process::exit(1);
                        },
                    };
                },
                ("volume", Some(val)) => {
                    match val.value_of_lossy("add") {
                        Some(v) => {
                            let step: f32 = match v.parse() {
                                Err(e) => {
                                    eprintln!("unable to parse increment: {:}", e);
                                    std::process::exit(1);
                                }
                                Ok(v) => v,
                            };
                            let cap: Option<f32> = match val.value_of_lossy("max") {
                                None => None,
                                Some(v) => {
                                    match v.parse::<f32>() {
                                        Err(e) => {
                                            eprintln!("unable to parse max: {:}", e);
                                            std::process::exit(1);
                                        }
                                        Ok(v) => Some(v)
                                    }
                                },
                            };
                            match p.add_volume(step, cap) {
                                Err(e) => {
                                    eprintln!("unable to add volume: {:}", e);
                                    std::process::exit(3);
                                }
                                Ok(_) => (),
                            };
                        },
                        None => {
                            eprintln!("Must specify volume increment");
                            std::process::exit(1);
                        },
                    };
                },
                _ => {
                    println!("{}", p.volume());
                }
            }
        }
        _ => {
            eprintln!("must specify subcommand");
            std::process::exit(1);
        }
    }
}
