use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("ancestry")
        .version("0.1.0")
        .author("Brian Moore")
        .about("Data Engineers Rust Tool Kit")
        .subcommand(
            SubCommand::with_name("lineage")
                .about("Tracks and displays the lineage of your SQL data")
                .arg(Arg::with_name("INPUT")
                    .help("Sets the input file to use")
                    .required(true)
                    .index(1)),
        )
        .subcommand(
            SubCommand::with_name("build")
                .about("Builds the SQL transformation process")
                .arg(Arg::with_name("config")
                    .short('c')
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true)),
        )
        .get_matches();

        match matches.subcommand() {
            Some(("lineage", sub_m)) => {
                // Use sub_m to access the arguments for the lineage subcommand
                if let Some(input) = sub_m.value_of("INPUT") {
                    println!("Processing lineage for: {}", input);
                    // Implement your lineage functionality here
                }
            },
            Some(("build", sub_m)) => {
                // Use sub_m to access the arguments for the build subcommand
                if let Some(config) = sub_m.value_of("config") {
                    println!("Building with config: {}", config);
                    // Implement your build functionality here
                } else {
                    println!("Building with default settings");
                    // Implement your build functionality here
                }
            },
            None => {
                println!("No subcommand was provided. Please specify 'lineage' or 'build'.");
            }
            _ => unreachable!(), // Handling the None case
        }
        
}
