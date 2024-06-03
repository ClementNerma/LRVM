use std::hash::{DefaultHasher, Hash, Hasher};

pub fn gen_aux_id(name: &'static str) -> u64 {
    let mut hasher = DefaultHasher::new();
    name.hash(&mut hasher);
    hasher.finish()
}
