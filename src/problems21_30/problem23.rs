fn ds(n:u64) -> u64 {
	let mut sum = 0;
	let lim = (n/2 + 1) as u64;
	for i in 1..lim {
		if n % i == 0 {
			sum = sum + i;
		}
	}
	sum
}

fn is_abundant(n:u64) -> bool {
	ds(n) > n
}

pub fn go() -> u64 {
	let mut abunds : Vec<bool> = Vec::new();
	abunds.push(false); //0
	abunds.push(false); //1
	for i in 2..28123 {
		abunds.push(is_abundant(i));
	}
	let mut sum : u64 = 0;
	for x in 1..28123 {
		let mut canbe = false;
		for i in 1..x {
			if abunds[i] && abunds[x - i] {
				canbe = true;
				break;
			}
		}
		if !canbe {
			sum = sum + (x as u64);
		}
	}
	sum
}