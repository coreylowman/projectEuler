use prime_gen;

pub fn go() -> u64 {
	let num : u64 = 600851475143;
	let primes = prime_gen::gen(775146);
	for &p in primes.iter().rev() {
		if num % p == 0 {
			return p
		}
	}
	0
}