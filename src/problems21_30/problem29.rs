use num::bigint::{BigUint,ToBigUint};
use num;

pub fn go() -> u64 {
	let mut results : Vec<BigUint> = Vec::new();
	for a in 2..101 {
		for b in 2..101 {
			results.push(num::pow(a.to_biguint().unwrap(),b));
		}
	}
	results.sort();
	let mut last = 0.to_biguint().unwrap();
	let mut num = 0;
	for x in results.iter(){
		if *x != last {
			num += 1;
			last = x.clone();
		}
	}
	num
}