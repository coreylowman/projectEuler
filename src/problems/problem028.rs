
pub fn go() -> String {
	let mut dim = 3;
	let mut sum = 1;
	let mut num = 3;
	while dim != 1003 {
		for _ in 0..4 {
			sum += num;
			num += dim - 1;
		}
		num -= dim - 1;
		dim += 2;
		num += dim - 1;		
	}
	sum.to_string()
}

problem!(go);
