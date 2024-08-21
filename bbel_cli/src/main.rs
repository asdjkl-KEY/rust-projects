use bbel::{
    commands::get_matches,
    mkdir::make_dirs,
    structures::get_structure,
    autotasks::{save_commands, execute_commands,list_commands},
};
#[allow(unused_imports)]
use colored::*;

fn main() {
    let matches = get_matches();
    
    match matches.subcommand() {
        Some(("structure", sub_m)) => {
            let structure_type = sub_m.get_one::<String>("type").unwrap();
            let root = sub_m.get_one::<String>("root").map(String::as_str);
            let structure = get_structure(&structure_type, Some(root.unwrap_or("_src")));
            match make_dirs(structure) {
                Ok(_) => println!("Estructura creada"),
                Err(e) => eprintln!("Error creando estructura: {}", e),
            }
        },
        Some(("auto", sub_m)) => {
            match sub_m.subcommand() {
                Some(("save", sub_m)) => {
                    let name = sub_m.get_one::<String>("name").unwrap();
                    match save_commands(&name) {
                        Ok(_) => println!("Comandos guardados"),
                        Err(e) => eprintln!("Error guardando comandos: {}", e),
                    }
                },
                Some(("execute", sub_m)) => {
                    let name = sub_m.get_one::<String>("name").unwrap();
                    match execute_commands(&name) {
                        Ok(_) => println!("Comandos ejecutados"),
                        Err(e) => eprintln!("Error ejecutando comandos: {}", e),
                    }
                },
                Some(("list", sub_m)) => {
                    let name = sub_m.get_one::<String>("file").map(String::as_str);
                    match list_commands(name) {
                        Ok(_) => println!("Comandos listados"),
                        Err(e) => eprintln!("Error listando comandos: {}", e),
                    }
                }
                _ => println!("Sub comando no reconocido"),
            }
        },
        _ => println!("Comando no reconocido"),
    }
}
