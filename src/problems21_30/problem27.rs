use prime_gen;

pub fn go() -> i64 {
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
			while prime_gen::icontains(n*n + a * n + b,&primes) {
				n += 1;
			}
			if n > mv {
				mv = n;
				m = a * b;
			}
		}
	}	
	m
}