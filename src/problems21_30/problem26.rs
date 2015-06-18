use prime_gen;
use num::bigint::{ToBigInt};
use num;
use num::integer::Integer;

//http://en.wikipedia.org/wiki/Repeating_decimal#Decimal_expansion_and_recurrence_sequence

fn prime_length(p : u64) -> u64{
	if p == 2 || p == 5 { return 0 }
	let mut k = 1;
	let one = 1.to_bigint().unwrap();
	let ten = 10.to_bigint().unwrap();
	let pp = p.to_bigint().unwrap();
	while num::pow(ten.clone(),k) % &pp != one {
		k = k + 1;
	}
	k as u64
}

//assume n doesn't have a prime factor bigger than 1000
//assume primes is a vector with the primes up to 1000
fn composite_length(mut n : u64,primes : &Vec<u64>) -> u64 {
	let mut prime_powers : [u64;1000] = [0;1000];
	for &p in primes.iter() {
		while n % p  == 0 {
			prime_powers[p as usize] += 1;
			n = n / p;
		}
	}
	prime_powers[2] = 0;
	prime_powers[5] = 0;
	//period
	let mut res = 1;
	for i in 3..1000 {	
		if prime_powers[i] == 1 {
			res = res.lcm(&prime_length(i as u64));
		}else if prime_powers[i] > 1 {
			let t : u64 = (i as u64).pow((prime_powers[i]-1) as u32) * prime_length(i as u64);
			res = res.lcm(&t);
		}
	}
	res
}

pub fn go() -> u64 {
	let primes = prime_gen::gen(1000);
	let mut m = 0;
	let mut mv = 0;
	//for i in 1..1000 {
	let mut i = 999;
	while i > 0 {
		if prime_gen::contains(i,&primes) {
			let l = prime_length(i);
			if l == i - 1 { return i }
			if l > mv {
				m = i;
				mv = l;
			}
		}else{
			let l = composite_length(i,&primes);
			if l > mv {
				m = i;
				mv = l;
			}
		}
		i -= 1;
	}
	m
}