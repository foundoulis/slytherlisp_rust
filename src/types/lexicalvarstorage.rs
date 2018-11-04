
use std::collections::HashMap;

use types::lispvalue::LispValue;

#[derive(Debug)]
pub struct LexicalVarStorage {
    environ: HashMap<String, LispValue>,
    local: HashMap<String, LispValue>
}
impl LexicalVarStorage {
    pub fn new(environ: HashMap<String, LispValue>) -> LexicalVarStorage {
        LexicalVarStorage {
            environ: environ,
            local: HashMap::new()
        }
    }
    pub fn fork(&self) -> HashMap<String, LispValue> {
        let mut new_map: HashMap<String, LispValue> = HashMap::new();
        for (key, value) in self.environ.iter() {
            new_map.insert((*key).clone(), value.clone());
        }
        for (key, value) in self.local.iter() {
            new_map.insert((*key).clone(), value.clone());
        }
        new_map
    }
    pub fn get(&self, key: String) -> Result<LispValue, String> {
        if self.local.contains_key(&key) {
            return Ok(self.local[&key].clone());
        } else if self.environ.contains_key(&key) {
            return Ok(self.environ[&key].clone());
        } else {
            return Err(format!("Undefined variable '{}'", key));
        }
    }
}