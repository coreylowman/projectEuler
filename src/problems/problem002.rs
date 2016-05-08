struct Pair {
	pub t : u64,
	pub x : u64,
	pub cur : u64,
	pub max : u64,
}

impl Iterator for Pair {
	type Item = u64;
	
	fn next(&mut self) -> Option<u64> {
		if self.cur >= self.max {
			return None
		}
		let t : u64 = self.t;
		let x : u64 = self.x;
		self.t = x;
		self.x = t + x;
		self.cur += 1;
		Some(x)
	}
}

fn go() -> String {
	let mut sum : u64 = 0;
	let fpair = Pair { t : 1, x : 1, cur : 1, max : 4000000};
	for x in fpair {
		if x > 4000000 {
			break;
		}
		if x % 2 == 0 {
			sum += x;
		}		
	}
	
	sum.to_string()
}

problem!(go, 4613732);
