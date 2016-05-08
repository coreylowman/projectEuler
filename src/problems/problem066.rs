extern crate num;

use self::num::bigint::{BigInt, ToBigInt};
use self::num::traits::Signed;
use self::num::One;

fn modulo(a: &BigInt, b: &BigInt) -> BigInt {
    let zero = 0.to_bigint().unwrap();
    let r = a % b;
    if r < zero {
        r + b
    } else {
        r
    }
}

// computes the inverse b of a mod n st a * b mod n = 1
fn modulo_inverse(a: &BigInt, n: &BigInt) -> BigInt {
    let zero = 0.to_bigint().unwrap();
    let one = 1.to_bigint().unwrap();
    let mut b = 1.to_bigint().unwrap();
    while &b < n {
        if modulo(&(a * &b), n) == One::one() {
            return b;
        }
        b = b + &one;
    }
    zero
}

// https://en.wikipedia.org/wiki/Diophantine_equation
// https://en.wikipedia.org/wiki/Pell%27s_equation
// https://en.wikipedia.org/wiki/Chakravala_method
fn chakravala_method(d: BigInt) -> BigInt {
    let zero = 0.to_bigint().unwrap();
    let one = 1.to_bigint().unwrap();
    let eight = 8.to_bigint().unwrap();
    let sixty_four = 64.to_bigint().unwrap();

    // find any solution to a^2 - d * b^2 = k
    // let a = 8, b = 1
    let (mut a, mut b, mut k): (BigInt, BigInt, BigInt) = (eight,
                                                           1.to_bigint().unwrap(),
                                                           sixty_four - &d);
    let mut kabs = k.abs();
    while &k != &one {
        // want a positive integer m st k | a + bm and |m^2 - d| is minimal
        // m is of the form k * t + (-a % k)
        let mut c = modulo(&(&zero - &a), &kabs);
        let t = modulo_inverse(&b, &kabs);
        c = modulo(&(c * t), &k);

        let mut m = c;
        let mut last_diff = (&m * &m - &d).abs();
        let mut diff;

        loop {
            m = &m + &kabs;
            diff = (&m * &m - &d).abs();

            if diff > last_diff {
                // we've found the minimal solution... the diff has started to
                // increase
                break;
            }

            last_diff = diff;
        }

        // found minimal m
        m = &m - &kabs;

        // update values
        let ta = (&a * &m + &d * &b) / &kabs;
        let tb = (&a + &b * &m) / &kabs;
        let tk = (&m * &m - &d) / &k;
        a = ta;
        b = tb;
        k = tk;
        kabs = k.abs();
    }

    a
}

fn go() -> String {
    let mut max_x = 3.to_bigint().unwrap();
    let mut max_d = 2;
    let mut q;

    println!("{}", chakravala_method(61.to_bigint().unwrap()));
    println!("{}", chakravala_method(67.to_bigint().unwrap()));

    for d in 3..1000 {
        q = (d as f64).sqrt();
        if q == q.floor() {
            continue;
        }
        let x = chakravala_method(d.to_bigint().unwrap());
        if x > max_x {
            max_x = x;
            max_d = d;
        }
    }

    max_d.to_string()
}

problem!(go, 661);
