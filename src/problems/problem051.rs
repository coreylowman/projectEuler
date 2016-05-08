use util::prime_gen;

fn replace(mut n:u64,x:u64,replace : &Vec<usize>) -> u64{
	let mut digits : Vec<u64> = Vec::new();
	while n > 0 {
		digits.push(n%10);
		n/=10;
	}
	digits.reverse();
	for &r in replace {
		digits[r] = x;
	}
	n = 0;
	for x in digits {
		n *= 10;
		n += x;
	}
	n
}

fn has(n:u64)->Option<u64> {
	let l = length(n);
	for i in 0..l {
		for j in i+1..l {
			let mut num : Vec<u64> = Vec::new();
			let mut q : Vec<usize> = Vec::new();
			q.push(i);
			q.push(j);			
			for k in 0..10{
				let x = replace(n,k,&q);
				if x % 2 != 0 && length(x) == l && prime_gen::is_prime(x) {
					num.push(x);
				}
			}
			if num.len() == 8 { return Some(num[0]) }
			
			
			for k in j+1..l {
				let mut num : Vec<u64> = Vec::new();
				let mut q : Vec<usize> = Vec::new();
				q.push(i);
				q.push(j);
				q.push(k);
				for z in 0..10{				
					let x = replace(n,z,&q);
					if x % 2 != 0 && length(x) == l && prime_gen::is_prime(x) {
						num.push(x);
					}
				}
				if num.len() == 8 { return Some(num[0]) }
			}
		}
	}
	None
}

fn length(mut n:u64)->usize {
	let mut l = 0;
	while n > 0 {
		l += 1;
		n /= 10;
	}
	l
}

fn go() -> String {
	let primes = prime_gen::gen(500_000);
	for &p in primes.iter() {
		if p < 56000 { continue; }
		match has(p) {
			Some(x) => return x.to_string(),
			None => {},
		}
	}
	0.to_string()
}

problem!(go, 121313);
