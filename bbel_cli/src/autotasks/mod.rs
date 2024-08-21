use colored::Colorize;
use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    io::{self, Write, Read},
    path::PathBuf,
    process::Command,
};
use dirs;

#[derive(Serialize, Deserialize, Debug)]
struct Data {
    items: Vec<String>,
}

fn get_route(nombre: &str) -> PathBuf {
    // Detecta la ruta del directorio de usuario multiplataforma
    let base_path = if cfg!(target_os = "windows") {
        dirs::data_local_dir().unwrap().join("bbel/cli/saved_commands")
    } else {
        dirs::home_dir().unwrap().join("bbel/cli/saved_commands")
    };

    if nombre == "__default" {
        base_path
    } else {
        base_path.join(format!("{}.json", nombre))
    }
}


pub fn save_commands(name: &str) -> io::Result<()> {
    let mut items: Vec<String> = Vec::new();
    let stdin = io::stdin();
    let mut input = String::new();

    println!("{}", "Escribe los comandos en orden que deseas guardar. Luego escribe '--exit' para terminar:\n".bright_blue());

    while stdin.read_line(&mut input)? != 0 {
        let trim_input = input.trim();
        if trim_input == "--exit" {
            break;
        }
        items.push(input.to_string().replace("\n", ""));
        input.clear();
    }

    let data = Data { items };
    let json = serde_json::to_string_pretty(&data)?;
    let path = get_route(name);
    std::fs::create_dir_all(path.parent().unwrap())?;
    let mut file = File::create(&path)?;
    file.write_all(json.as_bytes())?;

    println!("Comandos guardados en {}", path.display());

    Ok(())
}

pub fn execute_commands(name: &str) -> io::Result<()> {
    let path = get_route(name);
    let mut file = File::open(&path)?;
    let mut json_content = String::new();
    file.read_to_string(&mut json_content)?;

    // Deserializamos el contenido JSON
    let data: Data = serde_json::from_str(&json_content)?;

    // Ejecutamos los comandos
    for command in data.items {
        let mut parts = command.split_whitespace();
        let command = parts.next().unwrap();
        let args = parts.collect::<Vec<&str>>();
        let output = Command::new(command).args(args).output()?;
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    Ok(())
}

pub fn list_commands(name: Option<&str>) -> io::Result<()> {
    // Ruta fija correcta para los archivos de comandos guardados

    let path = if let Some(name) = name {
        // Si se especifica un nombre, obtenemos la ruta del archivo
        get_route(name)
    } else {
        // Si no se especifica un nombre, usamos el directorio base
        get_route("__default")
    };

    if path.is_file() {
        // Listar comandos en un archivo específico
        let mut file = File::open(&path)?;
        let mut json_content = String::new();
        file.read_to_string(&mut json_content)?;

        let data: Data = serde_json::from_str(&json_content)?;
        for (index, command) in data.items.iter().enumerate() {
            println!("Comando {}: {}", index + 1, command);
        }
    } else {
        if !path.exists() {
            println!("El directorio de comandos guardados no existe.");
            return Ok(());
        }

        // Leer todos los archivos JSON del directorio
        let entries = std::fs::read_dir(path.clone())?
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().map_or(false, |ext| ext == "json"))
            .collect::<Vec<_>>();

        if entries.is_empty() {
            println!("No se encontraron archivos de comandos.");
        } else {
            for entry in entries {
                let mut filename = entry.file_name().into_string().unwrap();
                filename = filename.replace(".json", "");
                println!("-{}", filename);
            }
        }
    }

    Ok(())
}
