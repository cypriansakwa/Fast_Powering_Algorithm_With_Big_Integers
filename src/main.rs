use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::str::FromStr;

fn main() {
    let base = BigUint::from_str("279738749834588626262626").unwrap();
    let exponent = BigUint::from_str("23488975663434343434242424").unwrap();
    let modulus = BigUint::from_str("313111111111111111222222222").unwrap();

    let result = mod_exp(&base, &exponent, &modulus);

    println!("{}^{} mod {} = {}", base, exponent, modulus, result);
}

fn mod_exp(base: &BigUint, exp: &BigUint, modulus: &BigUint) -> BigUint {
    if modulus == &BigUint::one() {
        return BigUint::zero();
    }
    let mut result = BigUint::one();
    let mut base = base % modulus;
    let mut exp = exp.clone();

    while exp > BigUint::zero() {
        if &exp % 2u32 == BigUint::one() {
            result = (result * &base) % modulus;
        }
        exp = exp >> 1;
        base = (&base * &base) % modulus;
    }
    result
}


