use std::collections::{HashMap, HashSet};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct SymbolId {
    id: usize,
}

impl SymbolId {
    fn first() -> SymbolId {
        SymbolId{id: 0}
    }

    fn increment(&mut self) {
        self.id += 1;
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum SymbolType {
    NonTerminal = 1,
    Terminal = 2,
    Empty = 3
}

impl SymbolType {
    fn index(self) -> usize {
        match self {
            SymbolType::NonTerminal => 1,
            SymbolType::Terminal => 2,
            SymbolType::Empty => 3,
        }
    }
}

pub struct RawTypedAlphabet {
    name_map: HashMap<String, (SymbolId, SymbolType)>,
    next_id: SymbolId,
}

impl RawTypedAlphabet {
    fn new() -> RawTypedAlphabet {
        RawTypedAlphabet {
            name_map: HashMap::new(),
            next_id: SymbolId::first(),
        }
    }

    fn insert(&mut self, sym_name: String, sym_type: SymbolType) {
        self.name_map.insert(sym_name, (self.next_id, sym_type));
        self.next_id.increment();
    }

    fn finalize(self) -> TypedAlphabet {
        let mut type_sets = Vec::with_capacity(3);
        type_sets[SymbolType::NonTerminal.index()] = HashSet::new();
        type_sets[SymbolType::Terminal.index()] = HashSet::new();
        type_sets[SymbolType::Empty.index()] = HashSet::new();
        
        let mut name_map = HashMap::new();
        let mut id_map = HashMap::new();
        for (name, &(sym_id, sym_type)) in &self.name_map {
            name_map.insert(name.clone(), sym_id.clone());
            id_map.insert(sym_id.clone(), (name.clone(), sym_type.clone()));
            type_sets[sym_type.index()].insert(sym_id.clone());
        }


        TypedAlphabet {
            name_map: name_map,
            id_map: id_map,
            type_sets: type_sets,
        }
    }
}

pub struct TypedAlphabet {
    id_map: HashMap<SymbolId, (String, SymbolType)>,
    name_map: HashMap<String, SymbolId>,
    type_sets: Vec<HashSet<SymbolId>>
}

impl TypedAlphabet {
    pub fn name_for_id<'a>(&'a self, id: SymbolId) -> Option<&'a String> {
        self.id_map
            .get(&id)
            .map(|result| &result.0)
    }

    pub fn type_for_id<'a>(&'a self, id: SymbolId) -> Option<SymbolType> {
        self.id_map
            .get(&id)
            .map(|result| result.1.clone())
    }

    pub fn id_for_name<'a>(&'a self, name: &str) -> Option<SymbolId> {
        self.name_map
            .get(name)
            .map(|result| result.clone())
    }

    pub fn get_type_set<'a>(&'a self, sym_type: SymbolType) -> &'a HashSet<SymbolId> {
        &self.type_sets[sym_type.index()]
    }

    pub fn is_type(&self, sym_id: &SymbolId, test_type: SymbolType) -> bool {
        self.type_sets[test_type.index()]
            .contains(sym_id)
    }
}
