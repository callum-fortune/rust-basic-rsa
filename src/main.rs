use num_bigint::{BigUint, RandBigInt};
use rand::prelude::*;

fn generate_key_pair() {

}

fn encrypt() {

}

fn decrypt() {
    
}

fn main() {

    let mut rng = rand::thread_rng();

    let rand = rng.gen_bigint(4096);

    println!("{}", rand);

}
