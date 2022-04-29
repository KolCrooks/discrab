use std::{collections::{HashSet, hash_map::DefaultHasher}, hash::{Hash, Hasher}};

use crate::api::ApplicationCommandOption;


pub fn options_equal(a: &Option<Vec<ApplicationCommandOption>>, b: &Option<Vec<ApplicationCommandOption>>) -> bool {
    if a.is_none() && b.is_none() {
        return true;
    }
    if a.is_none() != b.is_none() {
        return false;
    }

    let a = a.as_ref().unwrap();
    let b = b.as_ref().unwrap();

    if a.len() != b.len() {
        return false;
    }
    
    let hasher = |o: &ApplicationCommandOption| {
        let mut hasher = DefaultHasher::new();
        o.hash(&mut hasher);
        hasher.finish()
    };
    
    let ha: HashSet<u64> = a.iter().map(hasher).collect();
    let hb: HashSet<u64> = b.iter().map(hasher).collect();

    ha == hb
}