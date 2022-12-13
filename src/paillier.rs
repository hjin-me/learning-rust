use num::{BigUint};
use num::pow::Pow;
use tracing::trace;
use crate::lcm::lcm;
use crate::prime;

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

#[cfg(test)]
mod test {
    use tracing::Level;
    use super::*;

    #[test]
    fn test_euler_n() {
        let subscriber = tracing_subscriber::FmtSubscriber::builder()
            // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
            // will be written to stdout.
            .with_max_level(Level::TRACE)
            // completes the builder.
            .finish();

        tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
        let EulerParams(n, nn, en, ln, g) = euler_n().unwrap();
        println!("{:?}, {:?}, {:?}, {:?}, {:?}", n, nn, en, ln, g);
    }
}