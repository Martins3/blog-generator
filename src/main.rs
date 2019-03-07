use blog;
use std::process;
extern crate clap;

use clap::{App, SubCommand};

fn main() {
    let matches = App::new("IGNB")
        .version("0.1.0")
        .author("Bachelar Hu. <hubachelar@gmail.com>")
        .about("Lightweight blog framework based on github")
        .args_from_usage("-d, --directory=[DIR] 'blog source tree directory'")
        .subcommand(
            SubCommand::with_name("create")
                .about("create new papers")
                .arg_from_usage("-p, --paper 'create paper reading notes'"),
        )
        .get_matches();

    if let Some(d) = matches.value_of("directory") {
        println!("Blog directory : {}", d);
        eprintln!("Blog directory is not implemented");
    }

    if let Some(matches) = matches.subcommand_matches("create") {
        if matches.is_present("paper") {
            println!("Create a paper tempalete");
            // blog::category::paper::create_template();
        } else {
            println!("Create a normal blog");
            // blog::category::create_template();
        }
    }

    if let Err(e) = blog::gg() {
        println!("Application error: {}", e);
        process::exit(1);
    }
}
