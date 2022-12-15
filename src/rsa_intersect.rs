use crate::mod_inverse::mod_inverse;
use num_bigint::{BigUint, RandBigInt};
use rsa::{PublicKeyParts, RsaPrivateKey, RsaPublicKey};

trait Host {
    fn recv_rsa_key(&mut self, key: &RsaPublicKey) -> Vec<BigUint>;
    fn compare(&mut self, a: Vec<BigUint>, b: Vec<BigUint>) -> Vec<u128>;
}
trait Guest {
    fn exchange(&mut self, y: Vec<BigUint>) -> (Vec<BigUint>, Vec<BigUint>);
}
struct DataSetHost {
    data: Vec<u128>,
    pub_key: RsaPublicKey,
    ri: BigUint,
}
struct DataSetGuest {
    data: Vec<u128>,
    priv_key: RsaPrivateKey,
}

impl Host for DataSetHost {
    fn recv_rsa_key(&mut self, key: &RsaPublicKey) -> Vec<BigUint> {
        self.pub_key = key.clone();

        let e = BigUint::from_bytes_be(key.e().to_bytes_be().as_slice());
        let n = BigUint::from_bytes_be(key.n().to_bytes_be().as_slice());

        let mut rng = rand::thread_rng();
        let ri = rng.gen_biguint_below(&BigUint::from(100u32));
        self.ri = ri.clone();

        let mut data: Vec<BigUint> = Vec::new();
        for d in self.data.iter() {
            data.push((ri.modpow(&e, &n) * BigUint::from(d.clone())) % &n);
        }
        data
    }
    fn compare(&mut self, za: Vec<BigUint>, zb: Vec<BigUint>) -> Vec<u128> {
        let mut data: Vec<u128> = Vec::new();
        let n = BigUint::from_bytes_be(self.pub_key.n().to_bytes_be().as_slice());
        let mi = mod_inverse(&self.ri, &n).unwrap();

        for (i, a) in za.iter().enumerate() {
            let d = (a * &mi) % &n;
            for b in zb.iter() {
                if &d == b {
                    data.push(self.data[i].clone());
                }
            }
        }
        data
    }
}

impl Guest for DataSetGuest {
    fn exchange(&mut self, y: Vec<BigUint>) -> (Vec<BigUint>, Vec<BigUint>) {
        let d = BigUint::from_bytes_be(self.priv_key.d().to_bytes_be().as_slice());
        let n = BigUint::from_bytes_be(self.priv_key.n().to_bytes_be().as_slice());
        let mut za: Vec<BigUint> = Vec::new();
        for v in y.iter() {
            za.push(v.modpow(&d, &n));
        }

        let mut zb: Vec<BigUint> = Vec::new();
        for v in self.data.iter() {
            zb.push(BigUint::from(v.clone()).modpow(&d, &n));
        }
        (za, zb)
    }
}

#[cfg(test)]
#[test]
fn test_intersect() {
    let priv_key = RsaPrivateKey::new(&mut rand::thread_rng(), 1024).unwrap();
    let pub_key = priv_key.to_public_key();
    let mut host = DataSetHost {
        data: vec![1, 2, 4, 5, 6, 7],
        pub_key,
        ri: BigUint::from(3u32),
    };
    let mut guest = DataSetGuest {
        data: vec![1, 2, 6, 7, 10],
        priv_key,
    };

    let y = host.recv_rsa_key(&guest.priv_key.to_public_key());
    let (za, zb) = guest.exchange(y);
    let result = host.compare(za, zb);
    assert_eq!(4, result.len());
    assert_eq!(result, vec![1, 2, 6, 7]);
}
