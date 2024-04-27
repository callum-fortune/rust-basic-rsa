use num_bigint::{BigInt, RandBigInt};
use rand;
use rust_miller_rabin::miller_rabin::miller_rabin;

fn generate_prime(bit_size: u64) -> BigInt {

    let mut rng = rand::thread_rng();

    let mut result: BigInt = BigInt::from(0);

    loop {
        let potential_prime = rng.gen_bigint(bit_size);
        if miller_rabin(&potential_prime){
            result = potential_prime;
            break;
        }

    }

    return result;

}

fn generate_key_pair() -> (BigInt, BigInt) {

    let prime_bit_size = 256;

    let prime_one = generate_prime(prime_bit_size);
    let prime_two = generate_prime(prime_bit_size);

    return (prime_one, prime_two)

    // let prime = confirmed_prime;

    // println!("{}", prime);

    // return ((prime, confirmed_prime), (confirmed_prime, confirmed_prime));

}

fn encrypt() {

}

fn decrypt() {
    
}

fn main() {

    println!("result: {:?}", generate_key_pair())

}
