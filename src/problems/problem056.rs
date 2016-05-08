extern crate num;

use self::num::bigint::{ToBigInt, BigInt};
use self::num::One;

fn dig_sum(n: BigInt) -> u64 {
    n.to_string().into_bytes().iter().fold(0, |acc, &x| acc + (x - 48) as u64)
}

pub fn go() -> String {
    let mut max = 0u64;
    for a in 95..100 {
        let mut q = BigInt::one();
        let p = a.to_bigint().unwrap();
        for _ in 1..100 {
            q = q * &p;
            let t: u64 = dig_sum(q.clone());
            if t > max {
                max = t;
            }
        }
    }
    max.to_string()
}

problem!(go, 972);
