use std::fs;
use std::path::Path;

pub fn create_api_structure(paths: Vec<String>) -> std::io::Result<()> {
    for path_str in paths {
        let path = Path::new(&path_str);

        if path_str.ends_with('/') {
            // Si la ruta termina con '/', es un directorio
            let dir_path = path.to_path_buf();
            fs::create_dir_all(&dir_path)?;
        } else {
            // Si no termina con '/', es un archivo
            let parent_dir = path.parent().unwrap_or(Path::new(""));
            // Primero creamos los directorios padres
            fs::create_dir_all(parent_dir)?;
            // Luego creamos el archivo
            fs::File::create(&path)?;
        }
    }

    Ok(())
}