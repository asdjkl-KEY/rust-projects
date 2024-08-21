use crossterm::terminal::{Clear, ClearType};
use std::io::{self, Write};
use crate::spritesheet::process_sprites;

pub fn menu(options: [String; 2]) {
    let mut option = 0;
    let mut input = String::new();

    while option != 0 {
        Clear(ClearType::All);
        println!("Escoja una de las opciones");
        for opt in &options {
            println!("{}", opt);
            println!("> ");
        }
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");
        option = input.trim().parse().expect("Error al convertir la entrada a número");

        match option {
            0 => __exit(),
            1 => {
                Clear(ClearType::All);
                println!("Informa el nombre de la spritesheet:");
                let mut spritesheet = String::new();
                io::stdin().read_line(&mut spritesheet).expect("Error al leer la entrada");
                let spritesheet = spritesheet.trim();

                println!("Informa el ancho de los sprites:");
                let mut width = String::new();
                io::stdin().read_line(&mut width).expect("Error al leer la entrada");
                let width = width.trim().parse().expect("Error al convertir la entrada a número");

                println!("Informa el alto de los sprites:");
                let mut height = String::new();
                io::stdin().read_line(&mut height).expect("Error al leer la entrada");
                let height = height.trim().parse().expect("Error al convertir la entrada a número");

                process_sprites(spritesheet, width, height);
            },
            _ => println!("Opción inválida"),
        }
    }
}

fn __exit() {
    println!("Saliendo...");
}