use num::{BigUint, Integer};

pub fn lcm(a: &BigUint, b: &BigUint) -> BigUint {
    a.lcm(b)
}

#[cfg(test)]
mod test {
    use num::BigUint;

    #[test]
    fn lcm() {
        assert_eq!(super::lcm(&BigUint::from(4u64), &BigUint::from(6u64)), BigUint::from(12u64));
        assert_eq!(super::lcm(&BigUint::from(4u64), &BigUint::from(2u64)), BigUint::from(4u64));
    }
}