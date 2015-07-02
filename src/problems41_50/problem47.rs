use prime_gen;

//num distinct prime factors
fn num_dpfs(mut n:u64,primes:&Vec<u64>) -> u64 {
	let mut num = 0;
	let mut i = 0;
   
	if prime_gen::contains(n,primes) { return 1 };
   
	while n > 1 {
		let p = primes[i];
		if n % p == 0 {
			num+=1;
			while n % p == 0 {
				n = n / p;
			}
		}
		i+=1;
	}

	num
}
 
pub fn go() -> u64{
	let primes = prime_gen::gen(1_000_000);
	let mut i = 1;
	loop {
		let a = num_dpfs(i,&primes) == 4;
		let b = num_dpfs(i+1,&primes) == 4;
		let c = num_dpfs(i+2,&primes) == 4;
		let d = num_dpfs(i+3,&primes) == 4;
		if a && b && c && d {
			return i
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