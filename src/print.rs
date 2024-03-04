use std::process::exit;

use PrintLib::error::ErrorFactory;

pub fn help() {

}

pub fn help_cmd(cmd: String) {
    match cmd {
        _ => {}
    }
}

pub fn error(ecode: &str, msg: &str) {
    let fab: ErrorFactory = ErrorFactory::new(ecode.to_string(), msg.to_string());
    fab.print();

    exit(1);
}