#[macro_use(problem)]
extern crate util;

use util::prime_gen;

//num distinct prime factors
fn num_dpfs(n:u64,primes:&Vec<u64>) -> u64 {
	let mut num = 0;
	if prime_gen::contains(n,primes) { return 1 }
	
	let lim = (n as f64).sqrt() as u64 + 1;
	for i in 2..lim {
		if n % i == 0 {
			if prime_gen::contains(i,primes) {
				num += 1;
			}
			if prime_gen::contains(n / i,primes) {
				num += 1;
			}
		}		
	}
	num
}

pub fn go() -> String{
	let primes = prime_gen::gen(200_000);
	let mut i = 1;
	loop {
		let a = num_dpfs(i,&primes) == 4;
		let b = num_dpfs(i+1,&primes) == 4;
		let c = num_dpfs(i+2,&primes) == 4;
		let d = num_dpfs(i+3,&primes) == 4;
		if a && b && c && d {
			return i.to_string()
		}else if !b {
			i+=1;
		}else if !c {
			i +=2;
		}else if !d {
			i+=3;
		}
		i+=1;
	}
}

problem!(go);
