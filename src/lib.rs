extern crate glob;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;

pub mod category;
mod tags;

use glob::glob;

pub struct Article {
    path: PathBuf,
    title: String,
    is_draft: bool,
    is_recent: bool,
    tags: Vec<String>,
    category: String,
    date: String,
}

const DIR: &str = "/home/shen/Core/Vn/";

impl Article {
    fn new(p: &PathBuf) -> Article {
        Article {
            path: p.clone(),
            title: String::new(),
            is_draft: false,
            is_recent: false,
            tags: vec![],
            category: String::new(),
            // maybe we can have a better implementaion for this
            date: String::new(),
        }
    }
}

fn break_line(s: &String) -> Result<(&str, &str), &'static str> {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b':' {
            return Ok((&s[0..i], &s[i + 1..]));
        }
    }

    return Err("Unrecognized pattern");
}

fn abs_dir(p: &str) -> String {
    let mut readme = DIR.to_owned();
    readme.push_str(p);
    readme
}

fn relative_dir(p: &String) -> &str {
    let len = DIR.len();
    &p[len..]
}

fn syntax_warning(entry: &PathBuf, line_num: i32, msg: &str) {
    eprintln!("Warn {} : {}:{}", msg, entry.display(), line_num);
}

pub fn gg() -> Result<(), Box<dyn std::error::Error>> {
    let readme = abs_dir("Readme.md");
    let readme_template = abs_dir("Readme-Template.md");

    let template = File::open(&readme_template);
    match template {
        Ok(_) => {
            fs::copy(readme_template, readme)?;
        }
        Err(error) => {
            eprintln!("Open Readme-Template.md warning {:?}", error);
        }
    }

    let mut articles: Vec<_> = vec![];
    // let regex = abs_dir("kernel/syscall.md");
    let regex = abs_dir("**/*.md");

    for entry in glob(regex.as_str())? {
        let entry: PathBuf = entry?;
        let a = Rc::new(RefCell::new(Article::new(&entry)));

        // println!("{}", entry.display());
        let file = File::open(&entry)?;

        let mut line_num = 0;
        for line in BufReader::new(file).lines() {
            let line: String = line?;
            if line_num == 0 {
                if line != "---" {
                    a.borrow_mut().is_draft = true;
                    syntax_warning(&entry, 0, "Missing header");
                    break;
                }
            } else if line == "---" {
                a.borrow_mut().is_draft = false;
                break;
            } else {
                let (function, parameter) = match break_line(&line) {
                    Ok(v) => v,
                    Err(err) => {
                        syntax_warning(&entry, line_num, "Syntax");
                        eprintln!("{}", err);
                        continue;
                    }
                };
                let function = function.trim();
                let parameter = parameter.trim();

                let split = parameter.split(" ");
                let vec: Vec<&str> = split.collect();
                if vec.is_empty() {
                    continue;
                }

                match function {
                    "Category" => {
                        if vec.len() != 0 {
                            a.borrow_mut().category = String::from(parameter);
                        }
                    }

                    "date" => {
                        // TODO maybe date is useful
                    }

                    "title" => {
                        // TODO trim the space
                        a.borrow_mut().title = String::from(parameter);
                    }

                    "tags" => {
                        for i in vec {
                            a.borrow_mut().tags.push(String::from(i));
                        }
                    }
                    _ => {
                        syntax_warning(&entry, line_num, "Unimplemented function");
                    }
                };
            }
            line_num = line_num + 1;
        }

        if a.borrow().title.is_empty() {
            let m = match a.borrow().path.file_name() {
                Some(s) => match s.to_str() {
                    Some(k) => k.to_string(),
                    None => String::from("empty title"),
                },
                None => String::from("empty title"),
            };

            a.borrow_mut().title = m;
        }

        if a.borrow().category.is_empty() {
            a.borrow_mut().category = String::from("miscellaneous");
        }

        if a.borrow().tags.is_empty() {
            // dir name used by default
            a.borrow_mut().tags.push(String::from("empty_tags"));
        }

        articles.push(Rc::clone(&a));
    }
    generate(articles)?;

    Ok(())
}

fn generate(articles: Vec<Rc<RefCell<Article>>>) -> Result<(), Box<dyn std::error::Error>> {
    let readme = abs_dir("Readme.md");

    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(readme)
        .unwrap();

    let mut contend = String::from("---\n");

    // just list categories
    category::table(&mut contend, &articles);
    tags::table(&mut contend, &articles);
    // tags table

    category::contend(&mut contend, &articles);
    tags::contend(&mut contend, &articles);
    // cacontend

    if let Err(e) = writeln!(file, "{}", contend) {
        eprintln!("Couldn't write to file: {}", e);
    }

    Ok(())
}

fn create_table(contend: &mut String, set: HashSet<String>) {
    for i in set {
        contend.push_str("- [");
        contend.push_str(i.as_ref());
        contend.push_str("](#");
        let mut l = i.clone();
        l.make_ascii_lowercase();
        contend.push_str(l.as_ref());
        contend.push_str(")\n");
    }
    contend.push_str("\n\n");
}

fn create_contend(contend: &mut String, map: HashMap<String, Vec<Rc<RefCell<Article>>>>) {
    for (category, articles) in &map {
        contend.push_str("#### ");
        contend.push_str(category);
        contend.push_str("\n");

        let mut num = 0;
        for i in articles {
            let path = i
                .borrow()
                .path
                .clone()
                .into_os_string()
                .into_string()
                .unwrap();
            let path = relative_dir(&path);
            contend.push_str(num.to_string().as_ref());
            contend.push_str(". [");
            contend.push_str(i.borrow().title.as_ref());
            contend.push_str("](./");
            contend.push_str(path.as_ref());
            contend.push_str(")\n");
            num = num + 1;
        }
        contend.push_str("\n");
    }
    contend.push_str("\n");
}
