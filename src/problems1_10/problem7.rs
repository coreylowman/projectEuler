use prime_gen;

pub fn go() -> u64 {
	let primes = prime_gen::gen(1000000);
	primes[10000]
}