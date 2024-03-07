use crate::{conf::{Data, Package}, print};
use std::{fs, io::Write};
use PrintLib::colorize::Colorize;

pub fn new(name: &str, libary: bool, template: &str) {
    println!("name:     {}", name);
    println!("libary:   {}", libary);
    println!("template: {}", template);
    
    /*// creating dirs
    if let Err(err) = fs::create_dir_all(format!("{name}/src")) {
        print::error(
            "E008",
             &format!("Error while creating target folder {}", err.to_string())
            );
    }

    if let Err(err) = fs::create_dir_all(format!("{name}/include")) {
        print::error(
            "E008",
             &format!("Error while creating folder {}", err.to_string())
            );
    }

    // creating files
    let mut conf = match fs::File::create(format!("{name}/cpack.toml")) {
        Ok(file) => file,
        Err(e) => {
            print::error("E", &format!("couldn't create file cpack.toml in {}: {}", name, e.to_string()));
            return;
        }
    };

    let mut main = match fs::File::create(format!("{name}/src/main.cpp")) {
        Ok(file) => file,
        Err(e) => {
            print::error("E", &format!("couldn't create file main.cpp in {name}/src: {}", e.to_string()));
            return;
        }
    };

    // writing files
    let data = Data {
        package: Package {
            name: name.into(),
            version: "".into(),
            author: "".into(),
            description: "".into(),
        }
    };

    let toml = match toml::to_string(
        &data ) {
        Ok(d) => { d },
        Err(e) => {
            print::error("E", &format!("couldn't serialize example data: {}", e.to_string()) );
            return;
        }
    };

    match conf.write(toml.as_bytes()) {
        Ok(_) => {},

        Err(e) => {
            print::error("E", &format!("error while writing cpack.toml: {}", e.to_string()));
            return;
        }
    }

    match main.write(
"#include <iostream>\n\nint main(int argc, char* args[]) {\n\tstd::cout << \"Hello, World!\";\n\treturn 0; \n}\n".as_bytes()
    ) {
        Ok(_) => {}
        Err(e) => {
            print::error("E", &format!("error while writing main.cpp: {}", e.to_string()));
            return;
        },
    }*/

    println!("  - {} {}: '{name}'", "Created".color(0, 42, 71).bold(), match libary { true => "libary", false => "package" } );
}