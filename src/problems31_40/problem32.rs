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
	//1 or 2 digit number * 3 or 4 digit number = 4 digit number = 9 digits
	for x in 1..100 {
		//this will produce a 0
		if x % 10 == 0 {
			continue;
		}
		for y in 100..9999 {
			let xy = x * y;
			if xy > 9_999 { break; }			
			//we don't want any 0s!
			if y % 10 == 0 || xy % 10 == 0 {
				continue;
			}
			if has_all(x,y) && !res.contains(&xy) {
				res.push(xy);
			}
		}	
	}	
	
	res.iter().fold(0,|acc,&x| acc + x)
}