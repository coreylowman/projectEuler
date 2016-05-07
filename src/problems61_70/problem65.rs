use num::bigint::{ToBigInt};
use num::traits::ToPrimitive;
use std::mem;

//apparently this is using https://en.wikipedia.org/wiki/Farey_sequence
//i was trying to figure out why it worked when i came across that link
//i just noticed a pattern .___.
pub fn go() -> u64 {
	let mut lt = 1.to_bigint().unwrap();
	let mut lb = 0.to_bigint().unwrap();
	let mut top = 2.to_bigint().unwrap();
	let mut bottom = 1.to_bigint().unwrap();
	for i in 1..100{
		if i % 3 == 2 {
			let k = 2 * (i / 3) + 2;
			let k = k.to_bigint().unwrap();
			lt = &k * &top + &lt;
			lb = k * &bottom + &lb;		
		}else {
			lt = &top + &lt;
			lb = &bottom + &lb;
		}
		mem::swap(&mut lt,&mut top);
		mem::swap(&mut lb,&mut bottom);
	}
	let mut sum = 0;
	let ten = 10.to_bigint().unwrap();
	let zero = 0.to_bigint().unwrap();
	while top > zero {
		sum = sum + (&top % &ten).to_u64().unwrap();
		top = &top / &ten;
	}
	sum
}