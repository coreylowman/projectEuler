#[macro_use(problem)]
extern crate util;

use util::prime_gen;

pub fn go() -> String {
	let primes = prime_gen::gen(1_000_000);
	let pl = primes.len();      
	let all : u64 = primes.iter().fold(0,|acc,&item| acc + item);           
	let mut start = Vec::new();
	start.push(0);   
	let mut end = Vec::new();
	end.push(all);
	for i in 1..pl+1 {
		let slast = start[i-1];
		let elast = end[i-1];
		end.push(elast - primes[i-1]);
		start.push(slast + primes[i-1]);
	}
	
	let mut l : usize = pl;
	//make sure length is odd, see note below
	if l % 2 == 0 {
		l -= 1;
	}
	while l >= 1 {
		//for each possible length l
		//check if there are any consecutive sums that are prime
		//sum from s to e over primes == all - start[s] - end[e]
		//note: an even length won't be prime, because a sum of
		// an even number of odd numbers is even, so we skip all the
		// even lengths
		for s in 0..(pl-l+1) {
			//this will go way over 1_000_000 as all primes after primes[s] are bigger
			//and we won't likely get to the lengths where they don't go above 1_000_000
			if primes[s] >= 50_000 { break; } 
			let e = s + l;
			let q = all - start[s] - end[e];
			if q < 1000000 && q % 2 != 0 && prime_gen::is_prime(q) {
				return q.to_string()
			}
		}
		l -= 2;
	}
	0.to_string()
}

problem!(go);
