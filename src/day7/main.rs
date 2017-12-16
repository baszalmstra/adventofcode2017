use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::rc::Rc;
use std::rc::Weak;
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::hash_map::Entry;

struct Tower {
    name: String,
    parent: Weak<RefCell<Tower>>,
    subs: Vec<Rc<RefCell<Tower>>>,
    weight: u32
}

impl Tower {
    fn new(name: &str) -> Tower {
        Tower {
            name: String::from(name),
            subs: Vec::new(),
            parent: Weak::new(),
            weight: 0
        }
    }
}

fn find_or_get_tower(nodes:&mut HashMap<String, Rc<RefCell<Tower>>>, name:String) ->
    std::rc::Rc<std::cell::RefCell<Tower>> {
    match nodes.entry(name) {
        Entry::Occupied(tower)     => tower.into_mut().clone(),
        Entry::Vacant(v)           => {
            let tower = Rc::new(RefCell::new(Tower::new(v.key())));
            v.insert(tower.clone());
            tower
        }
    }
}

fn main() {
    let f = File::open("inputs/day7.txt").expect("input file not found");
    let line_iter = BufReader::new(f).lines().map(|line| line.unwrap());

    let mut nodes:HashMap<String, Rc<RefCell<Tower>>> = HashMap::new();
    for line in line_iter {
        let mut words_iter = line.split(" ");
        let name = String::from(words_iter.nth(0).unwrap());
        let tower = find_or_get_tower(&mut nodes, name);
        let mut tower_mut = tower.borrow_mut();

        // Get the weight of the tower
        let weight = words_iter.nth(0).unwrap().trim_matches('(').trim_matches(')').parse::<u32>().unwrap();
        tower_mut.weight = weight;

        // Get the sub towers
        tower_mut.subs = words_iter.skip(1)
            .map(|sub| sub.trim())
            .map(|sub| sub.trim_matches(','))
            .map(|sub| {
                let sub = find_or_get_tower(&mut nodes, String::from(sub));
                {
                    let mut sub_mut = sub.borrow_mut();
                    sub_mut.parent = Rc::downgrade(&tower);
                }
                sub
            })
            .collect();
    }

    // Find the root
    let root = nodes.values().find(|tower| tower.borrow().parent.upgrade().is_none()).unwrap().borrow();
    println!("Root: {}", root.name);
}