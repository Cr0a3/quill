use std::env;
mod print;
mod runner;
mod api;
mod conf;

pub fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() - 1 { // - 1 for the actual args
        1 => {
            let cmd: String = args[1].clone();

            //switch cmds
            match cmd.as_str() {
                "help" => {
                    print::help();
                }

                "build" => {
                    runner::build("debug");
                }

                "clean" => {
                    runner::clean("debug");
                }

                "new" => {
                    print::error("E002", &format!("expect package name {}", cmd));
                }

                "run" => {
                    runner::run("debug");
                }

                "publish" => {
                    runner::publish();
                }

                "add" => {
                    print::error("E002", &format!("expected package name {}", cmd));
                }

                _ => {
                    print::error("E001", &format!("invalid command {}", &cmd));
                }
            }
        }

        2 => {
            let cmd: String = args[1].clone();
            let opt: String = args[2].clone();

            match cmd.as_str() {
                "help" => {
                    print::help_cmd(opt);
                }

                "build" => {
                    runner::build(opt.as_str());
                }

                "clean" => {
                    runner::clean(opt.as_str());
                }

                "new" => {
                    runner::new(opt.as_str());
                }

                "run" => {
                    runner::run(opt.as_str());
                }

                "add" => {
                    runner::add(opt);
                }

                _ => {
                    print::error("E001", &format!("invalid command {}", cmd));
                }
            }
        }
        _ => {
            print::help();
        }
    }
}
