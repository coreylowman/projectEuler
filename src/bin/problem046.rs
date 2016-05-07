#[macro_use(problem)]
extern crate util;

use util::prime_gen;

fn can_decompose(n:u64,primes:&Vec<u64>)->bool {
	let mut i = 1;
	while 2*i*i < n {
		if prime_gen::contains(n - 2 * i * i,primes) {
			return true
		}
		i+=1;
	}
	false
}


pub fn go() -> String{
	let primes = prime_gen::gen(1_000_000);
	let mut i = 9;
	while i < 1_000_000 {
		if !prime_gen::contains(i,&primes) && !can_decompose(i,&primes){
			return i.to_string()
		}
		i += 2;
	}
	0.to_string()
}

problem!(go);
