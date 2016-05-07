
use util::prime_gen;

pub fn go() -> String {
	let primes : Vec<u64> = prime_gen::gen(2000000);
	primes.iter().fold(0,|acc,&x| acc + x).to_string()
}

problem!(go);
