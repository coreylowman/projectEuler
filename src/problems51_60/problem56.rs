use num::bigint::{ToBigInt,BigInt};
use num::{Zero,One};
use num::traits::ToPrimitive;
 
fn dig_sum(mut n:BigInt) -> u64{
	let mut sum = 0.to_bigint().unwrap();
	let t = 10.to_bigint().unwrap();
	while n > BigInt::zero() {
		sum = sum + (&n % &t);
		n = n / &t;
	}
	sum.to_u64().unwrap()
}
 
pub fn go() -> u64{
	let mut max = 0u64;
	for a in 95..100 {
		let mut q = BigInt::one();
		let p = a.to_bigint().unwrap();
		for _ in 1..100 {
			q = q * &p;
			let t : u64 = dig_sum(q.clone());
			if t > max {
				max = t;
			}                             
		}
	}
	max
}