use prime_gen;

pub fn go() -> u64 {
	let primes : Vec<u64> = prime_gen::gen(2000000);
	primes.iter().fold(0,|acc,&x| acc + x)
}