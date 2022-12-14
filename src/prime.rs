use num_bigint::BigUint;
use num_prime::RandPrime;

pub fn gen() -> BigUint {
    let mut rng = rand::thread_rng();
    rng.gen_prime(256, None)
}

#[cfg(test)]
mod test {
    use super::*;
    use num_prime::nt_funcs::is_prime;

    #[test]
    fn test_gen() {
        let p = gen();
        println!("{}", p);
        assert!(is_prime(&p, None).probably());
    }
}
