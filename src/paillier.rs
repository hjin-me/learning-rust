use crate::lcm::lcm;
use crate::mod_inverse::mod_inverse;
use crate::prime;
use num_bigint::BigUint;
use num_traits::Pow;
use tracing::trace;

pub struct EulerParams(BigUint, BigUint, BigUint, BigUint, BigUint);

fn euler_n() -> Result<EulerParams, Box<dyn std::error::Error>> {
    let p = prime::gen();
    // let p = BigUint::from(3u32);
    trace!("p: {}", p);
    let q = prime::gen();
    // let q = BigUint::from(5u32);
    trace!("q: {}", q);
    let n = &p * &q;
    trace!("n: {}", n);
    let n_square = &n * &n;
    trace!("n_square: {}", n_square);
    let euler_n = (&p - BigUint::from(1u8)) * (&q - BigUint::from(1u8));
    trace!("euler_n: {}", euler_n);
    let lambda_n = lcm(&(&p - BigUint::from(1u8)), &(&q - BigUint::from(1u8)));
    trace!("lambda_n: {}", lambda_n);
    let alpha = BigUint::from(1u8);
    trace!("alpha: {}", alpha);
    let beta = BigUint::from(1u8);
    trace!("beta: {}", beta);

    let bn = Pow::pow(&beta, &n);
    trace!("b^n: {}", bn);
    let an1 = &alpha * &n + BigUint::from(1u8);
    trace!("a*n+1: {}", an1);
    let g = (&bn * &an1) % &n_square;
    trace!("g = b^n*(a*n+1)%n^2: {}", g);
    Ok(EulerParams(n, n_square, euler_n, lambda_n, g))
}

fn encrypt(m: &BigUint, n: &BigUint, nn: &BigUint, g: &BigUint) -> BigUint {
    let r = prime::gen();
    trace!("r: {}", r);
    let rn = r.modpow(&n, &nn);
    trace!("r^n % n^2: {}", rn);
    let gm = g.modpow(&m, &nn);
    trace!("g^m % n^2: {}", gm);

    let cmr = (gm * rn) % nn;
    trace!("cmr: {}", &cmr);
    cmr
}

fn decrypt(cmr: &BigUint, n: &BigUint, nn: &BigUint, lambda_n: &BigUint, g: &BigUint) -> BigUint {
    let d1 = cmr.modpow(&lambda_n, &nn);
    trace!("cmr ^ lambda_n % nn: {}", d1);
    let lc: BigUint = (d1 + 1u8) / n;
    trace!("(d1 + 1) / n: {}", lc);
    let lg: BigUint = (g.modpow(&lambda_n, &nn) + 1u8) / n;
    trace!("(g ^ lambda_n + 1) / n: {}", lg);

    let lg = mod_inverse(&lg, &n).unwrap();
    trace!("lg: {}", lg);

    let z = (lc * lg) % n;
    trace!("z: {}", z);
    z
}

#[cfg(test)]
mod test {
    use super::*;
    use tracing::Level;

    #[test]
    fn test_euler_n() {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
            // will be written to stdout.
            .with_max_level(Level::TRACE)
            // completes the builder.
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
        let EulerParams(n, nn, en, ln, g) = euler_n().unwrap();
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", n, nn, en, ln, g);
    }
    #[test]
    fn test_normal() {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
            // will be written to stdout.
            .with_max_level(Level::TRACE)
            // completes the builder.
            .finish();

        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");

        let EulerParams(n, nn, _, ln, g) = euler_n().unwrap();
        let cmr = encrypt(&BigUint::from(6u8), &n, &nn, &g);
        let r = decrypt(&cmr, &n, &nn, &ln, &g);
        assert_eq!(r, BigUint::from(6u8));

        // add
        let a = BigUint::from(10u8);
        let b = BigUint::from(17u8);
        let cmr_a = encrypt(&a, &n, &nn, &g);
        let cmr_b = encrypt(&b, &n, &nn, &g);
        let cmr_ab = cmr_a * cmr_b;

        let r = decrypt(&cmr_ab, &n, &nn, &ln, &g);
        assert_eq!(r, a + b);
    }
}
