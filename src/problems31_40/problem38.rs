fn has_all(mut a:u64) -> bool{
	let mut digs = [0;10];
	
	while a > 0 {
		digs[(a%10) as usize] += 1;
		a /= 10;
	}
	
	if digs[0] > 0 {
		return false
	}
	
	for i in 1..digs.len() {
		if digs[i] != 1 {
			return false
		}
	}
	true
}

fn num_length(mut n:u64) -> u64 {
	let mut l = 0;
	while n > 0 {
		l+=1;
		n/=10;
	}
	l
}

pub fn go() -> u64 {
	let mut i = 987654;
	loop {
		
		i-=1;
	}
}