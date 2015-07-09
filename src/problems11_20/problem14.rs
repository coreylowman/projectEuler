pub fn chain_length(mut n : u64) -> u64 {	
	let mut l = 0;
	while n > 1 {
		if n % 2 == 0 {
			n = n >> 1;			
		}else{
			n = 3 * n + 1;
		}
		l += 1;
	}
	l
}

pub fn go() -> u64 {
	let mut max = 0;
	let mut max_num = 0;
	for i in 0..1000001 {
		let l = chain_length(i);
		if l > max {
			max = l;
			max_num = i;
		}
	}
	max_num
}