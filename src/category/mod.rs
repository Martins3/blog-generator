use super::Article;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;
// I have to admit, this way is really stupid

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

const PAPER_TEMPALTE: &str = "---\nCategory:\ntags:\ntitle:\ndate:";


fn append_date(){

}

