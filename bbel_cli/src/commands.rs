use clap::{ Command, Arg, ArgAction };

pub fn get_matches() -> clap::ArgMatches {
    let matches = Command::new("BBEL CLI")
        .version("0.1.0")
        .author("Jot4Ce <asdjkl.key@gmail.com>")
        .about("CLI para crear estructuras de proyectos")
        .subcommand(
            Command::new("structure")
            .about("Crea la estructura de un proyecto")
            .arg(
                Arg::new("type")
                .short('t')
                .long("type")
                .value_name("TYPE")
                .help("Tipo de estructura")
                .required(true)
                .action(ArgAction::Set)
            )
            .arg(
                Arg::new("root")
                .short('r')
                .long("root")
                .value_name("ROOT")
                .help("Directorio raíz")
                .required(false)
                .action(ArgAction::Set)
            )
        )
        .subcommand(
            Command::new("auto")
            .about("Guarda comandos para ejecutarlos automáticamente")
            .subcommand(
                Command::new("save")
                .about("Guarda comandos en un archivo")
                .arg(
                    Arg::new("name")
                    .short('n')
                    .long("name")
                    .value_name("NAME")
                    .help("Nombre del archivo")
                    .required(true)
                    .action(ArgAction::Set)
                )
            )
            .subcommand(
                Command::new("execute")
                .about("Ejecuta comandos guardados")
                .arg(
                    Arg::new("name")
                    .short('n')
                    .long("name")
                    .value_name("NAME")
                    .help("Nombre del archivo")
                    .required(true)
                    .action(ArgAction::Set)
                )
            )
            .subcommand(
                Command::new("list")
                .about("Lista los comandos guardados")
                .arg(
                    Arg::new("file")
                    .short('f')
                    .long("file")
                    .value_name("FILE")
                    .help("Nombre del archivo")
                    .required(false)
                    .action(ArgAction::Set)
                )
            )
        )
        .get_matches();

    matches
}