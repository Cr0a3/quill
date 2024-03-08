use crate::{print, conf::{self, Data, Package}};
use std::{io, env, fs, path::Path};
use PrintLib::colorize::Colorize;

pub fn new(name: &str, libary: bool, template: &str) -> std::io::Result<()>{
    println!("name:     {}", name);
    println!("libary:   {}", libary);
    println!("template: {}", template);

    let current_dir = env::current_dir()?;
    let path_str =  format!("{}/templates/{}.zip", current_dir.as_os_str().to_str().expect("couldn't get current dir"), template);
    
    let path = Path::new(&path_str);

    if !path.exists() {
        print::error("E", &format!("couldn't find template: '{}'", template));

        return Ok(());
    }

    let file = match fs::File::open(path) {
        Ok(f) => f,
        Err(e) => {
            print::error("", &format!("error while opening zip ({}): {}", path.display(), e.to_string()));
            return Ok(());
        },
    };

    let mut zip = zip::ZipArchive::new(file)?;

    // extracting zip
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;
        let outpath = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        if (*file.name()).ends_with('/') {
            fs::create_dir_all(&outpath).unwrap();
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    // rewrite config
    let cfg_path = Path::new( &format!("{}/cpack.toml", template) );
    

    // rename dir to project name
    fs::rename(template, name)?;

    println!("  - {} {}: '{name}'", "Created".color(0, 42, 71).bold(), match libary { true => "libary", false => "package" } );

    Ok(())
}