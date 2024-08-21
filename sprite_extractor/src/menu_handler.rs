use crossterm::terminal::{Clear, ClearType};

pub fn menu() {
    let options = [
        String::from("0 - Salir"),
        String::from("1 - Cortar automaticamente los sprites")
    ];
    let mut option: u32 = 0;

    
    println!("> ");

    while option != 0 {
        Clear(ClearType::All);
        println!("Escoja una de las opciones");
        for opt in &options {
            println!("{}", opt);
        }
    }
}