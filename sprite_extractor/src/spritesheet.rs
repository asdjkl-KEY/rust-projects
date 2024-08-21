use image::{DynamicImage, ImageReader};
use std::{fs, io::{Error, ErrorKind}, path::Path};

fn load_image(spritesheet_path: &str) -> Result<DynamicImage, Error> {
    let sheetpath = format!("../spritesheet/{}", spritesheet_path);
    let path = Path::new(&sheetpath);

    // Verifica si el archivo existe
    if !path.exists() {
        return Err(Error::new(ErrorKind::NotFound, format!("El archivo en {} no existe", path.display())));
    }

    // Intenta abrir el archivo de imagen
    let img = match ImageReader::open(path) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Ocurrió un error al abrir la imagen:\n{}", e);
            return Err(Error::new(ErrorKind::NotFound, e));
        },
    };

    // Intenta decodificar la imagen
    match img.decode() {
        Ok(decoded_img) => Ok(decoded_img),
        Err(e) => {
            eprintln!("Ocurrió un error al decodificar la imagen:\n{}", e);
            Err(Error::new(ErrorKind::InvalidData, e))
        },
    }
}

fn can_divide_image(img_width: u32, img_height: u32, sprite_width: u32, sprite_height: u32) -> Result<(), Error> {
    if sprite_width == 0 || sprite_height == 0 || img_width % sprite_width != 0 || img_height % sprite_height != 0 {
        return Err(Error::new(
            ErrorKind::InvalidInput,
            "La imagen no se puede dividir en sprites del tamaño dado.",
        ));
    }
    Ok(())
}

fn crop_sprite(image: &DynamicImage, x: u32, y: u32, sprite_width: u32, sprite_height: u32) -> DynamicImage {
    image.crop_imm(x, y, sprite_width, sprite_height)
}

fn save_sprite(sprite: &DynamicImage, path: &Path) {
    match sprite.save(path) {
        Ok(_) => {
            println!("Sprite {} guardado correctamente", path.display());
        },
        Err(e) => eprintln!("Ocurrió un error al guardar uno de los sprites:\n{}", e),
    }
}

pub fn process_sprites(spritesheet_path: &str, sprite_width: u32, sprite_height: u32) {
    let output_dir = Path::new("../sprites/");
    
    // Crea el directorio de salida si no existe
    if let Err(e) = fs::create_dir_all(output_dir) {
        eprintln!("Error al crear el directorio de sprites:\n{}", e);
        return;
    }

    // Carga la imagen
    let image = match load_image(spritesheet_path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Error al cargar la imagen: {}", e);
            return;
        }
    };

    let img_width = image.width();
    let img_height = image.height();

    // Verifica si la imagen puede ser dividida en sprites del tamaño dado
    if let Err(e) = can_divide_image(img_width, img_height, sprite_width, sprite_height) {
        eprintln!("Error de división de imagen: {}", e);
        return;
    }

    let num_sprites_x = img_width / sprite_width;
    let num_sprites_y = img_height / sprite_height;

    // Itera sobre cada sprite y lo guarda
    for y in 0..num_sprites_y {
        for x in 0..num_sprites_x {
            let left = x * sprite_width;
            let top = y * sprite_height;

            let sprite = crop_sprite(&image, left, top, sprite_width, sprite_height);

            // Crea un nombre de archivo para el sprite
            let filename = format!("{}/sprite_{}_{}.png", output_dir.display(), x, y);
            let path = Path::new(&filename);

            save_sprite(&sprite, path);
        }
    }
}