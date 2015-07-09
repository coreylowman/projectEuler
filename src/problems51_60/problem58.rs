use prime_gen;
 
pub fn go() -> u64 {
	let mut dim = 3;
	let mut n = 3;
	let mut num = 1;
	let mut num_primes = 0;
	loop {
		num += 4;
		for _ in 0..4 {                                      
			if prime_gen::is_prime(n) {
				num_primes += 1;
			}
			n += dim - 1;
		}
	   
		if 10 * num_primes < num{
			return dim
		}
	   
		n += 2;
		dim += 2;
	}
}