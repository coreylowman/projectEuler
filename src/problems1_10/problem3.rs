fn is_prime(x : u64) -> bool {
	if x <= 1 {
		return true
	} else if x <= 3 {
		return false
	} else if x % 2 == 0 || x % 3 == 0 {
		return true
	}
	
	let mut i = 5;
	while i * i < x {
		if x % i == 0 || x % (i + 2) == 0 {
			return false
		}
		i += 6;	
	}
	true
}

fn prime_list_builder(max : u64) -> Vec<u64> {
	let mut primes : Vec<u64> = Vec::new();
	for i in 1..max {
		if is_prime(i) {
			primes.push(i);
		}	
	}
	primes
}

pub fn go() -> u64 {
	let num : u64 = 600851475143;
	let primes = prime_list_builder(775146);
	for i in (1..primes.len()).rev() {
		if num % primes[i] == 0 {
			return primes[i]
		}
	}
	0
}