use crate::variantstruct::Genomecapture;
use std::collections::HashSet;
use std::error::Error;

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn hashsetref(hashvector: Vec<Genomecapture>) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut hashsetref: HashSet<String> = HashSet::new();
    let hashvector_borrow = hashvector;
    for i in hashvector_borrow.iter() {
        hashsetref.insert(i.generef.clone());
    }
    Ok(hashsetref)
}

pub fn hashsetalt(hashvector: Vec<Genomecapture>) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut hashsetalt: HashSet<String> = HashSet::new();
    let hashvector_alt = hashvector;
    for i in hashvector_alt.iter() {
        hashsetalt.insert(i.alt.clone());
    }
    Ok(hashsetalt)
}
