use num::bigint::{ToBigInt,BigInt};
use num::{Zero};
use num::traits::ToPrimitive;

fn num_digs(mut n:BigInt) -> u64{
	n.to_string().len() as u64
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