use PrintLib::Error::ErrorFactory;

pub fn help() {

}

pub fn help_cmd(cmd: String) {
    match cmd {
        _ => {}
    }
}

pub fn error(ecode: &str, msg: &str, line: &str) {
    let mut fab: ErrorFactory = ErrorFactory::new(ecode.to_string(), msg.to_string());
    fab.add_code_line(line.to_string(), false, 0, false);
    fab.print()
}