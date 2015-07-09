use num::bigint::{ToBigInt,BigInt};
use num::{Zero};
use num::traits::ToPrimitive;

fn num_digs(mut n:BigInt) -> u64{
	let mut sum = 0u64;
	let t = 10.to_bigint().unwrap();
	while n > BigInt::zero() {
		sum = sum + 1;
		n = n / &t;
	}
	sum.to_u64().unwrap()
}

pub fn go() -> u64 {
	let mut num = 0u64;
	let mut t = 1.to_bigint().unwrap();
	let mut b = 2.to_bigint().unwrap();
	let two = 2.to_bigint().unwrap();
	for _ in 0..1000 {
		if num_digs(b.clone()+t.clone()) > num_digs(b.clone()) {
			num += 1;
		}
		let q : BigInt =  &b * &two + t;
		t = b;
		b = q;
	}
	num
}