use std::process::exit;

use PrintLib::error::ErrorFactory;
use PrintLib::colorize::Colorize;
use PrintLib::arg::UsageFactory;

pub fn help() {
    let mut arg_printer = UsageFactory::new("A package manager for C++ by Cr0a3", "quill");
    arg_printer.add_opt("-v, --version",        "Prints version info");
    arg_printer.add_opt("--lib",                "Makes the new package a libary");
    arg_printer.add_opt("--template (name)",    "Uses template (name)");
    arg_printer.add_opt("build (target)",       "Builds current package with target (target)");
    arg_printer.add_opt("clean",                "Cleans builds");
    arg_printer.add_cmd("new (name)",           "Creates new package with name (name)");
    arg_printer.add_opt("add",                  "Adds dependenci");
    arg_printer.add_opt("publish",              "Publish current package");
    arg_printer.add_opt("help",                 "Showes this help");
    arg_printer.add_opt("help (cmd/option)",    "Showes help for (cmd/option)");

    arg_printer.print();

}

pub fn help_cmd(cmd: String) {
    match cmd {
        "-v, --version" => {

        }

        "--lib" => {

        }
        
        "--template" => {

        }

        "build" => {

        }

        "clean" => {

        }

        "new" => {

        }

        "add" => {

        }

        "publish" => {

        }

        "help" => {

        }
        
        _ => {
            error("", &format!(" Command or option {} not found", cmd));
        }
    }
}

pub fn version() {
    println!("quill v{}", "1.1".bold());
}

pub fn error(ecode: &str, msg: &str) {
    let fab: ErrorFactory = ErrorFactory::new(ecode.to_string(), msg.to_string());
    fab.print();

    exit(1);
}