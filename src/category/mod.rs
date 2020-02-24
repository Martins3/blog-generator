use super::Article;
use chrono::prelude::*;
use chrono::DateTime;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
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
    Kernel,
    Sentence,
    Kchecklist, // accumulate checklist for kernel
}
pub fn new(t: ArticleType) -> Result<(), Box<dyn std::error::Error>> {
    let template = {
        let dt = Local::now();
        let mut template = match t {
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
            ArticleType::Kernel => {
                let x = String::from("question:\ntag:");
                x
            }

            ArticleType::Kchecklist => {
                let x = String::from("desc:");
                x
            }

            ArticleType::Sentence=> {
                let x = String::from("sentence:\nnote:");
                x
            }
        };
        template.push_str("\n---");
        template
    };

    let buf = "/tmp/BlogGeneratorTemplate.md";
    let mut file: std::fs::File = OpenOptions::new()
        .read(true)
        .truncate(true)
        .write(true)
        .create(true)
        .open(buf)?;

    writeln!(file, "{}", template)?;

    let mut child = Command::new("/usr/bin/nvim")
        .arg("+4")
        .arg(buf)
        .spawn()
        .expect("failed to execute editor");
    child.wait()?;

    let mut body = String::new();
    let mut header = HashMap::new();
    let mut raw_header = String::new();
    get_header_body(&mut raw_header, &mut body, &mut header)?;

    println!("header {:?} \ncontend {}", header, body);

    match t {
        ArticleType::BlogNotes => {
            let utc: DateTime<Local> = Local::now();
            let start = Local.datetime_from_str("2019-3-1 12:00:00", "%Y-%m-%d %H:%M:%S")?;
            let duration = (utc - start).num_weeks();

            let mut relative_file_name = String::from("nonsense/blog/");
            relative_file_name.push_str(duration.to_string().as_ref());
            relative_file_name.push_str(".md");
            let blog_notes = super::abs_dir(relative_file_name.as_ref());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(blog_notes)?;

            let mut note = String::new();
            note.push_str("## [");
            match header.get("title") {
                Some(a) => note.push_str(a.as_str()),
                None => note.push_str("Miss Title"),
            }

            note.push(']');
            note.push('(');

            match header.get("link") {
                Some(a) => note.push_str(a.as_str()),
                None => note.push_str("https://martins3.github.io"),
            }

            note.push(')');
            note.push('\n');
            note.push_str(body.as_str());

            writeln!(file, "{}", note)?;
        }

        ArticleType::Paper => {
            let mut relative_file_name = String::from("papers/");

            match header.get("title") {
                Some(a) => relative_file_name.push_str(a.as_str()),
                None => {
                    let t = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
                    relative_file_name.push_str(t.as_ref());
                    eprintln!("Title not found !");
                }
            }
            relative_file_name.push_str(".md");
            let blog_notes = super::abs_dir(relative_file_name.as_ref());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(blog_notes)
                .expect("Create file failed !");

            let mut note = String::new();
            note.push_str("---\n");
            note.push_str(raw_header.as_ref());
            note.push_str("---\n");
            note.push_str(body.as_str());
            writeln!(file, "{}", note)?;
        }

        ArticleType::Kernel => {
            // @todo sort tags of questions
            let blog_notes = super::abs_dir(String::from("kernel/question.md").as_ref());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(blog_notes)?;

            let mut note = String::new();
            note.push_str("#### (");
            match header.get("tag") {
                Some(a) => note.push_str(a.as_str()),
                None => note.push_str("unsorted"),
            }

            note.push_str(") ");

            match header.get("question") {
                Some(a) => note.push_str(a.as_str()),
                None => {
                    return Ok(()); // no question, just return !
                }
            }

            note.push('\n');
            note.push_str(body.as_str());
            writeln!(file, "{}", note)?;
        }

        ArticleType::Kchecklist => {
            // @todo sort tags of questions
            let blog_notes = super::abs_dir(String::from("kernel/kchecklist.md").as_ref());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(blog_notes)?;

            let mut note = String::new();
            note.push_str("## ");
            match header.get("desc") {
                Some(a) => note.push_str(a.as_str()),
                None => note.push_str("Not description"),
            }

            note.push('\n');
            note.push_str(body.as_str());
            writeln!(file, "{}", note)?;
        }

        ArticleType::Sentence => {
            // @todo sort tags of questions
            let blog_notes = super::abs_dir(String::from("TG/sentence.md").as_ref());

            let mut file = OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(blog_notes)?;

            let mut note = String::new();
            match header.get("note") {
                Some(a) => note.push_str(a.as_str()),
                None => note.push_str("unsorted"),
            }

            note.push_str("\n");

            match header.get("sentence") {
                Some(a) => note.push_str(a.as_str()),
                None => note.push_str("empty sentence"),
            }

            note.push('\n');
            note.push_str(body.as_str());
            writeln!(file, "{}", note)?;
        }

        _ => {
            eprintln!("{}", "unimplemented !");
        }
    }


    Ok(())
}

fn get_header_body(
    raw_header: &mut String,
    contend: &mut String,
    map: &mut HashMap<String, String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let buf = "/tmp/BlogGeneratorTemplate.md";
    let file: std::fs::File = OpenOptions::new().read(true).open(buf)?;
    let mut is_header = true;

    for line in BufReader::new(file).lines() {
        let line: String = line?;

        if is_header {
            if line == "---" {
                is_header = false;
                continue;
            } else if &line[..1] != "#" {
                raw_header.push_str(line.as_str());
                raw_header.push('\n');

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
