use util::prime_gen;

fn go() -> String {
	let primes = prime_gen::gen(1000000);
	primes[10000].to_string()
}

problem!(go, 104743);
