use num_bigint::{BigUint, RandBigInt};
use num_prime::nt_funcs::is_prime;
use num_prime::RandPrime;
use num_traits::One;
use tracing::trace;

struct DH {
    n: BigUint,
    g: BigUint,
    k: BigUint,
    secret: BigUint,
}
impl DH {
    fn new(n: BigUint, g: BigUint) -> Self {
        let mut rnd = rand::thread_rng();
        let k = rnd.gen_biguint(128);
        DH {
            n,
            g,
            k,
            secret: BigUint::one(),
        }
    }
    fn key(&self) -> BigUint {
        let k = self.g.modpow(&self.k, &self.n);
        trace!("k: {}", k);
        k
    }
    fn recv_key(&mut self, b: &BigUint) -> BigUint {
        let s = b.modpow(&self.k, &self.n);
        trace!("s: {}", s);
        self.secret = s.clone();
        s
    }
}

fn gen_pairs() -> Option<(BigUint, BigUint)> {
    let mut rnd = rand::thread_rng();
    let mut n: BigUint = BigUint::one();
    let mut n1: BigUint = BigUint::one();
    for _ in 0..3 {
        n = rnd.gen_prime(128, None);
        n1 = &n - 1u32;
        if is_prime(&(&n1 / 2u8), None).probably() {
            break;
        }
    }
    if n == n1 {
        trace!("生成大素数 n 失败");
        return None;
    }

    for i in 2..10u32 {
        let g = BigUint::from(i);
        if g.modpow(&n1, &n) == BigUint::one() {
            return Some((n, g));
        }
    }
    trace!("生成大素数 n 的本原元失败");
    None
}

#[cfg(test)]
#[test]
fn test_hm() {
    let (n, g) = gen_pairs().unwrap();
    let mut alice = DH::new(n.clone(), g.clone());
    let mut bob = DH::new(n.clone(), g.clone());
    let a = alice.key();
    let b = bob.key();
    assert_ne!(a, b);
    let s1 = alice.recv_key(&b);
    let s2 = bob.recv_key(&a);

    assert_eq!(s1, s2);
}
