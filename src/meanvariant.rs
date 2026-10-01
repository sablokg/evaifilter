use crate::variantstruct::Genomecapture;
use std::error::Error;

/*
Gaurav Sablok,
gsablok@proton.me
*/

pub fn meanquality(hashvector: Vec<Genomecapture>) -> Result<f64, Box<dyn Error>> {
    let mut meanquality: Vec<f64> = Vec::new();
    let hashvector_borrow = hashvector;
    for i in hashvector_borrow.iter() {
        meanquality.push(i.qual.clone().parse::<f64>().unwrap());
    }
    let mut meanvariant: f64 = 0.0 as f64;
    for i in meanquality.iter() {
        meanvariant += i;
    }
    let finallen = meanquality.len();
    Ok(meanvariant / finallen as f64)
}
