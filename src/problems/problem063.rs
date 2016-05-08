extern crate num;

use self::num::bigint::{BigInt,ToBigInt};

fn num_length(n:BigInt)->usize {
	n.to_string().len()
}

pub fn go() -> String {
	let mut total = 0;
	let one = 1.to_bigint().unwrap();
	for e in 1..25 {
		let mut n = 1.to_bigint().unwrap();
		loop{
			let q = num::pow(n.clone(),e);
			let l = num_length(q);
			if l == e {
				total += 1;
			}else if l > e {
				break;
			}
			n = n + &one;
		}
	}
	total.to_string()
}

problem!(go, 49);
