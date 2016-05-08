extern crate num;

use self::num::bigint::{ToBigInt,BigInt};
use self::num::Zero;
use self::num::integer::Integer;

fn go() -> String {
	let mut n = 1.to_bigint().unwrap();
	for i in 2..101 {
		let bi = i.to_bigint().unwrap();
		n = n * bi;
	}
	let mut sum :BigInt = Zero::zero();
	let t = 10.to_bigint().unwrap();
	while n > Zero::zero() {
		let (q,r) = n.div_rem(&t);
		n = q;
		sum = sum + r;
	}
	sum.to_string()
}

problem!(go, 648);
