pub fn is_palindrome(mut n : u64) -> bool {
	let mut digits :Vec<i8> = Vec::new();
	while n > 0 {
		digits.push((n % 10) as i8);
		n = (n / 10) as u64;
	}
	
	let length = digits.len();
	
	if length % 2 == 1 {
		return false
	}
	
	for i in 0..(length/2) {
		if digits[i] != digits[length - 1 - i] {
			return false;
		}
	}
	
	true
}


pub fn go() -> u64 {
	let mut max = 0;
	for x in 100..999 {
		for y in x..999{
			let mul : u64= x * y;
			if is_palindrome(mul) && mul > max {
				max = mul;
			}
		}	
	}
	
	max
}