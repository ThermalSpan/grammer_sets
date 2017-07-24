use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct RawRule {
    pub head:String,
    pub alternate: Vec<String>
}

#[derive(Debug)]
pub struct RawGrammer {
    pub start: String,
    pub terminals:  Vec<String>,
    pub non_terminals: Vec<String>,
    pub rules: Vec<RawRule>
}

#[derive(Clone, Copy)]
enum SymbolType {
    NonTerminal,
    Terminal
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct SymbolId {
    id: usize
}

impl SymbolId {
    fn first() -> SymbolId {
        SymbolId{id: 0}
    }

    fn next(&mut self) {
        self.id += 1;
    }
}

struct Rule {
    head: SymbolId,
    alternate: Vec<SymbolId>
}

pub struct Grammer {
    id_map: HashMap<SymbolId, (String, SymbolType)>,
    terminals: HashSet<SymbolId>,
    non_terminals: HashSet<SymbolId>,
    rules: Vec<Rule> 
}

pub fn error_return<E>(error_count: usize) -> Option<E> {
    println!("There were {} errors", error_count);
    None
}

pub fn check_grammer(raw_grammer: &mut RawGrammer) -> Option<Grammer> {
    // As we progress we will count the errors. If there are any, then we will return None
    let mut error_count = 0;

    let mut name_to_id_map = HashMap::new();
    let mut id_to_name_map = HashMap::new();
    let mut terminals = HashSet::new();
    let mut non_terminals = HashSet::new();
    let mut rules = Vec::new();
    
    let mut next_id = SymbolId::first();
    for name in raw_grammer.terminals.drain(..) {
        // We only have one reserved name at the moment
        if name == "Empty" {
            println!("ERROR: Empty is a reserved name, it cannot be declared as a terminal");
            error_count += 1;
        }

        // Put the name in the map
        name_to_id_map.insert(name, (next_id, SymbolType::Terminal));
    
        // Put the id in the terminal set
        terminals.insert(next_id);
        
        // Increment the id
        next_id.next();
    }

    for name in raw_grammer.non_terminals.drain(..) {
        if name == "Empty" {
            println!("ERROR: Empty is a reserved name, it cannot be declared as a nonterminal");
            error_count += 1;
        }
 
        // Put the name in the map, save whatever we may have popped out
        let maybe_value  = name_to_id_map.insert(name.clone(), (next_id, SymbolType::NonTerminal));

        // If we kicked out an existing value, thats an error
        if let Some(_) =  maybe_value {
            println!("ERROR: {} is listed as both a terminal and a non-terminal", name);
            error_count += 1;
        } else {
            // If not then lets put the id in the non_terminals set
            non_terminals.insert(next_id);
        }

        next_id.next();
    }
    
    // Ensure the start symbol is properly setup as a nonterminal. I don't (currently) care if it
    // is also declared with the other non terminals, as long as it wasn't listed as a terminal
    let start_name = raw_grammer.start.clone();
    let start_id = next_id;
    let maybe_value = name_to_id_map.insert(start_name.clone(), (start_id, SymbolType::NonTerminal));
    if let Some((_, SymbolType::Terminal)) = maybe_value {
        println!("ERROR: {} is listed as both a terminal and the start symbol", start_name);
        error_count += 1;
    } else {
        non_terminals.insert(start_id);
    }
    next_id.next();
    
    // We also want a cheap way to get the name back from the id
    for (name, &(ref id, ref class)) in &name_to_id_map {
        id_to_name_map.insert(id.clone(), (name.clone(), class.clone()));
    }
   
    // As a sanity check, we should have caught this earlier, but make sure there is no overlap
    // between the terminals and the nonterminals 
    assert!(terminals.intersection(&non_terminals).count() == 0);

    // We need at least one rule that is based on the start symbol
    let mut found_start_rule = false;
    for rule in raw_grammer.rules.drain(..) {
        if rule.head == raw_grammer.start {
            found_start_rule = true; 
        }

        if rule.head == "Empty" {
            println!("ERROR: Empty cannot be the head of a rule");
            error_count += 1;
        }
        
        // Ensure the head is declared as a non terminal
        let mut head_id;
        match name_to_id_map.get(&rule.head) {
            Some(&(id, SymbolType::Terminal)) => {
                println!("ERROR: {} was listed as a terminal, but also used as a rule head", rule.head);
                error_count += 1;

                head_id = id;
            },
            Some(&(id, SymbolType::NonTerminal)) => {
                head_id = id;
            },
            None => {
                println!("ERROR: {} was used a rule head but was not declared as a non terminal", rule.head);
                error_count += 1;

                return error_return(error_count);
            }
        } 
   
        // Ensure all the alternates were delcared
        // if so, add their ids to the vec
        let mut alternate_ids = Vec::new();
        let alternate_length = rule.alternate.len();
        for name in rule.alternate {
            if name == "Empty" {
                if alternate_length != 1 {
                    println!("ERROR: Empty was not the only symbol in an a rule alternate");
                    error_count += 1;
                }
            } else {
                match name_to_id_map.get(&name) {
                    Some(&(id, _)) => {
                        alternate_ids.push(id);
                    }
                    None => {
                        println!("ERROR: {} was used in a rule alternate, but was not declared", name);
                        error_count += 1;
                    }
                }
            }
        }

        // Build an id based rule
        rules.push(
            Rule {
                head: head_id,
                alternate: alternate_ids
            }
        );
    }

    if ! found_start_rule {
        println!("ERROR: there was no rule with the start symbol, {}, as the head", start_name);
        error_count += 1;
    }

    if error_count != 0 {
        return error_return(error_count);
    }

    Some(Grammer {
        id_map: id_to_name_map,
        terminals: terminals,
        non_terminals: non_terminals,
        rules: rules
    })
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum SetEntry {
    Id(SymbolId),
    Empty,
    End
}

pub fn first_sets(grammer: &Grammer) {
    let mut set_map = HashMap::new();
    
    // All first sets start as empty
    for (id, _) in &grammer.id_map {
        set_map.insert(id, HashSet::new());
    }
   
    // For all terminals T, T is in First(T)
    for id in &grammer.terminals {
        set_map.get_mut(&id).unwrap().insert(SetEntry::Id(id.clone()));
    }

    let mut need_another_pass = true;
    while need_another_pass {
        need_another_pass = false;

        for rule in &grammer.rules {
            for rule_element in &rule.alternate {
                let element_first_set: Vec<SetEntry> = 
                    set_map.get(&rule_element).unwrap().iter()
                    .map(|x| x.clone())
                    .collect();

                let mut current_set = set_map.get_mut(&rule.head).unwrap();

                for id in &element_first_set {
                    let inserted_something_new = current_set.insert(id.clone());
                    need_another_pass = need_another_pass || inserted_something_new;
                }

                if ! id_follow_set.contains(&SetEntry::Empty) {
                    break;
                }
            }
        }
    }

    for (k, v) in &set_map {
        let &(ref h, _) = grammer.id_map.get(&k).unwrap();
        print!("First({}) = {{", h);
        for s in v {
            match *s {
                SetEntry::Id(id) => {
                    let &(ref n, _) = grammer.id_map.get(&id).unwrap();
                    print!("{},", n)
                }
                SetEntry::Empty => print!("Empty,"),
                SetEntry::End => print!("End,"),
            };
        }
        print!("}}\n");
    }}

pub fn follow_sets(grammer: &Grammer) {
    let mut set_map = HashMap::new();
    
    // All follow sets start as empty
    for (id, _) in &grammer.id_map {
        set_map.insert(id, HashSet::new());
    }
    
    // All terminal symbols follow set include themselves
    for id in &grammer.terminals {
        set_map.get_mut(&id).unwrap().insert(SetEntry::Id(id.clone()));
    }

    let mut need_another_pass = true;
    while need_another_pass {
        need_another_pass = false;

        for rule in &grammer.rules {

            for id in &rule.alternate {
                let id_follow_set: Vec<SetEntry> = set_map
                    .get(&id)
                    .unwrap().iter()
                    .map(|x| x.clone())
                    .collect();

                let mut current_set = set_map.get_mut(&rule.head).unwrap();

                for id in &id_follow_set {
                    let inserted_something_new = current_set.insert(id.clone());
                    need_another_pass = need_another_pass || inserted_something_new;
                }

                if ! id_follow_set.contains(&SetEntry::End) {
                    break;
                }
            }
        }
    }

    for (k, v) in &set_map {
        let &(ref h, _) = grammer.id_map.get(&k).unwrap();
        print!("Follow({}) = {{", h);
        for s in v {
            match *s {
                SetEntry::Id(id) => {
                    let &(ref n, _) = grammer.id_map.get(&id).unwrap();
                    print!("{},", n)
                }
                SetEntry::Empty => print!("Empty,"),
                SetEntry::End => print!("End,"),
            };
        }
        print!("}}\n");
    }
}
