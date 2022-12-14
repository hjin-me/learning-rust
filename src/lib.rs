mod lcm;
mod paillier;
mod prime;
mod mod_inverse;

#[macro_use]
extern crate uint;

construct_uint! {
	pub struct U256(4);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        // Example modular arithmetic using bigint U256 primitives

        // imagine the field 0..p
        // where the p is defined below
        // (it's a prime!)
        let p = U256::from_dec_str("38873241744847760218045702002058062581688990428170398542849190507947196700873")
            .expect("p to be a good number in the example");

        // then, on this field,
        // (p-1) + (p+1) = 0

        // (p - 1) mod p
        let p_minus_1 = (p - 1) % p;
        // (p + 1) mod p
        let p_plus_1 = (p + 1) % p;
        // ((p - 1) mod p + (p + 1) mod p) mod p
        let sum = (p_minus_1 + p_plus_1) % p;
        assert_eq!(sum, 0.into());

        // on this field,
        // (p-1) + (p-1) = p-2
        let p_minus_1 = (p - 1) % p;
        let sum = (p_minus_1 + p_minus_1) % p;
        assert_eq!(sum, p - 2);

        // on this field,
        // (p-1) * 3 = p-3
        let p_minus_1 = (p - 1) % p;

        // multiplication is a series of additions
        let multiplicator = 3;
        let mul = {
            let mut result = p_minus_1;
            for _ in 0..multiplicator - 1 {
                result = (p_minus_1 + result) % p;
            }
            result
        };

        assert_eq!(mul, p - 3);
        let p = U256::from_dec_str("3")
            .expect("p to be a good number in the example");
        assert_eq!(p.pow(p), 27.into())

    }
}
