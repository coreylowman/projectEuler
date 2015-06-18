fn has_all(mut a:u64,mut b:u64) -> bool{
	let mut c = a * b;
	let mut digs = [0;10];
	
	while a > 0 {
		digs[(a%10) as usize] += 1;
		a /= 10;
	}
	
	while b > 0 {
		digs[(b%10) as usize] += 1;
		b /= 10;
	}
	
	while c > 0 {
		digs[(c%10) as usize] += 1;
		c /= 10;
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

pub fn go() -> u64 {
	let mut res : Vec<u64> = Vec::new();
	for x in 1..5_000 {
		if x % 10 == 0 {
			continue;
		}
		for y in 1..5_000 {			
			let xy = x * y;
			if y % 10 == 0 || xy % 10 == 0 {
				continue;
			}
			if has_all(x,y) && !res.contains(&xy) {
				res.push(xy);
			}
		}	
	}	
	
	let mut sum = 0;
	for &x in res.iter() {
		sum += x;
	}
	sum
}