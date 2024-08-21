use std::fs;
use std::path::Path;
use std::io;
use colored::Colorize;

pub fn make_dirs(dirs: Vec<String>) -> io::Result<()> {
    for dir in dirs {
        let path = Path::new(&dir);

        if dir.ends_with("$") {
            let mut dir = dir.replace("$", "");
            dir = dir.trim().to_string();
            let mut parts = dir.split("/").collect::<Vec<&str>>();
            let filename = parts[parts.len() - 1];
            parts.pop();
            let parent = parts.join("/");
            let filedir = Path::new(&parent).join(&filename);

            // Crear directorios intermedios
            match fs::create_dir_all(&parent) {
                Ok(_) => {
                    let msg = format!("Creado el directorio '{}'", parent);
                    println!("{}", msg.green());
                },
                Err(e) => {
                    let msg = format!("Error al crear el directorio '{}'", parent);
                    eprintln!("{}: {}", msg.red(), e);
                },
            }

            // Crear el archivo
            match fs::File::create(&filedir) {
                Ok(_) => {
                    let msg = format!("Creado el archivo '{}'", &filedir.to_str().unwrap());
                    println!("{}", msg.green());
                },
                Err(e) => {
                    let msg = format!("Error al crear el archivo '{}'", &filedir.to_str().unwrap());
                    eprintln!("{}: {}", msg.red(), e);
                },
            }
            
        } else {
            match fs::create_dir_all(path) {
                Ok(_) => {
                    let msg = format!("Creado el directorio '{}'", dir);
                    println!("{}", msg.green());
                },
                Err(e) => {
                    let msg = format!("Error al crear el directorio '{}'", dir);
                    eprintln!("{}: {}", msg.red(), e);
                },
            }
        }
    }
    Ok(())
}
