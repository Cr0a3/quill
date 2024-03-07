use std::env;
mod print;
mod runner;
mod api;
mod conf;

use crate::runner::*;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let mut options: Vec<String>  = vec!();

    for arg in args.clone() {
        if arg.chars().next() == Some('-') {
            options.push(arg);
        }
    }

    match args.len() - 1 { // - 1 for the actual args
        0 => {
            print::help();
        }

        1 => {
            let cmd: String = args[1].clone();

            //switch cmds
            match cmd.as_str() {
                "help" => {
                    print::help();
                }

                "build" => {
                    let _ = build::build("debug");
                }

                "clean" => {
                    clean::clean();
                }

                "new" => {
                    print::error("E002", &format!("expect package name {}", cmd));
                }

                "run" => {
                    run::run("debug");
                }

                "publish" => {
                    runner::publish();
                }

                "add" => {
                    print::error("E002", &format!("expected package name {}", cmd));
                }

                _ => {
                    if options.contains(&"-v".into()) || options.contains(&"--version".into()) {
                        print::version();
                    } else {
                        print::help();
                    }
                }
            }
        }

        _ => {
            let cmd: String = args[1].clone();
            let opt: String = args[2].clone();

            match cmd.as_str() {
                "help" => {
                    print::help_cmd(opt);
                }

                "build" => {
                    let _ = build::build(opt.as_str());
                }

                "new" => {
                    let lib: bool = args.contains(&"--lib".to_string());

                    let mut template: &str = match lib {
                        true => "lib_std",
                        false => "std"
                    };

                    if let Some(index) = args.iter().position(|x| x == &String::from("--template")) {
                        if index < args.len() - 1 {
                            template = args[index + 1].as_str();
                        } else {
                            print::error("E", "--template needs the template name after it");
                        }
                    }
                    let _ = new::new(opt.as_str(), lib, template);
                }

                "run" => {
                    run::run(opt.as_str());
                }

                "add" => {
                    runner::add(opt);
                }

                _ => {
                    if options.contains(&"-v".into()) || options.contains(&"--version".into()) {
                        print::version();
                    } else {
                        print::help();
                    }
                }
            }
        }
    }
}
