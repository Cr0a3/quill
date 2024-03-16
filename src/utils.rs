use std::{fs::{self, File}, io::{self, Write}, path::Path};
use zip::write::FileOptions;

pub fn extract_zip(extract_path: &String, file: File) -> io::Result<()> {

    let mut zip = zip::ZipArchive::new(file)?;

    // extracting zip
    for i in 0..zip.len() {
        let mut file = zip.by_index(i)?;

        let mut _fmt_path = String::new();

        let outpath = match file.enclosed_name() {
            Some(path) => {
                _fmt_path = format!("{extract_path}/{}", path.display());
                let pth = Path::new(&_fmt_path);
                pth
            },
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
    };

    Ok(())
}

pub fn zip(outpath: &String, dir: &String) -> io::Result<()> {
    let path = std::path::Path::new(outpath);
    let file = std::fs::File::create(path).unwrap();

    let mut zip = zip::ZipWriter::new(file);

    let options = FileOptions::default()
        .compression_method(zip::CompressionMethod::Stored)
        .unix_permissions(0o755);

    for file in fs::read_dir(dir)? {
        let file = file?;
        let path = file.path();
        let fmt = format!("{}", path.display());

        if path.is_dir() {
            zip.add_directory(fmt, Default::default())?;
        } else {
            zip.start_file(fmt, options)?;
            zip.write_all(fs::read_to_string(path)?.as_bytes())?;
        }
    }

    zip.finish()?;
    Ok(())
}