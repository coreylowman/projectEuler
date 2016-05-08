extern crate num;

use self::num::bigint::{ToBigInt, BigInt};
use self::num::{Zero, One};

fn qmod(mut b: BigInt, mut e: BigInt, m: &BigInt) -> BigInt {
    let one = 1.to_bigint().unwrap();
    let two = 2.to_bigint().unwrap();
    let mut r: BigInt = one;
    b = b % m;
    while e > BigInt::zero() {
        if &e % &two == BigInt::one() {
            r = r * &b;
            r = r % m;
        }
        e = &e >> 1;
        b = (&b * &b) % m;
    }
    r
}

fn q(n: u64) -> BigInt {
    n.to_bigint().unwrap()
}

fn go() -> String {
    let t10: BigInt = q(10_000_000_000);
    let mut sum: BigInt = q(1);
    for i in 2..1001 {
        sum = sum + qmod(q(i), q(i), &t10);
        sum = sum % &t10;
    }
    sum.to_string()
}

problem!(go, 9110846700);
