use super::Article;
use chrono::prelude::*;
use chrono::DateTime;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::rc::Rc;

use std::process::Command;

pub fn table(contend: &mut String, articles: &Vec<Rc<RefCell<Article>>>) {
    let mut set: HashSet<String> = HashSet::new();
    for i in articles {
        set.insert(i.borrow().category.clone());
    }
    // - [Platforms](#platforms)
    contend.push_str("## Category\n\n");
    super::create_table(contend, set);
}

pub fn contend(contend: &mut String, articles: &Vec<Rc<RefCell<Article>>>) {
    let mut map: HashMap<String, Vec<Rc<RefCell<Article>>>> = HashMap::new();
    for i in articles {
        match map.entry(i.borrow().category.clone()) {
            Entry::Vacant(e) => {
                e.insert(vec![Rc::clone(i)]);
            }
            Entry::Occupied(mut e) => {
                e.get_mut().push(Rc::clone(i));
            }
        }
    }
    super::create_contend(contend, map);
}

// const PAPER_TEMPALTE: &str = "---\nCategory:\ntags:\ntitle:\ndate:";
// TODO how to create a enum for all kinds of category

pub enum ArticleType {
    BlogNotes,
    Paper,
    Blog,
}
pub fn new(t: ArticleType) -> Result<(), Box<dyn std::error::Error>> {
    let template = {
        let dt = Local::now();
        let template = match t {
            ArticleType::Paper => {
                let mut x = String::from("Category:\ntags:\ntitle:\nlink:\ndate:");
                x.push_str(dt.format("%Y-%m-%d %H:%M:%S").to_string().as_ref());
                x
            }

            ArticleType::BlogNotes => String::from("title:\nlink:"),

            ArticleType::Blog => {
                let mut x = String::from("Category:\ntags:\ntitle:\ndate:");
                x.push_str(dt.format("%Y-%m-%d %H:%M:%S").to_string().as_ref());
                x
            }
        };
        template
    };

    let buf = "/tmp/BlogGeneratorTemplate.md";
    let mut file: std::fs::File = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(buf)?;

    let comments = "# lines start with # will be omitted\n# the first blank line is end of header\n# every header info should reside in one line";

    writeln!(file, "{}", comments)?;
    writeln!(file, "{}", template)?;

    let mut child = Command::new("nvim")
        .arg(buf)
        .spawn()
        .expect("failed to execute editor");

    child.wait()?;

    let mut body = String::new();
    let mut header = HashMap::new();
    get_header_body(file, &mut body, &mut header)?;

    match t {
        ArticleType::BlogNotes => {
            let utc: DateTime<Local> = Local::now();
            let start = Local.datetime_from_str("2019-3-1 12:00:00", "%Y-%m-%d %H:%M:%S")?;
            let duration = (utc - start).num_weeks();

            let mut relative_file_name = String::from("nonsense/blog/");
            relative_file_name.push_str(duration.to_string().as_ref());
            relative_file_name.push_str(".md");
            let blog_notes = super::abs_dir(relative_file_name.as_ref());

            let file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(blog_notes);

            println!("{}", duration);
        }

        _ => {}
    }

    Ok(())
}
// fn first_word<'a>(s: &'a str) -> &'a str {

pub fn get_header_body(
    file: std::fs::File,
    contend: &mut String,
    map: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut is_header = true;

    for line in BufReader::new(file).lines() {
        let line: String = line?;

        if is_header {
            if line == "" {
                is_header = false;
                continue;
            } else if &line[..1] != "#" {
                let (function, parameter) = match super::break_line(&line) {
                    Ok(v) => v,
                    Err(err) => {
                        eprintln!("{}", err);
                        continue;
                    }
                };
                let function = String::from(function.trim());
                let parameter = String::from(parameter.trim());
                map.insert(function, parameter);
            }
        } else {
            contend.push_str(line.as_str());
            contend.push('\n');
        }
    }
    Ok(())
}

fn new_file() {}

fn append_file() {}

// TODO because blog is category with time and used by week
// create a tempalte in /tmp/
fn write_blog() {}
