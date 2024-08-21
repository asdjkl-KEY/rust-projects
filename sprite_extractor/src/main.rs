use sprite_extractor::menu_handler::menu;

fn main() {
    let options = [
        String::from("0 - Salir"),
        String::from("1 - Cortar automaticamente los sprites")
    ];
    print!("Hola mundo");
    menu(options);
}
