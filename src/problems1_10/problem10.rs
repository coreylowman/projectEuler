use prime_gen;

pub fn go() -> u64 {
	let primes : Vec<u64> = prime_gen::gen(2000000);
	let mut sum : u64 = 0;
	for x in primes.iter() {
		if *x > 2000000 {
			break;
		}
		sum += *x;
	}
	sum
}