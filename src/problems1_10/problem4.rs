pub fn is_palindrome(mut n : u64) -> bool {
	let mut digits :Vec<i8> = Vec::new();
	while n > 0 {
		digits.push((n % 10) as i8);
		n = (n / 10) as u64;
	}
	
	let length = digits.len();
	let mut i = 0;
	let mut j = length - 1;
	while i <= j {
		if digits[i] != digits[j] {
			return false
		}
		i += 1;
		j -=1;
	}
	true
}


pub fn go() -> u64 {
	let mut max = 0;
	for x in 100..999 {
		for y in x..999{
			let mul : u64 = x * y;
			if is_palindrome(mul) && mul > max {
				max = mul;
			}
		}	
	}
	
	max
}