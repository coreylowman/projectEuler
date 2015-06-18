use num::traits::PrimInt;
use prime_gen;

fn is_trunct(mut num:u64,primes:&Vec<u64>) -> bool {	
	while num > 0 {
		num /= 10;
		if num > 0 && !prime_gen::contains(num,primes) {
			return false
		}
	}	
	true
}

fn is_trunct_rev(num : u64,primes:&Vec<u64>)->bool {
	let mut cnum = num;
	let mut l = 0;
	while cnum > 0 {
		l += 1;
		cnum /= 10;
	}
	let mut st = 10.pow(l);
	while st > 1 {	
		st /= 10;
		if st > 1 && !prime_gen::contains((num % st),primes) {
			return false
		}
	}
	true
}

pub fn go() -> u64 {
	let primes = prime_gen::gen(1_000_000);
	let mut sum = 0;
	for &p in primes.iter() {
		if p > 10 && is_trunct(p,&primes) && is_trunct_rev(p,&primes) {
			sum += p;
		}
	}
	sum
}