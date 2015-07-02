use prime_gen;

pub fn go() -> u64 {
	let primes = prime_gen::gen(1000000);
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
	while l >= 1 {                      
		for s in 0..(pl-l+1) {
			if primes[s] >= 50_000 { break; } //this will go way over 1000000
			let e = s + l - 1;
			let q = all - start[s] - end[e];
			if q < 1000000 && q % 2 != 0 && prime_gen::contains(q,&primes) {
				return q as u64
			}
		}
		l -= 1;
	}
	0
}