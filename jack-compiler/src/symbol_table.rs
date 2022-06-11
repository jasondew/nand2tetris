use crate::parser::DataType;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind {
    Variable,
    Argument,
    Field,
    Static,
}

#[derive(Debug)]
pub struct SymbolTable {
    data: HashMap<String, SymbolData>,
}

#[derive(Debug)]
pub struct SymbolData {
    pub id: u8,
    pub data_type: DataType,
    pub kind: Kind,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    pub fn add(&mut self, name: String, kind: Kind, data_type: DataType) {
        let id = self.count_of(kind) as u8;
        let symbol_data = SymbolData {
            id,
            kind,
            data_type,
        };

        self.data.insert(name, symbol_data);
    }

    pub fn lookup(&self, name: &String) -> Option<&SymbolData> {
        self.data.get(name)
    }

    pub fn non_static_count(&self) -> usize {
        self.data
            .iter()
            .filter(|(_name, symbol_data)| symbol_data.kind != Kind::Static)
            .count()
    }

    pub fn count_of(&self, kind: Kind) -> usize {
        self.data
            .iter()
            .filter(|(_name, symbol_data)| symbol_data.kind == kind)
            .count()
    }
}
