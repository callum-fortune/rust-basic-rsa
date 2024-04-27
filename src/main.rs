use num_traits::{Zero, One};
use num_bigint::{BigInt, RandBigInt};
use rand;
use rust_miller_rabin::miller_rabin::miller_rabin;

fn gcd(num_one: BigInt, num_two: BigInt) -> BigInt {
    if num_one.is_zero() {
        return num_two;
    }
    return gcd(num_two.clone() % num_one.clone(), num_one);
}


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

fn generate_key_pair() -> (BigInt, BigInt, BigInt) {

    let prime_bit_size = 256;

    let prime_one = generate_prime(prime_bit_size);
    let prime_two = generate_prime(prime_bit_size);

    let multiple_of_primes = &prime_one * &prime_two;

    let totient = (&prime_one - BigInt::one()) * (&prime_two - BigInt::one());

    println!("totient {}", totient);

    let mut exponent = BigInt::from(0);

    loop {
        println!("expo loop");
        let potential_exponent = generate_prime(20);

        if gcd(potential_exponent.clone(), totient.clone()) == BigInt::one() {
            exponent = potential_exponent;
            break;
        }
    }



    return (prime_one, prime_two, exponent)

}

fn encrypt() {

}

fn decrypt() {
    
}

fn main() {

    println!("result: {:?}", generate_key_pair())

}
