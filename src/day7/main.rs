#![feature(entry_and_modify)]

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
    weight: u32,
    total_weight: u32
}

impl Tower {
    fn new(name: &str) -> Tower {
        Tower {
            name: String::from(name),
            subs: Vec::new(),
            parent: Weak::new(),
            weight: 0,
            total_weight: 0,
        }
    }

    fn calc_total(&mut self) -> u32 {
        self.total_weight = self.weight;
        for sub in self.subs.iter() {
            self.total_weight += sub.borrow_mut().calc_total();
        }

        self.total_weight
    }

    fn find_unbalanced_sub(&self) -> Option<Rc<RefCell<Tower>>> {
        let values = self.sub_histogram();
        if values.len() > 1 {
            for (_, ref indices) in values.iter() {
                if indices.len() == 1 {
                    return Some(self.subs[indices[0]].clone());
                }
            }
        }
        None
    }

    fn sub_histogram(&self) -> HashMap<u32, Vec<usize>> {
        let mut values:HashMap<u32, Vec<usize>> = HashMap::new();
        for (index, sub) in self.subs.iter().enumerate() {
            values.entry(sub.borrow().total_weight)
                .and_modify(|indices| { (*indices).push(index); })
                .or_insert(vec![index]);
        }
        values
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
    let root = nodes.values().find(|tower| tower.borrow().parent.upgrade().is_none()).unwrap();
    {
        let mut root_mut = root.borrow_mut();
        println!("Root: {}", root_mut.name);

        // Calc total weights
        root_mut.calc_total();
    }

    let mut unbalanced_sub = root.clone();
    loop {
        let unbalanced_child = unbalanced_sub.borrow().find_unbalanced_sub();
        match unbalanced_child {
            Some(child) => {
                unbalanced_sub = child;
            },
            None        => {
                let parent = unbalanced_sub.borrow().parent.upgrade().unwrap();
                let histogram = parent.borrow().sub_histogram();
                let (balanced_weight, _) = histogram.iter().find(|&(_,v)| v.len() > 1).unwrap();
                let unbalanced_sub_borrow = unbalanced_sub.borrow();
                println!("Unbalanced sub: {}, weight: {}, should be: {}", 
                    unbalanced_sub_borrow.name,
                    unbalanced_sub_borrow.weight, 
                    balanced_weight-(unbalanced_sub_borrow.total_weight-unbalanced_sub_borrow.weight)); 
                break;
            }
        }
        
    };   
}