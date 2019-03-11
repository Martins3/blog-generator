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
            SubCommand::with_name("new")
                .about("create new article")
                .arg_from_usage("-p, --paper 'create paper reading notes'")
                .arg_from_usage("-n, --note 'write notes'"),
        )
        .get_matches();

    if let Some(d) = matches.value_of("directory") {
        println!("Blog directory : {}", d);
        eprintln!("Blog directory is not implemented");
    }

    if let Some(matches) = matches.subcommand_matches("new") {
        let mut t = blog::category::ArticleType::Blog;
        if matches.is_present("paper") {
            t = blog::category::ArticleType::Paper;
            println!("Create a paper tempalete");
        } else if matches.is_present("note") {
            t = blog::category::ArticleType::BlogNotes;
            println!("Create a new note");
        }

        if let Err(e) = blog::category::new(t) {
            println!("Create temaplate error: {}", e);
            process::exit(1);
        }
    } else {
        if let Err(e) = blog::gg() {
            println!("Application error: {}", e);
            process::exit(1);
        }
    }
}
