fn reverse(mut n:u64) -> u64{
	let mut digits : Vec<u64> = Vec::new();
	while n > 0 {
		digits.push(n%10);
		n/=10;
	}
	n = 0;
	for x in digits {
		n *= 10;
		n += x;
	}
	n
}

fn is_palindrome(mut n:u64) -> bool {
	let mut digits : Vec<u64> = Vec::new();
	while n > 0 {
		digits.push(n%10);
		n/=10;
	}
	let mut i = 0;
	let mut j = digits.len() - 1;
	if i == j { return true }
	while i <= j {
		if digits[i] != digits[j] {
			return false
		}
		i += 1;
		j -= 1;
	}
	true
}

fn has(mut n:u64) -> bool {	
	for _ in 0..50 {
		let x = reverse(n);
		n = x + n;
		if is_palindrome(n) {
			return true
		}
	}
	false
}

pub fn go() -> u64 {
	let mut num = 0;
	for i in 1..10_000 {
		if !has(i) {
			num += 1;
		}
	}
	num
}