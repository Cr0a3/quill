use std::process::exit;

use PrintLib::error::ErrorFactory;
use PrintLib::colorize::Colorize;

pub fn help() {
    println!("{}", "C's package manager by Cr0a3".color(0, 42, 71).bold());
    println!();
    println!("{} {}", "Usage:".underline(), "quill [OPTIONS] [CMD]".color(0, 42, 71).bold());
    println!();
    println!("{}", "Options:");
    println!("   {}\t {} {}",       "-v, --version".color(59, 4, 105),      "|".color(227, 173, 25),    "Prints version info"                           );
    println!("   {}\t\t {} {}",     "--lib".color(59, 4, 105),              "|".color(227, 173, 25),    "Makes the new package a libary"                );
    println!("   {}\t {} {}",       "--template (name)".color(59, 4, 105),  "|".color(227, 173, 25),    "Uses template (name)"                          );
    println!();
    println!("{}", "Common commands:");
    println!("   {}\t {} {}",       "build (target)".color(59, 4, 105),     "|".color(227, 173, 25),    "Builds current package with target (target)"   );
    println!("   {}\t\t {} {}",     "clean".color(59, 4, 105),              "|".color(227, 173, 25),    "Cleans builds"                                 );
    println!("   {}\t\t {} {}",     "new (name)".color(59, 4, 105),         "|".color(227, 173, 25),    "Creates new package with name (name)"          );
    println!("   {}\t\t\t {} {}",   "add".color(59, 4, 105),                "|".color(227, 173, 25),    "Adds dependenci"                               );
    println!("   {}\t\t {} {}",     "publish".color(59, 4, 105),            "|".color(227, 173, 25),    "Publish current package"                       );
    println!("   {}\t\t\t {} {}",   "help".color(59, 4, 105),               "|".color(227, 173, 25),    "Showes this help"                              );
    println!("   {}\t {} {}",       "help (cmd/option)".color(59, 4, 105),  "|".color(227, 173, 25),    "Showes help for (cmd/option)"                  );
}

pub fn help_cmd(cmd: String) {
    match cmd {
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