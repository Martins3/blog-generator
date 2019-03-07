use super::Article;
use std::cell::RefCell;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::Rc;

pub fn table(contend: &mut String, articles: &Vec<Rc<RefCell<Article>>>) {
    let mut set: HashSet<String> = HashSet::new();
    for i in articles {
        for x in &i.borrow().tags {
            set.insert(x.clone());
        }
    }
    // - [Platforms](#platforms)
    contend.push_str("## Tags\n\n");
    super::create_table(contend, set);
}

pub fn contend(contend: &mut String, articles: &Vec<Rc<RefCell<Article>>>) {
    let mut map: HashMap<String, Vec<Rc<RefCell<Article>>>> = HashMap::new();
    for i in articles {
        for x in &i.borrow().tags {
            match map.entry(x.clone()) {
                Entry::Vacant(e) => {
                    e.insert(vec![Rc::clone(i)]);
                }
                Entry::Occupied(mut e) => {
                    e.get_mut().push(Rc::clone(i));
                }
            }
        }
    }
    super::create_contend(contend, map);
}
