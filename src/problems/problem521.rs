use std::thread;

use util::prime_gen;

pub fn go() -> String {
	let t6 : u64 = 1_000_000;
	let t9 : u64 = 1_000_000_000;
	let t12 : u64 = 1_000_000_000_000;
	let primes_10 = prime_gen::gen(10 as i64);
	let mut sum : u64 = 1;
	let mut prod : u64 = 1;
	for &p in primes_10.iter() {		
		prod *= p;
		sum += prod;
	}
	let primes_100 = prime_gen::gen(100 as i64);
	for &p in primes_100.iter() {
		if p > 10 {			
			sum += p;
			println!("{} {}",p,sum);
		}
	}
	sum.to_string()
}

problem!(go);
