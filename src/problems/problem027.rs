
use util::prime_gen;

pub fn go() -> String {
	let primes : Vec<i64> = prime_gen::igen(100000);
	let primes_to_thous : Vec<i64> = prime_gen::igen(1000);
	let mut mv = 0;
	let mut m : i64 = 0;	
	
	for a in -1000..1000 {	
		for b in primes_to_thous.iter() {
			let mut n = 0;
			if 1 + a + b <= 0 {
				continue;
			}
			while primes.binary_search(&(n*n + a * n + b)).is_ok() {
				n += 1;
			}
			if n > mv {
				mv = n;
				m = a * b;
			}
		}
	}	
	m.to_string()
}

problem!(go);
