
fn is_ten_palindrome(mut num : u64) -> bool {
	let mut b10 : Vec<u64> = Vec::new();
	while num > 0 {
		b10.push(num%10);
		num /= 10;
	}
	let mut i = 0;
	let mut j = b10.len() - 1;
	while i <= j {
		if b10[i] != b10[j] {
			return false
		}
		i+=1;
		if j == 0 {
			break;
		}
		j-=1;
	}
	true
}

fn is_two_palindrome(mut num: u64) -> bool {
	let mut b2 : Vec<u64> = Vec::new();
	while num > 0 {
		b2.push(num%2);
		num /= 2;
	}
	let mut i = 0;
	let mut j = b2.len() - 1;
	while i <= j {
		if b2[i] != b2[j] {
			return false
		}
		i+=1;
		if j == 0 {
			break;
		}
		j-=1;
	}
	true
}

pub fn go() -> String {
	let mut sum = 0;
	
	for i in 1..1_000_000 {
		if is_two_palindrome(i) && is_ten_palindrome(i) {
			sum += i;
		}
	}
	
	sum.to_string()
}

problem!(go);
