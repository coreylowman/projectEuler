use num::bigint::{ToBigInt,BigInt};

fn num_digs(n:&BigInt) -> u64{
	n.to_string().len() as u64
}

pub fn go() -> u64 {
	let mut num = 0u64;
	let mut t = 1.to_bigint().unwrap();
	let mut b = 2.to_bigint().unwrap();
	for _ in 0..1000 {
		let a = &b + &t;
		if num_digs(&a) > num_digs(&b) {
			num += 1;
		}
		let q : BigInt =  &b + a;
		t = b;
		b = q;
	}
	num
}