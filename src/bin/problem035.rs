#[macro_use(problem)]
extern crate util;

use util::prime_gen;

fn is_circ(p:u64,primes:&Vec<u64>) -> bool {	
	let mut digs :Vec<u64> = Vec::new();
	let mut t = p.clone();
	while t > 0 {
		digs.push(t%10);
		t/=10;
	}

	if digs.len() == 1 {
		return true
	}
	
	if digs.contains(&5) || digs.contains(&0) || digs.contains(&2) {
		return false
	}
	
	digs.reverse();
	let l = digs.len();
	for s in 1..l {
		let mut q = 0;
		for i in 0..l {
			q += digs[(s+i)%l] * 10u64.pow((l-1-i) as u32);
		}
		if !prime_gen::contains(q,primes) {
			return false
		}
	}
	true
}

pub fn go() -> String {
	let primes = prime_gen::gen(1_000_000);
	let mut num = 0;
	for &p in primes.iter() {
		if is_circ(p,&primes) {
			num += 1;
		}
	}
	num.to_string()
}

problem!(go);
