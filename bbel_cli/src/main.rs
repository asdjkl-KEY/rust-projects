// use std::io;
use bbel::mkdir::create_api_structure;
use bbel::trees::api_structure::get_api_structure;
use clap::{Command, Arg, ArgAction};

fn main() {
    let matches = Command::new("BBEL CLI")
        .version("0.1.0")
        .author("Jot4Ce asdjkl.key@gmail.com")
        .about("CLI de BBEL Studios")
        .subcommand(
            Command::new("structure")
            .about("Comando para crear una estructura de carpetas conforme al argumento")
            .arg(
                Arg::new("structure_name")
                .help("El tipo de estructura que se quiere crear")
                .short('s')
                .long("structure_name")
                .value_name("API_NODEJS")
                .action(ArgAction::Set)
            )
        ).get_matches();
    #[allow(unused)]
    match matches.subcommand() {
        Some(("structure", sub_m)) => {
            if let Some(option) = sub_m.get_one::<String>("structure_name") {
                if option == "api" {
                    create_api_structure(get_api_structure());
                }
            }
        }
        _ =>{
            println!("Otro");
        }
    }
}